use ggez::graphics::Color;

use crate::{
    grid::{Cell, Coord, Grid},
    utils::vary_color,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParticleKind {
    Sand,
    Water,
    Wood,
}

impl ParticleKind {
    pub fn base_color(&self) -> Color {
        match self {
            ParticleKind::Sand => Color::YELLOW,
            ParticleKind::Water => Color::BLUE,
            ParticleKind::Wood => Color::from_rgb(112, 74, 2),
        }
    }

    fn generate_color(&self) -> Color {
        vary_color(self.base_color())
    }
}

pub struct Particle {
    pub color: Color,
    pub kind: ParticleKind,
    ticked: bool,
}

impl Particle {
    pub fn new(kind: ParticleKind) -> Particle {
        Particle {
            color: kind.generate_color(),
            ticked: false,
            kind,
        }
    }
}

trait ParticleCell {
    fn is_solid(&self) -> bool;
}

impl ParticleCell for Cell<Particle> {
    fn is_solid(&self) -> bool {
        if let Some(particle) = &self.value {
            match particle.kind {
                ParticleKind::Sand | ParticleKind::Wood => true,
                ParticleKind::Water => false,
            }
        } else {
            false
        }
    }
}

trait ParticleGrid {
    fn is_solid(&self, coord: &Coord) -> bool;
}

impl ParticleGrid for Grid<Particle> {
    fn is_solid(&self, coord: &Coord) -> bool {
        let cell = self.get(coord);
        cell.is_solid()
    }
}

pub struct Simulator {}

impl Simulator {
    pub fn new() -> Simulator {
        Simulator {}
    }

    pub fn init(&mut self, grid: &mut Grid<Particle>) {
        grid.iter_mut()
            .filter_map(|cell| cell.value.as_mut())
            .for_each(|particle| particle.ticked = false);
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

        if particle.ticked {
            return;
        }
        particle.ticked = true;

        match particle.kind {
            ParticleKind::Sand => {
                if coord.is_at_bottom() {
                    return;
                }

                if let Some(other) = coord.move_by(0, 1).filter(|c| !grid.is_solid(c)) {
                    grid.swap(coord, &other);
                    return;
                }

                if let Some(other) = coord
                    .random_neighbors(vec![(-1, 1), (1, 1)])
                    .find(|c| !grid.get(c).is_solid())
                {
                    // try to move the cell we are moving into up instead of simply swapping.
                    // this is an attempt to prevent water from "climbing" up a diagonal line of
                    // sand.
                    if let Some(side) = other.move_by(0, -1).filter(|c| grid.is_empty(c)) {
                        grid.swap(&other, &side);
                    }
                    grid.swap(coord, &other);
                }
            }
            ParticleKind::Water => {
                if coord.is_at_bottom() {
                    return;
                }

                if let Some(other) = coord.move_by(0, 1).filter(|c| grid.is_empty(c)) {
                    grid.swap(coord, &other);
                    return;
                }
                if let Some(other) = coord
                    .random_neighbors(vec![(-1, 1), (1, 1)])
                    .find(|c| grid.get(c).is_empty())
                {
                    grid.swap(coord, &other);
                }
                if let Some(other) = coord
                    .random_neighbors(vec![(-1, 0), (1, 0)])
                    .find(|c| grid.get(c).is_empty())
                {
                    grid.swap(coord, &other);
                }
            }
            ParticleKind::Wood => {
                // Do nothing
            }
        }
    }
}
