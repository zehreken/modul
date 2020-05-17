pub enum EnvStage {
    Off,
    Attack,
    Decay,
    Sustain,
    Release,
}

pub struct Envelope {
    pub duration: f32,
    pub phase: f64,
    pub hz: f64,
}
