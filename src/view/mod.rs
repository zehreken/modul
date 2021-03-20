mod quad;
use quad::shader;
use {egui_miniquad as egui_mq, miniquad as mq};
mod egui_view;
use super::modul;
use super::modul_utils;

struct Stage {
    egui_mq: egui_mq::EguiMq,
    show_egui_demo_windows: bool,
    _quad_stage: quad::Quad,
    egui_view: egui_view::EguiView,
    modul: modul::Modul,
}

impl Stage {
    fn new(ctx: &mut mq::Context) -> Self {
        Self {
            egui_mq: egui_mq::EguiMq::new(ctx),
            show_egui_demo_windows: true,
            _quad_stage: quad::Quad::new(ctx),
            egui_view: egui_view::EguiView::default(),
            modul: modul::Modul::new(),
        }
    }

    fn ui(&mut self) {
        let Self {
            egui_mq,
            show_egui_demo_windows,
            _quad_stage: quad_stage,
            egui_view,
            modul,
        } = self;

        let egui_ctx = egui_mq.egui_ctx();

        if *show_egui_demo_windows {
            egui_view.ui(egui_ctx, modul);
        }

        egui::Window::new("modul ❤ ").show(egui_ctx, |ui| {
            ui.checkbox(show_egui_demo_windows, "show modul ui");

            #[cfg(not(target_arch = "wasm32"))]
            {
                if ui.button("Quit").clicked() {
                    std::process::exit(0);
                }
            }
        });
    }
}

impl mq::EventHandler for Stage {
    fn update(&mut self, _ctx: &mut mq::Context) {}

    fn draw(&mut self, ctx: &mut mq::Context) {
        ctx.clear(Some((1., 1., 1., 1.)), None, None);
        ctx.begin_default_pass(mq::PassAction::clear_color(0.0, 0.0, 0.0, 1.0));

        // Draw things behind egui here
        ctx.apply_pipeline(&self._quad_stage.pipeline);
        ctx.apply_bindings(&self._quad_stage.bindings);

        // Pass data to shader
        ctx.apply_uniforms(&shader::Uniforms {
            offset: (0f32, 0f32),
            wavepoints: self.modul.get_sample_averages(),
        });
        ctx.draw(0, 6, 1);

        ctx.end_render_pass();

        self.egui_mq.begin_frame(ctx);
        self.ui();
        self.egui_mq.end_frame(ctx);

        // Draw things in front of egui here

        ctx.commit_frame();
    }

    fn mouse_motion_event(&mut self, ctx: &mut mq::Context, x: f32, y: f32) {
        self.egui_mq.mouse_motion_event(ctx, x, y);
    }

    fn mouse_wheel_event(&mut self, ctx: &mut mq::Context, dx: f32, dy: f32) {
        self.egui_mq.mouse_wheel_event(ctx, dx, dy);
    }

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut mq::Context,
        mb: mq::MouseButton,
        x: f32,
        y: f32,
    ) {
        self.egui_mq.mouse_button_down_event(ctx, mb, x, y);
    }

    fn mouse_button_up_event(
        &mut self,
        ctx: &mut mq::Context,
        mb: mq::MouseButton,
        x: f32,
        y: f32,
    ) {
        self.egui_mq.mouse_button_up_event(ctx, mb, x, y);
    }

    fn char_event(
        &mut self,
        _ctx: &mut mq::Context,
        character: char,
        _keymods: mq::KeyMods,
        _repeat: bool,
    ) {
        self.egui_mq.char_event(character);
    }

    fn key_down_event(
        &mut self,
        ctx: &mut mq::Context,
        keycode: mq::KeyCode,
        keymods: mq::KeyMods,
        _repeat: bool,
    ) {
        self.egui_mq.key_down_event(ctx, keycode, keymods);
    }

    fn key_up_event(&mut self, _ctx: &mut mq::Context, keycode: mq::KeyCode, keymods: mq::KeyMods) {
        self.egui_mq.key_up_event(keycode, keymods);
    }
}

pub fn start() {
    let conf = mq::conf::Conf {
        window_width: 700,
        window_height: 700,
        window_title: "modul".to_string(),
        high_dpi: true,
        ..Default::default()
    };
    mq::start(conf, |mut ctx| {
        mq::UserData::owning(Stage::new(&mut ctx), ctx)
    });
}
