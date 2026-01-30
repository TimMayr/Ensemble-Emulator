//! GPU-accelerated bandlimited pixel filter using wgpu compute shaders.
//!
//! Implements themaister's pseudo-bandlimited pixel art filtering algorithm
//! for high-quality upscaling of NES frames.
//!
//! Reference: https://themaister.net/blog/2018/08/25/pseudo-bandlimited-pixel-art-filtering-in-3d-a-mathematical-derivation/

use crate::emulation::messages::RgbColor;
use std::sync::Arc;

/// WGSL compute shader implementing the bandlimited pixel filter.
///
/// This shader implements the "fast mode" from themaister's algorithm,
/// which uses a single 2x2 bilinear sample with sinusoidal UV shift.
const BANDLIMITED_SHADER: &str = r#"
// Bandlimited pixel filter compute shader
// Based on themaister's pseudo-bandlimited pixel art filtering

struct Params {
    src_width: u32,
    src_height: u32,
    dst_width: u32,
    dst_height: u32,
    // For PAR correction: effective source dimensions
    effective_src_width: f32,
    effective_src_height: f32,
}

@group(0) @binding(0) var<uniform> params: Params;
@group(0) @binding(1) var<storage, read> src: array<u32>;  // RGB packed as u32
@group(0) @binding(2) var<storage, read_write> dst: array<u32>;  // BGRA output

const PI: f32 = 3.14159265359;
const PI_HALF: f32 = 1.5707963268;
const TAYLOR_NORMALIZATION: f32 = 0.99549102425;  // 1/1.00452485553

// Taylor series sin approximation (for values in [-PI, PI])
fn taylor_sin(p: f32) -> f32 {
    let p2 = p * p;
    let p3 = p * p2;
    let p5 = p2 * p3;
    return clamp(TAYLOR_NORMALIZATION * (p - p3 / 6.0 + p5 / 120.0), -1.0, 1.0);
}

// Sample source texture with bilinear interpolation
fn sample_bilinear(x: f32, y: f32) -> vec3<f32> {
    let x0 = u32(floor(x));
    let y0 = u32(floor(y));
    let x1 = min(x0 + 1u, params.src_width - 1u);
    let y1 = min(y0 + 1u, params.src_height - 1u);
    
    let fx = x - floor(x);
    let fy = y - floor(y);
    
    // Clamp coordinates
    let cx0 = min(x0, params.src_width - 1u);
    let cy0 = min(y0, params.src_height - 1u);
    
    // Get four corner pixels
    let p00 = src[cy0 * params.src_width + cx0];
    let p10 = src[cy0 * params.src_width + x1];
    let p01 = src[y1 * params.src_width + cx0];
    let p11 = src[y1 * params.src_width + x1];
    
    // Unpack RGB
    let c00 = vec3<f32>(f32(p00 & 0xFFu), f32((p00 >> 8u) & 0xFFu), f32((p00 >> 16u) & 0xFFu));
    let c10 = vec3<f32>(f32(p10 & 0xFFu), f32((p10 >> 8u) & 0xFFu), f32((p10 >> 16u) & 0xFFu));
    let c01 = vec3<f32>(f32(p01 & 0xFFu), f32((p01 >> 8u) & 0xFFu), f32((p01 >> 16u) & 0xFFu));
    let c11 = vec3<f32>(f32(p11 & 0xFFu), f32((p11 >> 8u) & 0xFFu), f32((p11 >> 16u) & 0xFFu));
    
    // Bilinear interpolation
    let c0 = mix(c00, c10, fx);
    let c1 = mix(c01, c11, fx);
    return mix(c0, c1, fy);
}

// Sample source with nearest neighbor
fn sample_nearest(x: f32, y: f32) -> vec3<f32> {
    let ix = min(u32(x), params.src_width - 1u);
    let iy = min(u32(y), params.src_height - 1u);
    let p = src[iy * params.src_width + ix];
    return vec3<f32>(f32(p & 0xFFu), f32((p >> 8u) & 0xFFu), f32((p >> 16u) & 0xFFu));
}

@compute @workgroup_size(8, 8)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let dst_x = global_id.x;
    let dst_y = global_id.y;
    
    if (dst_x >= params.dst_width || dst_y >= params.dst_height) {
        return;
    }
    
    // Map destination pixel to source coordinates
    // Use effective dimensions for PAR-corrected aspect ratio
    let u = (f32(dst_x) + 0.5) / f32(params.dst_width);
    let v = (f32(dst_y) + 0.5) / f32(params.dst_height);
    
    let src_x = u * params.effective_src_width;
    let src_y = v * params.effective_src_height;
    
    // Compute extent (derivative in texel space)
    // For pure magnification, extent = src_size / dst_size
    let extent_x = params.effective_src_width / f32(params.dst_width);
    let extent_y = params.effective_src_height / f32(params.dst_height);
    let extent = vec2<f32>(max(extent_x, 1.0 / 256.0), max(extent_y, 1.0 / 256.0));
    
    // Get base pixel and phase
    let pixel = vec2<f32>(src_x, src_y) - 0.5;
    let base_pixel = floor(pixel);
    let phase = pixel - base_pixel;
    
    var color: vec3<f32>;
    
    // Fast mode: For magnification (extent <= 1.0), use bandlimited filter
    let max_extent = max(extent.x, extent.y);
    
    if (max_extent > 1.0) {
        // Minification: just use bilinear
        color = sample_bilinear(src_x, src_y);
    } else {
        // Magnification: apply bandlimited filter
        // Compute sinusoidal shift
        let phase_shifted = (phase - 0.5) / min(extent, vec2<f32>(0.5, 0.5));
        let clamped = clamp(phase_shifted, vec2<f32>(-1.0), vec2<f32>(1.0));
        let shift = 0.5 + 0.5 * vec2<f32>(taylor_sin(PI_HALF * clamped.x), taylor_sin(PI_HALF * clamped.y));
        
        // Compute lerp factor between bandlimited and regular sampling
        // max_extent = 1 -> l = 0, max_extent = 0.5 -> l = 1
        let l = clamp(2.0 - 2.0 * max_extent, 0.0, 1.0);
        
        // Sample at shifted UV
        let sample_pos = base_pixel + 0.5 + shift;
        let bandlimited = sample_bilinear(sample_pos.x, sample_pos.y);
        
        // For high magnification, use pure bandlimited
        // For lower magnification, blend with regular bilinear
        let regular = sample_bilinear(src_x, src_y);
        color = mix(regular, bandlimited, l);
    }
    
    // Pack as BGRA (for FFmpeg)
    let r = u32(clamp(color.x, 0.0, 255.0));
    let g = u32(clamp(color.y, 0.0, 255.0));
    let b = u32(clamp(color.z, 0.0, 255.0));
    let bgra = b | (g << 8u) | (r << 16u) | (255u << 24u);
    
    dst[dst_y * params.dst_width + dst_x] = bgra;
}
"#;

/// Parameters passed to the GPU shader
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct ShaderParams {
    src_width: u32,
    src_height: u32,
    dst_width: u32,
    dst_height: u32,
    effective_src_width: f32,
    effective_src_height: f32,
    _padding: [f32; 2],  // Align to 32 bytes
}

/// GPU-accelerated upscaler using wgpu compute shaders
pub struct GpuUpscaler {
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,
    pipeline: wgpu::ComputePipeline,

    // Cached buffers (recreated if size changes)
    dst_width: u32,
    dst_height: u32,
    
    src_buffer: wgpu::Buffer,
    dst_buffer: wgpu::Buffer,
    staging_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
}

impl GpuUpscaler {
    /// Try to create a GPU upscaler. Returns None if GPU is unavailable.
    pub fn try_new(
        src_width: u32,
        src_height: u32,
        dst_width: u32,
        dst_height: u32,
        effective_src_width: f32,
        effective_src_height: f32,
    ) -> Option<Self> {
        // Request GPU device (blocking)
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: None,
            force_fallback_adapter: false,
        }))?;
        
        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: Some("Upscaler Device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                ..Default::default()
            },
            None,
        )).ok()?;
        
        let device = Arc::new(device);
        let queue = Arc::new(queue);
        
        // Create shader module
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Bandlimited Upscale Shader"),
            source: wgpu::ShaderSource::Wgsl(BANDLIMITED_SHADER.into()),
        });
        
        // Create bind group layout
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Upscale Bind Group Layout"),
            entries: &[
                // Params uniform
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                // Source buffer
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                // Destination buffer
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });
        
        // Create pipeline
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Upscale Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });
        
        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Upscale Compute Pipeline"),
            layout: Some(&pipeline_layout),
            module: &shader,
            entry_point: Some("main"),
            compilation_options: Default::default(),
            cache: None,
        });
        
        // Create buffers
        let src_size = (src_width * src_height * 4) as u64;  // 4 bytes per pixel (packed RGB)
        let dst_size = (dst_width * dst_height * 4) as u64;  // 4 bytes per pixel (BGRA)
        
        let params_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Params Buffer"),
            size: size_of::<ShaderParams>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        
        let src_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Source Buffer"),
            size: src_size,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        
        let dst_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Destination Buffer"),
            size: dst_size,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
        
        let staging_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Staging Buffer"),
            size: dst_size,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        
        // Create bind group
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Upscale Bind Group"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: params_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: src_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: dst_buffer.as_entire_binding(),
                },
            ],
        });
        
        // Write initial params
        let params = ShaderParams {
            src_width,
            src_height,
            dst_width,
            dst_height,
            effective_src_width,
            effective_src_height,
            _padding: [0.0; 2],
        };
        queue.write_buffer(&params_buffer, 0, bytemuck::bytes_of(&params));
        
        Some(Self {
            device,
            queue,
            pipeline,
            dst_width,
            dst_height,
            src_buffer,
            dst_buffer,
            staging_buffer,
            bind_group,
        })
    }
    
    /// Upscale a frame using the GPU.
    /// 
    /// Input: RGB pixels (3 bytes per pixel)
    /// Output: BGRA pixels (4 bytes per pixel, ready for FFmpeg)
    pub fn upscale(&self, src: &[RgbColor]) -> Vec<u8> {
        // Pack RGB to u32 for upload
        let src_packed: Vec<u32> = src.iter()
            .map(|(r, g, b)| (*r as u32) | ((*g as u32) << 8) | ((*b as u32) << 16))
            .collect();
        
        // Upload source data
        self.queue.write_buffer(&self.src_buffer, 0, bytemuck::cast_slice(&src_packed));
        
        // Create command encoder
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Upscale Encoder"),
        });
        
        // Dispatch compute shader
        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("Upscale Pass"),
                timestamp_writes: None,
            });
            compute_pass.set_pipeline(&self.pipeline);
            compute_pass.set_bind_group(0, &self.bind_group, &[]);
            
            // Dispatch enough workgroups to cover all pixels
            let workgroup_x = self.dst_width.div_ceil(8);
            let workgroup_y = self.dst_height.div_ceil(8);
            compute_pass.dispatch_workgroups(workgroup_x, workgroup_y, 1);
        }
        
        // Copy result to staging buffer
        let dst_size = (self.dst_width * self.dst_height * 4) as u64;
        encoder.copy_buffer_to_buffer(&self.dst_buffer, 0, &self.staging_buffer, 0, dst_size);
        
        // Submit commands
        self.queue.submit(std::iter::once(encoder.finish()));
        
        // Read back result
        let buffer_slice = self.staging_buffer.slice(..);
        let (tx, rx) = std::sync::mpsc::channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
            tx.send(result).unwrap();
        });
        
        self.device.poll(wgpu::Maintain::Wait);
        rx.recv().unwrap().unwrap();
        
        let data = buffer_slice.get_mapped_range();
        let result = data.to_vec();
        drop(data);
        self.staging_buffer.unmap();
        
        result
    }
    
    /// Get destination dimensions
    pub fn dst_dimensions(&self) -> (u32, u32) {
        (self.dst_width, self.dst_height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_gpu_upscaler_creation() {
        // This test may fail on CI without GPU, which is expected
        let upscaler = GpuUpscaler::try_new(256, 240, 1920, 1080, 293.0, 240.0);
        // Just check that it doesn't panic - actual GPU may not be available
        println!("GPU upscaler available: {}", upscaler.is_some());
    }
}
