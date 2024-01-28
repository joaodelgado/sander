use ggez::graphics::Color;
use rand::prelude::*;

use crate::{
    grid::{Coord, Grid},
    utils::vary_color,
};

pub enum ParticleKind {
    Sand,
}

impl ParticleKind {
    fn generate_color(&self) -> Color {
        match self {
            ParticleKind::Sand => vary_color(Color::YELLOW),
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

pub struct Simulator {
    rng: ThreadRng,
}

impl Simulator {
    pub fn new() -> Simulator {
        Simulator { rng: thread_rng() }
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

                let bellow = coord
                    .directly_bellow()
                    .expect("already validated that it's not in the bottom row");
                if grid.is_empty(&bellow) {
                    grid.swap(coord, &bellow);
                } else {
                    let random_cell_bellow = coord
                        .bellow()
                        .filter(|c| c.p.is_lateral(&coord.p))
                        .filter(|c| grid.is_empty(c))
                        .choose(&mut self.rng);
                    if let Some(other) = random_cell_bellow {
                        grid.swap(coord, &other)
                    }
                }
            }
        }
    }
}
