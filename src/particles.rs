use ggez::graphics::Color;

use crate::utils::vary_color;

pub enum Particle {
    Sand(Color),
}

impl Particle {
    pub fn new() -> Particle {
        Self::Sand(vary_color(Color::YELLOW))
    }

    pub fn color(&self) -> &Color {
        match self {
            Particle::Sand(color) => color,
        }
    }
}
