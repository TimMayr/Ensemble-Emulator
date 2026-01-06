use std::time::Instant;
use crate::emulation::nes::Nes;

/// NES master clock frequency in Hz (NTSC)
const NES_MASTER_CLOCK_HZ: f64 = 21_477_272.0;

/// Performance benchmark test.
/// Run with: `cargo test performance_baseline -- --ignored --nocapture`
#[test]
#[ignore]
fn performance_baseline() {
    let mut emu = Nes::default();
    emu.load_rom(&String::from("./tests/nes-test-roms/nestest_headless.nes"));
    emu.power();
    
    // Warm up - actually execute cycles
    for _ in 0..1_000_000 {
        let _ = emu.step();
    }
    
    // Run 10 million master cycles
    let cycles = 10_000_000u64;
    let mut times = Vec::new();
    
    for run in 0..3 {
        let start = Instant::now();
        for _ in 0..cycles {
            let _ = emu.step();
        }
        let duration = start.elapsed();
        times.push(duration);
        
        // NES runs at ~21.47 MHz master clock
        // So 10M cycles = ~0.47 seconds of NES time
        let nes_time_sec = cycles as f64 / NES_MASTER_CLOCK_HZ;
        let speedup = nes_time_sec / duration.as_secs_f64();
        
        println!("Run {}: {} master cycles in {:?}", run, cycles, duration);
        println!("  NES time: {:.3}s, Real time: {:.3}s, Speed: {:.1}x realtime", 
            nes_time_sec, duration.as_secs_f64(), speedup);
    }
    
    let avg_secs = times.iter().map(|d| d.as_secs_f64()).sum::<f64>() / times.len() as f64;
    let nes_time = cycles as f64 / NES_MASTER_CLOCK_HZ;
    
    println!("\n=== Performance Summary ===");
    println!("Average speed: {:.1}x realtime", nes_time / avg_secs);
}
