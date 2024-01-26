use std::collections::VecDeque;

use wgpu::{Device, Queue, Surface, SurfaceCapabilities, SurfaceConfiguration, TextureFormat};
use winit::{
    dpi::{PhysicalSize, Size},
    event::{self, Event, WindowEvent},
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

use crate::{
    core::Modul,
    winit_view::{gui, renderer},
    Config,
};

pub struct AudioApp {}

pub struct App {
    window: Window,
    surface: Surface,
    device: Device,
    queue: Queue,
    _size: PhysicalSize<u32>,
    surface_config: SurfaceConfiguration,
    texture_format: TextureFormat,
    rolling_frame_time: VecDeque<f32>,
    config: Config,
    surface_caps: SurfaceCapabilities,
}

impl App {
    async fn new(window: Window, config: Config) -> App {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        // Since app owns the window, this is safe
        // App's lifetime is longer than surface
        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        // Async is fine but you can also use pollster::block_on without await
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        // Same with this one, pollster::block_on(adapter_request(...)).unwrap(); is another way
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::default(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None,
            )
            .await
            .unwrap();

        let size = window.inner_size();
        let surface_caps = surface.get_capabilities(&adapter);
        let texture_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: texture_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &surface_config);

        let init = vec![0.0; 60];
        Self {
            window,
            surface,
            device,
            queue,
            _size: size,
            surface_config,
            texture_format,
            rolling_frame_time: VecDeque::from(init),
            config,
            surface_caps,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: self.texture_format,
            width: width,
            height: height,
            present_mode: self.surface_caps.present_modes[0],
            alpha_mode: self.surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        self.surface.configure(&self.device, &self.surface_config);
    }

    pub fn device(&self) -> &Device {
        &self.device
    }

    pub fn queue(&self) -> &Queue {
        &self.queue
    }
}

pub async fn start(config: Config) {
    // let size = Size::Physical(PhysicalSize {
    //     width: 1600,
    //     height: 1200,
    // });
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_decorations(true)
        .with_resizable(true)
        .with_transparent(false)
        .with_title("modul ❤")
        // .with_inner_size(size)
        .build(&event_loop)
        .unwrap();

    let mut app = App::new(window, config).await;
    // create renderer
    let mut renderer = renderer::Renderer::new(&app.device, &app.surface_config);
    // create gui
    let mut gui = gui::Gui::new(&event_loop, &app.device, app.texture_format);

    let init = [0.0; 60];
    let mut rolling_frame_times = VecDeque::from(init);
    let mut earlier = std::time::Instant::now();
    let mut elapsed_time = 0.0;

    let mut modul = Modul::new(&app.config);

    event_loop.run(move |event, _elwt, control_flow| match event {
        Event::WindowEvent {
            window_id,
            event: WindowEvent::Resized(size),
        } if window_id == app.window.id() => {
            app.resize(size.width, size.height);
            gui.resize(size.width, size.height);
        }
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            window_id,
        } if window_id == app.window.id() => control_flow.set_exit(),
        Event::WindowEvent { event, .. } => {
            gui.handle_event(&event);
        }
        Event::MainEventsCleared => app.window.request_redraw(),
        Event::RedrawRequested(_) => {
            let frame_time = std::time::Instant::now().duration_since(earlier);
            elapsed_time += frame_time.as_secs_f32();
            earlier = std::time::Instant::now();
            rolling_frame_times.pop_front();
            rolling_frame_times.push_back(frame_time.as_secs_f32());
            let fps = calculate_fps(&rolling_frame_times);
            let output_frame = match app.surface.get_current_texture() {
                Ok(frame) => frame,
                Err(wgpu::SurfaceError::Outdated) => {
                    // This error occurs when the app is minimized on Windows.
                    // Silently return here to prevent spamming the console with:
                    // "The underlying surface has changed, and therefore the swap chain must be updated"
                    return;
                }
                Err(e) => {
                    eprintln!("Dropped frame with error: {}", e);
                    return;
                }
            };
            let output_view = output_frame
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());

            renderer.render(&app.device, &app.queue, &output_view, elapsed_time);
            gui.render(&app.window, &output_view, &app, fps, &mut modul);
            output_frame.present();

            // Modul does not complain at this point
            modul.update();
        }
        _ => {}
    });
}

pub fn calculate_fps(times: &VecDeque<f32>) -> f32 {
    let sum: f32 = times.iter().sum();

    let average_time = sum / times.len() as f32;
    return 1.0 / average_time;
}
