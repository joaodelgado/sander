mod grid;
mod particles;
mod utils;

use ggegui::{
    egui::{self, Button, Slider},
    Gui,
};
use ggez::{
    conf, event, glam,
    graphics::{self, DrawParam, FillOptions},
    input::mouse::MouseContext,
    timer,
    winit::event::VirtualKeyCode,
    Context, ContextBuilder, GameError,
};
use particles::{Particle, ParticleKind, Simulator};
use rand::prelude::*;

use grid::{Coord, Grid};
use rand::{rngs::ThreadRng, thread_rng};

const GRID_HEIGHT: isize = 200;
const GRID_WIDTH: isize = 200;
const CELL_SIZE: usize = 5;
const WINDOW_HEIGHT: f32 = GRID_HEIGHT as f32 * CELL_SIZE as f32;
const WINDOW_WIDTH: f32 = GRID_WIDTH as f32 * CELL_SIZE as f32;
const TARGET_FPS: u32 = 120;

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
    gui: Gui,
    grid: Grid<Particle>,
    simulator: Simulator,
    dropper_size: isize,
    selected_particle_kind: Option<ParticleKind>,
    mouse_on_ui: bool,
    keyboard_on_ui: bool,
    rng: ThreadRng,
}

impl State {
    fn new(ctx: &mut Context) -> State {
        State {
            gui: Gui::new(ctx),
            grid: Grid::new(GRID_WIDTH, GRID_HEIGHT),
            simulator: Simulator::new(),
            dropper_size: 5,
            selected_particle_kind: Some(ParticleKind::Sand),
            mouse_on_ui: false,
            keyboard_on_ui: false,
            rng: thread_rng(),
        }
    }

    fn update_ui(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        let gui_ctx = self.gui.ctx();
        egui::Window::new("Menu").show(&gui_ctx, |ui| {
            ui.add(Slider::new(&mut self.dropper_size, 1..=20).text("Dropper size"));

            if ui
                .add(Button::new("Empty").selected(self.selected_particle_kind.is_none()))
                .clicked()
            {
                self.selected_particle_kind = None;
            }
            if ui
                .add(
                    Button::new("Sand")
                        .selected(self.selected_particle_kind == Some(ParticleKind::Sand)),
                )
                .clicked()
            {
                self.selected_particle_kind = Some(ParticleKind::Sand);
            }
            if ui
                .add(
                    Button::new("Wood")
                        .selected(self.selected_particle_kind == Some(ParticleKind::Wood)),
                )
                .clicked()
            {
                self.selected_particle_kind = Some(ParticleKind::Wood);
            }
            if ui
                .add(
                    Button::new("Water")
                        .selected(self.selected_particle_kind == Some(ParticleKind::Water)),
                )
                .clicked()
            {
                self.selected_particle_kind = Some(ParticleKind::Water);
            }

            if ui.button("quit").clicked() {
                ctx.request_quit();
            }
        });

        self.mouse_on_ui = gui_ctx.wants_pointer_input();
        self.keyboard_on_ui = gui_ctx.wants_keyboard_input();

        Ok(())
    }
}

impl event::EventHandler<GameError> for State {
    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        input: ggez::input::keyboard::KeyInput,
        _repeated: bool,
    ) -> Result<(), GameError> {
        if self.keyboard_on_ui {
            return Ok(());
        }

        match input.keycode {
            Some(VirtualKeyCode::S) => self.selected_particle_kind = Some(ParticleKind::Sand),
            Some(VirtualKeyCode::W) => self.selected_particle_kind = Some(ParticleKind::Wood),
            Some(VirtualKeyCode::D) => self.selected_particle_kind = Some(ParticleKind::Water),
            Some(VirtualKeyCode::E) => self.selected_particle_kind = None,
            _ => (),
        }

        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        self.update_ui(ctx)?;

        while ctx.time.check_update_time(TARGET_FPS) {
            if ctx.mouse.button_pressed(event::MouseButton::Left) && !self.mouse_on_ui {
                for coord in ctx
                    .mouse
                    .grid_position()
                    .iter()
                    .flat_map(|c| c.neighbors(self.dropper_size).into_iter())
                {
                    match self.selected_particle_kind {
                        Some(kind) => self.grid.set(&coord, Particle::new(kind)),
                        None => self.grid.clear(&coord),
                    }
                }
            }

            self.simulator.init(&mut self.grid);

            for y in (0..self.grid.height).rev() {
                let row_range = if self.rng.gen() {
                    itertools::Either::Left(0..self.grid.width)
                } else {
                    itertools::Either::Right((0..self.grid.width).rev())
                };

                for x in row_range {
                    if let Some(coord) = self.grid.to_coord((x, y)) {
                        self.simulator.simulate(&mut self.grid, &coord);
                    }
                }
            }
        }
        self.gui.update(ctx);
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
                particle.color,
            )?;
        }
        let grid_mesh = graphics::Mesh::from_data(ctx, mb.build());

        canvas.draw(&grid_mesh, graphics::DrawParam::default());

        // Draw UI
        canvas.draw(&self.gui, DrawParam::default().dest(glam::Vec2::ZERO));

        canvas.finish(ctx)?;

        timer::yield_now();
        Ok(())
    }
}

fn main() {
    let c = conf::Conf::new().window_mode(conf::WindowMode {
        width: WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
        ..Default::default()
    });
    let (mut ctx, event_loop) = ContextBuilder::new("Sander", "joaomtdelgado@gmail.com")
        .default_conf(c)
        .build()
        .expect("error building ggez context");

    let state = State::new(&mut ctx);

    event::run(ctx, event_loop, state);
}
