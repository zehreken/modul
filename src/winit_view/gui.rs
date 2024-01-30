use crate::view::{Modul, Windows};
use crate::winit_view::app::App;
use egui::{Color32, RichText};
use egui_wgpu::wgpu::TextureFormat;
use egui_wgpu::{renderer::ScreenDescriptor, Renderer};
use egui_winit::{
    egui::{self, ClippedPrimitive, Context, TexturesDelta},
    State,
};
use winit::{event_loop::EventLoopWindowTarget, window::Window};
const SCALE_FACTOR: f32 = 1.0;

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
    pub fn new<T>(
        event_loop: &EventLoopWindowTarget<T>,
        device: &wgpu::Device,
        texture_format: TextureFormat,
    ) -> Self {
        let (width, height) = (1600, 1200);
        let max_texture_size = device.limits().max_texture_dimension_2d as usize;

        let egui_ctx = Context::default();
        let mut egui_state = egui_winit::State::new(event_loop);
        egui_state.set_max_texture_side(max_texture_size);
        egui_state.set_pixels_per_point(SCALE_FACTOR);

        let screen_descriptor = ScreenDescriptor {
            size_in_pixels: [width, height],
            pixels_per_point: SCALE_FACTOR,
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
            width,
            height,
        }
    }

    pub fn handle_event(&mut self, event: &winit::event::WindowEvent) {
        let _ = self.state.on_event(&self.ctx, event);
    }

    // resize
    pub fn resize(&mut self, width: u32, height: u32) {
        self.screen_descriptor = ScreenDescriptor {
            size_in_pixels: [width, height],
            pixels_per_point: SCALE_FACTOR,
        };
    }

    // update scale factor

    pub fn render(
        &mut self,
        window: &Window,
        render_target: &wgpu::TextureView,
        app: &App,
        modul: &mut Modul,
    ) {
        let raw_input = self.state.take_egui_input(window);
        let output = self.ctx.run(raw_input, |egui_ctx| {
            self.view.draw(egui_ctx, modul);
        });

        self.textures.append(output.textures_delta);
        self.state
            .handle_platform_output(window, &self.ctx, output.platform_output);
        self.paint_jobs = self.ctx.tessellate(output.shapes);

        // Upload all resources to the GPU.
        for (id, image_delta) in &self.textures.set {
            self.renderer
                .update_texture(app.device(), app.queue(), *id, image_delta);
        }

        let mut encoder = app
            .device()
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("gui encoder"),
            });

        self.renderer.update_buffers(
            app.device(),
            app.queue(),
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
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            self.renderer
                .render(&mut rpass, &self.paint_jobs, &self.screen_descriptor);
        }
        // dropping rpass here
        app.queue().submit(Some(encoder.finish()));
        // Cleanup
        let textures = std::mem::take(&mut self.textures);
        for id in &textures.free {
            self.renderer.free_texture(id);
        }
    }
}
