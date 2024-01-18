use super::app;
use crate::Config;

pub fn start(config: Config) {
    pollster::block_on(app::start());
}
