use std::error::Error;
use std::mem::{self, size_of};

use imgui::internal::RawWrapper;
use imgui::{DrawCmd, DrawCmdParams, DrawIdx, DrawVert, TextureId, Textures, Ui};
use imgui_sdl3::platform::Platform;
use imgui_sdl3::utils::create_texture;
use sdl3::event::Event;
use sdl3::gpu::*;
use sdl3::rect::Rect;
use sdl3::{EventPump, Sdl};

pub struct ImguiBackend {
    context: imgui::Context,
    platform: Platform,
    renderer: Renderer,
}

impl ImguiBackend {
    pub fn new<F>(device: &Device, window: &Window, configure: F) -> Self
    where
        F: FnOnce(&mut imgui::Context),
    {
        let mut context = imgui::Context::create();
        configure(&mut context);

        let platform = Platform::new(&mut context);
        let renderer =
            Renderer::new(device, window, &mut context).expect("failed to create ImGui renderer");

        Self {
            context,
            platform,
            renderer,
        }
    }

    pub fn handle_event(&mut self, event: &Event) {
        self.platform.handle_event(&mut self.context, event);
    }

    pub fn render<F>(
        &mut self,
        sdl_context: &mut Sdl,
        device: &Device,
        window: &Window,
        event_pump: &EventPump,
        command_buffer: &mut CommandBuffer,
        color_targets: &[ColorTargetInfo],
        mut draw_callback: F,
    ) where
        F: FnMut(&mut Ui),
    {
        self.platform
            .prepare_frame(sdl_context, &mut self.context, window, event_pump);

        let ui = self.context.new_frame();
        draw_callback(ui);

        self.renderer
            .render(device, command_buffer, color_targets, &mut self.context)
            .expect("failed to render ImGui frame");
    }

    pub fn register_texture(&mut self, texture: Texture<'static>, sampler: Sampler) -> TextureId {
        self.renderer.register_texture(texture, sampler)
    }

    pub fn replace_texture(&mut self, id: TextureId, texture: Texture<'static>, sampler: Sampler) {
        self.renderer.replace_texture(id, texture, sampler);
    }
}

#[derive(Clone)]
struct TextureEntry {
    texture: Texture<'static>,
    sampler: Sampler,
}

struct Renderer {
    pipeline: GraphicsPipeline,
    textures: Textures<TextureEntry>,
    font_texture_id: TextureId,
    vertex_data: Vec<DrawVert>,
    index_data: Vec<DrawIdx>,
}

impl Renderer {
    fn new(
        device: &Device,
        window: &Window,
        imgui_context: &mut imgui::Context,
    ) -> Result<Self, Box<dyn Error>> {
        let vert = device
            .create_shader()
            .with_code(
                ShaderFormat::SPIRV,
                include_bytes!(concat!(env!("OUT_DIR"), "/imgui.vert.spv")),
                ShaderStage::Vertex,
            )
            .with_uniform_buffers(1)
            .with_entrypoint(c"main")
            .build()?;

        let frag = device
            .create_shader()
            .with_code(
                ShaderFormat::SPIRV,
                include_bytes!(concat!(env!("OUT_DIR"), "/imgui.frag.spv")),
                ShaderStage::Fragment,
            )
            .with_samplers(1)
            .with_entrypoint(c"main")
            .build()?;

        let format = device.get_swapchain_texture_format(window);

        let pipeline = device
            .create_graphics_pipeline()
            .with_vertex_shader(&vert)
            .with_vertex_input_state(
                VertexInputState::new()
                    .with_vertex_buffer_descriptions(&[VertexBufferDescription::new()
                        .with_slot(0)
                        .with_pitch(size_of::<DrawVert>() as u32)
                        .with_input_rate(VertexInputRate::Vertex)
                        .with_instance_step_rate(0)])
                    .with_vertex_attributes(&[
                        VertexAttribute::new()
                            .with_format(VertexElementFormat::Float2)
                            .with_location(0)
                            .with_buffer_slot(0)
                            .with_offset(mem::offset_of!(DrawVert, pos) as u32),
                        VertexAttribute::new()
                            .with_format(VertexElementFormat::Float2)
                            .with_location(1)
                            .with_buffer_slot(0)
                            .with_offset(mem::offset_of!(DrawVert, uv) as u32),
                        VertexAttribute::new()
                            .with_format(VertexElementFormat::Ubyte4Norm)
                            .with_location(2)
                            .with_buffer_slot(0)
                            .with_offset(mem::offset_of!(DrawVert, col) as u32),
                    ]),
            )
            .with_rasterizer_state(
                RasterizerState::new()
                    .with_fill_mode(FillMode::Fill)
                    .with_front_face(FrontFace::Clockwise),
            )
            .with_fragment_shader(&frag)
            .with_primitive_type(PrimitiveType::TriangleList)
            .with_target_info(
                GraphicsPipelineTargetInfo::new().with_color_target_descriptions(&[
                    ColorTargetDescription::new()
                        .with_format(format)
                        .with_blend_state(
                            ColorTargetBlendState::new()
                                .with_color_blend_op(BlendOp::Add)
                                .with_src_color_blendfactor(BlendFactor::SrcAlpha)
                                .with_dst_color_blendfactor(BlendFactor::OneMinusSrcAlpha)
                                .with_alpha_blend_op(BlendOp::Add)
                                .with_src_alpha_blendfactor(BlendFactor::One)
                                .with_dst_alpha_blendfactor(BlendFactor::OneMinusSrcAlpha)
                                .with_enable_blend(true),
                        ),
                ]),
            )
            .build()?;

        let font_texture = create_imgui_font_texture(device, imgui_context)?;

        let font_sampler = device
            .create_sampler(
                SamplerCreateInfo::new()
                    .with_min_filter(Filter::Linear)
                    .with_mag_filter(Filter::Linear)
                    .with_mipmap_mode(SamplerMipmapMode::Linear)
                    .with_address_mode_u(SamplerAddressMode::ClampToEdge)
                    .with_address_mode_v(SamplerAddressMode::ClampToEdge)
                    .with_address_mode_w(SamplerAddressMode::ClampToEdge),
            )
            .unwrap();

        let mut textures = Textures::new();
        let font_texture_id = textures.insert(TextureEntry {
            texture: font_texture,
            sampler: font_sampler,
        });

        imgui_context.fonts().tex_id = font_texture_id;

        Ok(Self {
            pipeline,
            textures,
            font_texture_id,
            vertex_data: Vec::new(),
            index_data: Vec::new(),
        })
    }

    fn register_texture(&mut self, texture: Texture<'static>, sampler: Sampler) -> TextureId {
        self.textures.insert(TextureEntry {
            texture,
            sampler,
        })
    }

    fn replace_texture(&mut self, id: TextureId, texture: Texture<'static>, sampler: Sampler) {
        self.textures.replace(
            id,
            TextureEntry {
                texture,
                sampler,
            },
        );
    }

    fn texture_entry(&self, id: TextureId) -> &TextureEntry {
        self.textures
            .get(id)
            .or_else(|| self.textures.get(self.font_texture_id))
            .expect("font texture missing")
    }

    fn render(
        &mut self,
        device: &Device,
        command_buffer: &mut CommandBuffer,
        color_targets: &[ColorTargetInfo],
        imgui_context: &mut imgui::Context,
    ) -> Result<(), Box<dyn Error>> {
        let io = imgui_context.io();
        let [width, height] = io.display_size;
        let [scale_w, scale_h] = io.display_framebuffer_scale;

        let fb_width = width * scale_w;
        let fb_height = height * scale_h;

        let draw_data = imgui_context.render();

        if width == 0.0
            || height == 0.0
            || draw_data.total_vtx_count == 0
            || draw_data.total_idx_count == 0
        {
            return Ok(());
        }

        self.vertex_data.clear();
        self.vertex_data.reserve(draw_data.total_vtx_count as usize);
        self.index_data.clear();
        self.index_data.reserve(draw_data.total_idx_count as usize);

        for draw_list in draw_data.draw_lists() {
            self.vertex_data.extend_from_slice(draw_list.vtx_buffer());
            self.index_data.extend_from_slice(draw_list.idx_buffer());
        }

        let mut render_pass = device.begin_render_pass(command_buffer, color_targets, None)?;
        render_pass.bind_graphics_pipeline(&self.pipeline);

        let mut bound_texture: Option<TextureId> = None;

        let vtx_bytes = self.vertex_data.len() * size_of::<DrawVert>();
        let idx_bytes = self.index_data.len() * size_of::<DrawIdx>();
        let upload_size = vtx_bytes.max(idx_bytes) as u32;

        let copy_commands = device.acquire_command_buffer()?;
        let transfer_buffer = device
            .create_transfer_buffer()
            .with_size(upload_size)
            .with_usage(TransferBufferUsage::UPLOAD)
            .build()?;

        let copy_pass = device.begin_copy_pass(&copy_commands)?;

        let vertex_buffer = create_buffer_with_data(
            device,
            &transfer_buffer,
            &copy_pass,
            BufferUsageFlags::VERTEX,
            &self.vertex_data,
        )?;

        let index_buffer = create_buffer_with_data(
            device,
            &transfer_buffer,
            &copy_pass,
            BufferUsageFlags::INDEX,
            &self.index_data,
        )?;

        device.end_copy_pass(copy_pass);
        copy_commands.submit()?;

        render_pass.bind_vertex_buffers(
            0,
            &[BufferBinding::new()
                .with_buffer(&vertex_buffer)
                .with_offset(0)],
        );

        render_pass.bind_index_buffer(
            &BufferBinding::new()
                .with_buffer(&index_buffer)
                .with_offset(0),
            if size_of::<DrawIdx>() == 2 {
                IndexElementSize::_16BIT
            } else {
                IndexElementSize::_32BIT
            },
        );

        device.set_viewport(
            &render_pass,
            Viewport::new(0.0, 0.0, fb_width, fb_height, 0.0, 1.0),
        );

        let matrix = [
            [2.0 / width, 0.0, 0.0, 0.0],
            [0.0, 2.0 / -height, 0.0, 0.0],
            [0.0, 0.0, -1.0, 0.0],
            [-1.0, 1.0, 0.0, 1.0],
        ];
        command_buffer.push_vertex_uniform_data(0, &matrix);

        let mut voffset = 0;
        let mut ioffset = 0;

        for draw_list in draw_data.draw_lists() {
            for draw_cmd in draw_list.commands() {
                match draw_cmd {
                    DrawCmd::Elements {
                        count,
                        cmd_params,
                    } => {
                        let DrawCmdParams {
                            clip_rect: [x, y, w, h],
                            texture_id,
                            idx_offset,
                            vtx_offset,
                        } = cmd_params;

                        let scissor_x = (x * scale_w) as i32;
                        let scissor_y = (y * scale_h) as i32;
                        let scissor_w = ((w - x) * scale_w).max(0.0) as u32;
                        let scissor_h = ((h - y) * scale_h).max(0.0) as u32;

                        if scissor_w == 0 || scissor_h == 0 {
                            continue;
                        }

                        render_pass
                            .set_scissor(Rect::new(scissor_x, scissor_y, scissor_w, scissor_h));

                        if bound_texture != Some(texture_id) {
                            let entry = self.texture_entry(texture_id);
                            render_pass.bind_fragment_samplers(
                                0,
                                &[TextureSamplerBinding::new()
                                    .with_texture(&entry.texture)
                                    .with_sampler(&entry.sampler)],
                            );
                            bound_texture = Some(texture_id);
                        }

                        render_pass.draw_indexed_primitives(
                            count as u32,
                            1,
                            (idx_offset + ioffset) as u32,
                            (vtx_offset + voffset) as i32,
                            0,
                        );
                    }
                    DrawCmd::ResetRenderState => {
                        render_pass.bind_graphics_pipeline(&self.pipeline);
                        bound_texture = None;
                    }
                    DrawCmd::RawCallback {
                        callback,
                        raw_cmd,
                    } => unsafe {
                        callback(draw_list.raw(), raw_cmd);
                    },
                }
            }

            ioffset += draw_list.idx_buffer().len();
            voffset += draw_list.vtx_buffer().len();
        }

        device.end_render_pass(render_pass);

        Ok(())
    }
}

fn create_imgui_font_texture(
    device: &Device,
    imgui_context: &mut imgui::Context,
) -> Result<Texture<'static>, Box<dyn Error>> {
    let font_atlas = imgui_context.fonts().build_rgba32_texture();

    let copy_commands = device.acquire_command_buffer()?;
    let copy_pass = device.begin_copy_pass(&copy_commands)?;

    let font_texture = create_texture(
        device,
        &copy_pass,
        font_atlas.data,
        font_atlas.width,
        font_atlas.height,
    )?;

    device.end_copy_pass(copy_pass);
    copy_commands.submit()?;

    Ok(font_texture)
}

fn create_buffer_with_data<T: Copy>(
    device: &Device,
    transfer_buffer: &TransferBuffer,
    copy_pass: &CopyPass,
    usage: BufferUsageFlags,
    data: &[T],
) -> Result<Buffer, sdl3::Error> {
    let len_bytes = std::mem::size_of_val(data);

    let buffer = device
        .create_buffer()
        .with_size(len_bytes as u32)
        .with_usage(usage)
        .build()?;

    let mut map = transfer_buffer.map::<T>(device, true);
    let mem = map.mem_mut();
    for (index, &value) in data.iter().enumerate() {
        mem[index] = value;
    }
    map.unmap();

    copy_pass.upload_to_gpu_buffer(
        TransferBufferLocation::new()
            .with_offset(0)
            .with_transfer_buffer(transfer_buffer),
        BufferRegion::new()
            .with_offset(0)
            .with_size(len_bytes as u32)
            .with_buffer(&buffer),
        true,
    );

    Ok(buffer)
}
