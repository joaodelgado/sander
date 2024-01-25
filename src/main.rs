use ggez::{
    conf, event,
    graphics::{self, Canvas, FillOptions, StrokeOptions},
    timer, Context, ContextBuilder, GameError,
};

const WINDOW_HEIGHT: usize = 800;
const WINDOW_WIDTH: usize = 1024;
const CELL_SIZE: usize = 5;
const DROPPER_SIZE: isize = 5;
const GRID_HEIGHT: isize = WINDOW_HEIGHT as isize / CELL_SIZE as isize;
const GRID_WIDTH: isize = WINDOW_WIDTH as isize / CELL_SIZE as isize;

struct Grid {
    cells: Vec<bool>,
    next_cells: Vec<bool>,
}

impl Grid {
    fn new() -> Grid {
        Grid {
            cells: vec![false; GRID_HEIGHT as usize * GRID_WIDTH as usize],
            next_cells: vec![false; GRID_HEIGHT as usize * GRID_WIDTH as usize],
        }
    }

    fn screen_to_grid_coords(&self, x: f32, y: f32) -> (usize, usize) {
        (
            usize::try_from((x / CELL_SIZE as f32) as i32).unwrap_or(0),
            usize::try_from((y / CELL_SIZE as f32) as i32).unwrap_or(0),
        )
    }

    fn cell_index(&self, x: isize, y: isize) -> Option<usize> {
        if x > 0 && x < GRID_WIDTH && y > 0 && y < GRID_HEIGHT {
            Some((y * GRID_WIDTH + x) as usize)
        } else {
            None
        }
    }

    fn get_cell(&self, x: isize, y: isize) -> Option<bool> {
        self.cell_index(x, y).map(|i| self.cells[i])
    }

    fn set_cell(&mut self, x: isize, y: isize) {
        if let Some(i) = self.cell_index(x, y) {
            self.next_cells[i] = true;
        }
    }

    fn swap_grid(&mut self) {
        self.cells = self.next_cells.clone();
        self.next_cells = vec![false; GRID_HEIGHT as usize * GRID_WIDTH as usize];
    }

    fn update(&mut self) -> Result<(), GameError> {
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                if self.get_cell(x, y) != Some(true) {
                    continue;
                }
                if self.get_cell(x, y + 1) == Some(false) {
                    self.set_cell(x, y + 1);
                } else if self.get_cell(x - 1, y + 1) == Some(false) {
                    self.set_cell(x - 1, y + 1);
                } else if self.get_cell(x + 1, y + 1) == Some(false) {
                    self.set_cell(x + 1, y + 1);
                } else {
                    self.set_cell(x, y);
                }
            }
        }
        self.swap_grid();
        Ok(())
    }

    fn draw(&mut self, ctx: &Context, canvas: &mut Canvas) -> Result<(), GameError> {
        // TODO remove draw logic from the grid itself.
        // instead the grid should just provide an iterator of cells?
        let mut mb = graphics::MeshBuilder::new();
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                if self.get_cell(x, y) == Some(true) {
                    mb.rectangle(
                        if self.get_cell(x, y) == Some(true) {
                            graphics::DrawMode::Fill(FillOptions::DEFAULT)
                        } else {
                            graphics::DrawMode::Stroke(StrokeOptions::DEFAULT)
                        },
                        [
                            x as f32 * CELL_SIZE as f32,
                            y as f32 * CELL_SIZE as f32,
                            CELL_SIZE as f32,
                            CELL_SIZE as f32,
                        ]
                        .into(),
                        graphics::Color::WHITE,
                    )?;
                }
            }
        }
        let grid_mesh = graphics::Mesh::from_data(ctx, mb.build());

        canvas.draw(&grid_mesh, graphics::DrawParam::default());

        Ok(())
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::new()
    }
}

struct State {
    grid: Grid,
    mouse_down: bool,
}

impl State {
    fn new() -> State {
        State {
            grid: Grid::new(),
            mouse_down: false,
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
        while ctx.time.check_update_time(60) {
            if self.mouse_down {
                let (x, y) = self
                    .grid
                    .screen_to_grid_coords(ctx.mouse.position().x, ctx.mouse.position().y);
                for j in (y as isize - DROPPER_SIZE)..(y as isize + DROPPER_SIZE) {
                    for i in (x as isize - DROPPER_SIZE)..(x as isize + DROPPER_SIZE) {
                        self.grid.set_cell(i, j);
                    }
                }
            }
            self.grid.update()?;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);

        self.grid.draw(ctx, &mut canvas)?;

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
