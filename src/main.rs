mod grid;

use ggez::{
    conf, event,
    graphics::{self, FillOptions},
    input::mouse::MouseContext,
    timer, Context, ContextBuilder, GameError,
};

use grid::{Coord, Grid};

const WINDOW_HEIGHT: usize = 800;
const WINDOW_WIDTH: usize = 1024;
const CELL_SIZE: usize = 5;
const DROPPER_SIZE: isize = 5;
const GRID_HEIGHT: isize = WINDOW_HEIGHT as isize / CELL_SIZE as isize;
const GRID_WIDTH: isize = WINDOW_WIDTH as isize / CELL_SIZE as isize;

trait MouseExt {
    fn grid_position(&self) -> Coord;
}

impl MouseExt for MouseContext {
    fn grid_position(&self) -> Coord {
        let p = self.position();

        (
            (p.x / CELL_SIZE as f32) as i32 as isize,
            (p.y / CELL_SIZE as f32) as i32 as isize,
        )
            .into()
    }
}

struct State {
    grid: Grid<()>,
    next_grid: Grid<()>,
    mouse_down: bool,
}

impl State {
    fn new() -> State {
        State {
            grid: Grid::new(GRID_WIDTH, GRID_HEIGHT),
            next_grid: Grid::new(GRID_WIDTH, GRID_HEIGHT),
            mouse_down: false,
        }
    }

    fn swap_grid(&mut self) {
        self.grid = self.next_grid.clone();
        self.next_grid = Grid::new(GRID_WIDTH, GRID_HEIGHT);
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
        while ctx.time.check_update_time(60) {
            if self.mouse_down {
                for coord in ctx.mouse.grid_position().expand_area(DROPPER_SIZE) {
                    self.grid.set(&coord, ());
                }
            }

            for cell in self.grid.get_all().filter(|cell| !cell.is_empty()) {
                let coord_bellow = cell.coord.directly_bellow();
                if self.grid.is_empty(&coord_bellow) {
                    self.next_grid.set(&coord_bellow, ());
                } else {
                    let next_candidates: Vec<_> = cell
                        .coord
                        .neighbors()
                        .into_iter()
                        .filter(|c| c.is_bellow(&cell.coord) && c.is_lateral(&cell.coord))
                        .filter(|c| self.grid.is_empty(c) && self.next_grid.is_empty(c))
                        .collect();
                    if let Some(next) = self.next_grid.get_random_mut(&next_candidates) {
                        next.set(());
                    } else {
                        self.next_grid.set(&cell.coord, ())
                    }
                }
            }
            self.swap_grid();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);

        let mut mb = graphics::MeshBuilder::new();
        for cell in self.grid.get_all().filter(|cell| !cell.is_empty()) {
            mb.rectangle(
                graphics::DrawMode::Fill(FillOptions::DEFAULT),
                [
                    cell.coord.x as f32 * CELL_SIZE as f32,
                    cell.coord.y as f32 * CELL_SIZE as f32,
                    CELL_SIZE as f32,
                    CELL_SIZE as f32,
                ]
                .into(),
                graphics::Color::WHITE,
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
        width: WINDOW_WIDTH as f32,
        height: WINDOW_HEIGHT as f32,
        ..Default::default()
    });
    let (ctx, event_loop) = ContextBuilder::new("Sander", "joaomtdelgado@gmail.com")
        .default_conf(c)
        .build()
        .expect("error building ggez context");

    event::run(ctx, event_loop, state);
}
