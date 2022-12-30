use self::super::windows::Windows;
use super::scene::Scene;
// use super::scene2::Scene2;
use crate::core::Modul;
use crate::Config;
use egui::Context;
use {egui_miniquad as egui_mq, miniquad as mq};

struct Stage {
    windows: super::windows::Windows,
    egui_mq: egui_mq::EguiMq,
    scene: Scene,
    // scene2: Scene2,
    modul: Modul,
}

impl Stage {
    fn new(mq_ctx: &mut mq::Context, config: Config) -> Self {
        let egui_mq = egui_mq::EguiMq::new(mq_ctx);

        Self {
            windows: super::windows::Windows::new(egui_mq.egui_ctx()),
            egui_mq,
            scene: Scene::new(mq_ctx),
            // scene2: Scene2::new(mq_ctx),
            modul: Modul::new(&config),
        }
    }
}

impl mq::EventHandler for Stage {
    fn update(&mut self, _ctx: &mut mq::Context) {
        // TODO: Does the order of these calls matter?
        self.modul.update();
        self.scene.update();
    }

    fn draw(&mut self, ctx: &mut mq::Context) {
        ctx.begin_default_pass(mq::PassAction::clear_color(0.0, 0.0, 0.0, 1.0));

        // Draw things behind egui here
        // You can draw as many scenes as you want
        self.scene.draw(ctx, &self.modul);

        ctx.end_render_pass();

        self.egui_mq.run(ctx, |_mq_ctx, egui_ctx| {
            draw_ui(&mut self.windows, egui_ctx, &mut self.modul);
        });

        self.egui_mq.draw(ctx);

        // Draw things in front of egui here

        ctx.commit_frame();
    }

    fn mouse_motion_event(&mut self, _ctx: &mut mq::Context, x: f32, y: f32) {
        self.egui_mq.mouse_motion_event(x, y);
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut mq::Context, dx: f32, dy: f32) {
        self.egui_mq.mouse_wheel_event(dx, dy);
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

fn draw_ui(windows: &mut Windows, egui_ctx: &Context, modul: &mut Modul) {
    windows.draw(egui_ctx, modul);
}

pub fn start(config: Config) {
    let conf = mq::conf::Conf {
        window_width: 700,
        window_height: 700,
        window_title: "modul".to_string(),
        high_dpi: true,
        ..Default::default()
    };

    mq::start(conf, |ctx| Box::new(Stage::new(ctx, config)));
}
