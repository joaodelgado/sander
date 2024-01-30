use ggez::graphics::Color;

use crate::{
    grid::{Coord, Grid},
    utils::vary_color,
};

#[derive(Debug, Clone, Copy)]
pub enum ParticleKind {
    Sand,
    Wood,
}

impl ParticleKind {
    fn generate_color(&self) -> Color {
        match self {
            ParticleKind::Sand => vary_color(Color::YELLOW),
            ParticleKind::Wood => vary_color(Color::from_rgb(112, 74, 2)),
        }
    }
}

pub struct Particle {
    pub color: Color,
    kind: ParticleKind,
}

impl Particle {
    pub fn new(kind: ParticleKind) -> Particle {
        Particle {
            color: kind.generate_color(),
            kind,
        }
    }
}

pub struct Simulator {}

impl Simulator {
    pub fn new() -> Simulator {
        Simulator {}
    }

    pub fn simulate(&mut self, grid: &mut Grid<Particle>, coord: &Coord) {
        let cell = grid.get_mut(coord);
        if cell.is_empty() {
            return;
        }
        let particle = cell
            .value
            .as_mut()
            .expect("already checked that cell is not empty");

        match particle.kind {
            ParticleKind::Sand => {
                if coord.is_at_bottom() {
                    return;
                }

                if let Some(other) = coord.move_by(0, 1).filter(|c| grid.is_empty(c)) {
                    grid.swap(coord, &other);
                } else if let Some(other) = coord.move_by(-1, 1).filter(|c| grid.is_empty(c)) {
                    grid.swap(coord, &other);
                } else if let Some(other) = coord.move_by(1, 1).filter(|c| grid.is_empty(c)) {
                    grid.swap(coord, &other);
                }
            }
            ParticleKind::Wood => {
                // Do nothing
            }
        }
    }
}
