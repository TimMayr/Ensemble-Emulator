# Performance Analysis and Optimization Guide

## Executive Summary

Performance analysis of the NES emulator identified several optimization opportunities. After implementing targeted optimizations, performance improved from **~6.5x realtime to ~6.9x realtime**, a **~6% improvement**.

## Current Performance

| Metric | Before | After |
|--------|--------|-------|
| Speed | 6.5x realtime | 6.9x realtime |
| Time per 10M cycles | ~72ms | ~67ms |

## Implemented Optimizations

### 1. Fast Opcode Lookup Table (opcode.rs)

**Problem**: Original code used `HashMap<u8, &OpCode>` for opcode lookup during instruction fetch.

**Solution**: Added a 256-element array `OPCODES_TABLE` for O(1) direct indexing.

```rust
// Before: HashMap lookup with hash computation
self.current_opcode = Some(**OPCODES_MAP.get().unwrap().get(&opcode).unwrap());

// After: Direct array indexing
self.current_opcode = get_opcode(opcode);
```

**Impact**: Marginal (compiler was already optimizing HashMap), but cleaner code path.

### 2. VecDeque Pre-allocation (cpu.rs)

**Problem**: `get_instructions_for_op_type()` created a new `VecDeque::new()` for every instruction, causing heap allocations in the hot path.

**Solution**: Pre-allocate with `VecDeque::with_capacity(8)` since max instruction count is 7.

```rust
// Before
let mut instructions = VecDeque::new();

// After  
let mut instructions = VecDeque::with_capacity(8);
```

**Impact**: Reduces allocation overhead during instruction decode.

### 3. PPU Cycle Calculation Optimization (ppu.rs)

**Problem**: PPU step calculated scanline and dot using expensive u128 modulo operations:
```rust
let frame_dot = self.dot_counter % DOTS_PER_FRAME;  // u128 modulo
self.scanline = (frame_dot / (DOTS_PER_SCANLINE + 1) as u128) as u16;  // u128 division
self.dot = (frame_dot % (DOTS_PER_SCANLINE + 1) as u128) as u16;  // u128 modulo
```

**Solution**: Increment `dot` and `scanline` directly instead of recalculating from `dot_counter`:
```rust
self.dot += 1;
if self.dot > DOTS_PER_SCANLINE {
    self.dot = 0;
    self.scanline += 1;
    if self.scanline > PRE_RENDER_SCANLINE {
        self.scanline = 0;
    }
}
```

**Impact**: **Main performance gain** - eliminates 3 expensive u128 operations per PPU cycle.

### 4. NES Step Internal Cleanup (nes.rs)

**Problem**: Unnecessary arithmetic in hot path.

**Solution**: Simplified condition check:
```rust
// Before
if self.cpu_cycle_counter.wrapping_add(2) == 12

// After
if self.cpu_cycle_counter == 10
```

**Impact**: Cleaner code, marginal performance improvement.

---

## Architectural Recommendations for Future Improvements

### 1. **Replace VecDeque with Fixed-Size Ring Buffer**

**Current Architecture**: The CPU uses `VecDeque<MicroOp>` for the micro-operation queue. Even with pre-allocation, VecDeque has pointer indirection overhead.

**Recommended Solution**: Use a fixed-size array with head/tail indices:

```rust
struct MicroOpQueue {
    ops: [MicroOp; 8],  // Max 7 ops needed
    head: u8,
    tail: u8,
    len: u8,
}

impl MicroOpQueue {
    #[inline(always)]
    fn push(&mut self, op: MicroOp) {
        self.ops[self.tail as usize] = op;
        self.tail = (self.tail + 1) & 7;  // Mask for wrap-around
        self.len += 1;
    }
    
    #[inline(always)]
    fn pop(&mut self) -> Option<MicroOp> {
        if self.len == 0 { return None; }
        let op = self.ops[self.head as usize];
        self.head = (self.head + 1) & 7;
        self.len -= 1;
        Some(op)
    }
}
```

**Benefits**:
- No heap allocation
- Better cache locality
- Predictable memory access patterns

**Drawbacks**:
- Fixed maximum size (acceptable since max is known)
- More code to maintain

**Estimated Impact**: 2-5% improvement

### 2. **Replace Rc<RefCell<Ppu>> with Direct Ownership**

**Current Architecture**: PPU is shared via `Rc<RefCell<Ppu>>` between CPU and NES structs. Every PPU access requires:
1. `Rc` reference count check
2. `RefCell` borrow checking at runtime

**Recommended Solution**: Restructure to have NES own PPU directly and pass references:

```rust
struct Nes {
    cpu: Cpu,
    ppu: Ppu,  // Direct ownership
    // ...
}

impl Nes {
    fn step(&mut self) {
        // Can borrow both mutably without runtime checks
        self.ppu.step();
        self.cpu.step(&mut self.ppu);
    }
}
```

**Benefits**:
- Eliminates RefCell borrow checking overhead per cycle
- Better compiler optimizations possible
- Removes Rc reference counting

**Drawbacks**:
- Requires significant refactoring
- Changes CPU interface to take PPU reference
- May complicate save state handling

**Estimated Impact**: 5-10% improvement

### 3. **Instruction-Level Parallelism via Batched Execution**

**Current Architecture**: Emulates one master clock cycle at a time, checking cycle counters every step.

**Recommended Solution**: Execute multiple CPU cycles in a batch when safe:

```rust
fn run_batch(&mut self) {
    // Calculate how many cycles until next interrupt/special event
    let safe_cycles = self.calculate_safe_batch_size();
    
    for _ in 0..safe_cycles {
        // Fast path without interrupt checks
        self.cpu.step_unchecked();
    }
    
    // Handle any pending interrupts
    self.handle_pending_events();
}
```

**Benefits**:
- Reduces branch mispredictions
- Better CPU pipeline utilization
- Can execute longer without interrupt checks

**Drawbacks**:
- Complex to implement correctly
- Must carefully track interrupt timing
- May reduce accuracy for timing-sensitive games

**Estimated Impact**: 10-20% improvement (significant)

### 4. **Memory Access Optimization via Direct Mapping**

**Current Architecture**: Memory access goes through `MemoryMap` which has a 64KB lookup table mapping addresses to memory regions.

**Recommended Solution**: For known memory regions (RAM, ROM), use direct pointers:

```rust
struct FastMemory {
    // Direct access for common regions
    ram: [u8; 2048],
    prg_rom: Box<[u8]>,
    
    // Fallback for special regions (PPU regs, APU, etc.)
    memory_map: MemoryMap,
}

impl FastMemory {
    #[inline(always)]
    fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x07FF => self.ram[(addr & 0x7FF) as usize],
            0x0800..=0x1FFF => self.ram[(addr & 0x7FF) as usize],  // RAM mirror
            0x8000..=0xFFFF => self.prg_rom[(addr - 0x8000) as usize],
            _ => self.memory_map.mem_read(addr),  // Fallback
        }
    }
}
```

**Benefits**:
- Eliminates lookup table indirection for common cases
- Branch predictor can optimize common paths
- Better cache utilization

**Drawbacks**:
- Duplicates mapping logic
- Must maintain two code paths
- ROM banking (mappers) complicates this

**Estimated Impact**: 5-15% improvement

### 5. **JIT Compilation (Long-term)**

**Current Architecture**: Interprets each 6502 instruction cycle-by-cycle.

**Recommended Solution**: Just-In-Time compile frequently-executed code blocks:

```rust
struct JitBlock {
    native_code: *const u8,
    entry_pc: u16,
    exit_pc: u16,
    cycles: u32,
}

impl Cpu {
    fn execute_block(&mut self, block: &JitBlock) {
        // Execute native code directly
        unsafe { (block.native_code as fn(&mut Cpu))(&mut self); }
    }
}
```

**Benefits**:
- 10-100x speedup possible for hot code
- Can optimize across instruction boundaries
- Modern technique used by production emulators

**Drawbacks**:
- Very complex to implement
- Platform-specific native code generation
- Cycle accuracy harder to maintain
- Security considerations

**Estimated Impact**: 10-100x improvement (transformative)

---

## Priority Recommendations

1. **High Priority, Low Effort**: VecDeque → Fixed Ring Buffer
2. **High Priority, Medium Effort**: Remove Rc<RefCell<Ppu>>
3. **Medium Priority, Medium Effort**: Memory Direct Mapping
4. **High Priority, High Effort**: Batched Execution
5. **Transformative, Very High Effort**: JIT Compilation

## Conclusion

The implemented optimizations provide a solid ~6% performance improvement. The most impactful future optimization would be restructuring to remove `Rc<RefCell>` overhead and implementing batched execution. For a production emulator, JIT compilation would be the ultimate solution but requires significant engineering investment.

Current performance of 6.9x realtime is sufficient for comfortable gameplay on modern hardware, but additional optimizations would be valuable for:
- Lower-powered devices (mobile, embedded)
- Running multiple instances
- Fast-forward features
- Reducing power consumption
