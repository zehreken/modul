use super::windows::Windows;
use crate::core::Modul;
use crate::winit_view::app::App;
use egui::ViewportId;
use egui_wgpu::wgpu::TextureFormat;
use egui_wgpu::{Renderer, ScreenDescriptor};
use egui_winit::{
    egui::{ClippedPrimitive, Context, TexturesDelta},
    State,
};
use wgpu::{Device, Queue};
use winit::window::Window;

struct Test {
    is_window_open: bool,
    windows: Windows,
}

impl Test {
    fn new(ctx: &Context) -> Self {
        Self {
            is_window_open: false,
            windows: Windows::new(ctx),
        }
    }

    fn draw(&mut self, ctx: &Context, modul: &mut Modul) {
        self.windows.draw(ctx, modul);
    }
}

pub struct Gui {
    ctx: Context,
    state: State,
    renderer: Renderer,
    screen_descriptor: ScreenDescriptor,
    view: Test,
    paint_jobs: Vec<ClippedPrimitive>,
    textures: TexturesDelta,
    width: u32,
    height: u32,
}

impl Gui {
    pub fn new(window: &Window, device: &wgpu::Device, texture_format: TextureFormat) -> Self {
        let scale_factor = window.scale_factor();
        let size = window.inner_size();
        let max_texture_size = device.limits().max_texture_dimension_2d as usize;

        let egui_ctx = Context::default();
        let egui_state = egui_winit::State::new(
            egui_ctx.clone(),
            ViewportId::ROOT,
            window,
            Some(scale_factor as f32),
            Some(max_texture_size),
        );
        let max_texture_size = device.limits().max_texture_dimension_2d as usize;

        let screen_descriptor = ScreenDescriptor {
            size_in_pixels: [size.width, size.height],
            pixels_per_point: scale_factor as f32,
        };
        let renderer = Renderer::new(device, texture_format, None, 1);
        let textures = TexturesDelta::default();

        let view = Test::new(&egui_ctx);

        Self {
            ctx: egui_ctx,
            state: egui_state,
            renderer,
            screen_descriptor,
            view,
            paint_jobs: vec![],
            textures,
            width: size.width,
            height: size.height,
        }
    }

    pub fn handle_event(&mut self, window: &Window, event: &winit::event::WindowEvent) {
        let _ = self.state.on_window_event(window, event);
    }

    // resize
    pub fn resize(&mut self, width: u32, height: u32, scale_factor: f32) {
        self.screen_descriptor = ScreenDescriptor {
            size_in_pixels: [width, height],
            pixels_per_point: scale_factor,
        };
    }

    // update scale factor

    pub fn render(
        &mut self,
        window: &Window,
        render_target: &wgpu::TextureView,
        device: &Device,
        queue: &Queue,
        modul: &mut Modul,
    ) {
        let raw_input = self.state.take_egui_input(window);
        let output = self.ctx.run(raw_input, |egui_ctx| {
            self.view.draw(egui_ctx, modul);
        });

        self.textures.append(output.textures_delta);
        self.state
            .handle_platform_output(window, output.platform_output);
        self.paint_jobs = self
            .ctx
            .tessellate(output.shapes, window.scale_factor() as f32);

        // Upload all resources to the GPU.
        for (id, image_delta) in &self.textures.set {
            self.renderer
                .update_texture(&device, &queue, *id, image_delta);
        }

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("gui encoder"),
        });

        self.renderer.update_buffers(
            &device,
            &queue,
            &mut encoder,
            &self.paint_jobs,
            &self.screen_descriptor,
        );

        // Render egui with WGPU
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("egui"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: render_target,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            self.renderer
                .render(&mut rpass, &self.paint_jobs, &self.screen_descriptor);
        }
        // dropping rpass here
        &queue.submit(Some(encoder.finish()));
        // Cleanup
        let textures = std::mem::take(&mut self.textures);
        for id in &textures.free {
            self.renderer.free_texture(id);
        }
    }
}
