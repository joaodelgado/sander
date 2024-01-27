mod grid;
mod particles;
mod utils;

use ggez::{
    conf, event,
    graphics::{self, FillOptions},
    input::mouse::MouseContext,
    timer, Context, ContextBuilder, GameError,
};
use particles::Particle;
use rand::prelude::*;

use grid::{Coord, Grid};
use rand::{rngs::ThreadRng, seq::IteratorRandom, thread_rng};

const GRID_HEIGHT: isize = 200;
const GRID_WIDTH: isize = 300;
const CELL_SIZE: usize = 5;
const DROPPER_SIZE: isize = 7;
const WINDOW_HEIGHT: f32 = GRID_HEIGHT as f32 * CELL_SIZE as f32;
const WINDOW_WIDTH: f32 = GRID_WIDTH as f32 * CELL_SIZE as f32;

trait MouseExt {
    fn grid_position(&self) -> Option<Coord>;
}

impl MouseExt for MouseContext {
    fn grid_position(&self) -> Option<Coord> {
        let p = self.position();
        Coord::new(
            (
                (p.x / CELL_SIZE as f32) as i32 as isize,
                (p.y / CELL_SIZE as f32) as i32 as isize,
            ),
            GRID_WIDTH,
            GRID_HEIGHT,
        )
    }
}

struct State {
    grid: Grid<Particle>,
    mouse_down: bool,
    rng: ThreadRng,
}

impl State {
    fn new() -> State {
        State {
            grid: Grid::new(GRID_WIDTH, GRID_HEIGHT),
            mouse_down: false,
            rng: thread_rng(),
        }
    }
}

impl event::EventHandler<GameError> for State {
    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: event::MouseButton,
        _x: f32,
        _y: f32,
    ) -> Result<(), GameError> {
        self.mouse_down = true;
        Ok(())
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: event::MouseButton,
        _x: f32,
        _y: f32,
    ) -> Result<(), GameError> {
        self.mouse_down = false;
        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        while ctx.time.check_update_time(TARGET_FPS) {
            if self.mouse_down {
                for coord in ctx
                    .mouse
                    .grid_position()
                    .iter()
                    .flat_map(|c| c.neighbors(DROPPER_SIZE).into_iter())
                {
                    self.grid.set(&coord, Particle::new());
                }
            }

            for y in (0..self.grid.height).rev() {
                let row_range = if self.rng.gen() {
                    itertools::Either::Left(0..self.grid.width)
                } else {
                    itertools::Either::Right((0..self.grid.width).rev())
                };

                for x in row_range {
                    let cell = self
                        .grid
                        .get_point((x, y))
                        .expect("iterating only through valid indices");
                    if cell.is_empty() {
                        continue;
                    }

                    let coord = cell.coord;
                    if coord.is_at_bottom() {
                        continue;
                    }

                    let bellow = coord
                        .directly_bellow()
                        .expect("already validated that it's not in the bottom row");
                    if self.grid.is_empty(&bellow) {
                        self.grid.swap(&coord, &bellow);
                    } else {
                        let random_cell_bellow = coord
                            .bellow()
                            .filter(|c| c.p.is_lateral(&coord.p))
                            .filter(|c| self.grid.is_empty(c))
                            .choose(&mut self.rng);
                        if let Some(other) = random_cell_bellow {
                            self.grid.swap(&coord, &other)
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);

        let text = graphics::Text::new(format!("{:.2}", ctx.time.fps()));
        canvas.draw(&text, graphics::DrawParam::default());

        let mut mb = graphics::MeshBuilder::new();
        for (coord, particle) in self
            .grid
            .iter()
            .filter(|cell| !cell.is_empty())
            .map(|cell| {
                (
                    &cell.coord,
                    cell.value.as_ref().expect("already validated as no empty"),
                )
            })
        {
            mb.rectangle(
                graphics::DrawMode::Fill(FillOptions::DEFAULT),
                [
                    coord.p.x as f32 * CELL_SIZE as f32,
                    coord.p.y as f32 * CELL_SIZE as f32,
                    CELL_SIZE as f32,
                    CELL_SIZE as f32,
                ]
                .into(),
                *particle.color(),
            )?;
        }
        let grid_mesh = graphics::Mesh::from_data(ctx, mb.build());

        canvas.draw(&grid_mesh, graphics::DrawParam::default());

        canvas.finish(ctx)?;

        timer::yield_now();
        Ok(())
    }
}

fn main() {
    let state = State::new();

    let c = conf::Conf::new().window_mode(conf::WindowMode {
        width: WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
        ..Default::default()
    });
    let (ctx, event_loop) = ContextBuilder::new("Sander", "joaomtdelgado@gmail.com")
        .default_conf(c)
        .build()
        .expect("error building ggez context");

    event::run(ctx, event_loop, state);
}
