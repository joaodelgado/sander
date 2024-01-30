use rand::prelude::*;
use std::ptr;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    fn in_bounds(&self, min_x: isize, max_x: isize, min_y: isize, max_y: isize) -> bool {
        self.x >= min_x && self.x < max_x && self.y >= min_y && self.y < max_y
    }

    fn distance(&self, other: &Point) -> f32 {
        let diff_x = self.x - other.x;
        let diff_y = self.y - other.y;
        ((diff_x * diff_x + diff_y * diff_y) as f32).sqrt()
    }
}

impl<T: Into<isize>> From<(T, T)> for Point {
    fn from((x, y): (T, T)) -> Self {
        Point {
            x: x.into(),
            y: y.into(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Coord {
    pub p: Point,
    world_height: isize,
    world_width: isize,
}

impl Coord {
    pub fn new(p: impl Into<Point>, world_width: isize, world_height: isize) -> Option<Coord> {
        let p = p.into();
        if p.in_bounds(0, world_width, 0, world_height) {
            Some(Coord {
                p,
                world_width,
                world_height,
            })
        } else {
            None
        }
    }

    pub fn move_by(&self, x: isize, y: isize) -> Option<Coord> {
        self.move_to((self.p.x + x, self.p.y + y))
    }

    fn move_to(&self, p: impl Into<Point>) -> Option<Coord> {
        Coord::new(p.into(), self.world_width, self.world_height)
    }

    fn cell_index(&self) -> usize {
        (self.p.y * self.world_width + self.p.x) as usize
    }

    pub fn is_at_bottom(&self) -> bool {
        self.p.y == self.world_height - 1
    }

    pub fn random_neighbors(
        &self,
        mut motions: Vec<(isize, isize)>,
    ) -> impl Iterator<Item = Coord> + '_ {
        motions.shuffle(&mut thread_rng());
        motions
            .into_iter()
            .filter_map(move |(x, y)| self.move_by(x, y))
    }

    pub fn neighbors(&self, radious: impl Into<isize>) -> Vec<Coord> {
        let radious = radious.into();
        let mut neighbors = Vec::new();

        for j in (self.p.y - radious)..=(self.p.y + radious) {
            for i in (self.p.x - radious)..=(self.p.x + radious) {
                let other = (i, j).into();
                if ((self.p.distance(&other).round()) as isize) < radious {
                    if let Some(neighbor) = self.move_to(other) {
                        neighbors.push(neighbor);
                    }
                }
            }
        }
        neighbors
    }
}

#[derive(Debug, Clone)]
pub struct Cell<T> {
    pub value: Option<T>,
    pub coord: Coord,
}

impl<T> Cell<T> {
    fn empty(coord: Coord) -> Cell<T> {
        Cell { value: None, coord }
    }

    pub fn is_empty(&self) -> bool {
        self.value.is_none()
    }
}

#[derive(Debug, Clone)]
pub struct Grid<T> {
    cells: Vec<Cell<T>>,
    pub width: isize,
    pub height: isize,
}

impl<T> Grid<T> {
    pub fn new(width: isize, height: isize) -> Grid<T> {
        let mut cells = Vec::with_capacity((width * height) as usize);
        for y in 0..height {
            for x in 0..width {
                cells.push(Cell::empty(
                    Coord::new((x, y), width, height)
                        .expect("iterating through valid coordinates only"),
                ))
            }
        }

        Grid {
            cells,
            width,
            height,
        }
    }

    pub fn is_empty(&self, coord: &Coord) -> bool {
        self.get(coord).is_empty()
    }

    pub fn to_coord(&self, p: impl Into<Point>) -> Option<Coord> {
        Coord::new(p, self.width, self.height)
    }

    pub fn get(&self, coord: &Coord) -> &Cell<T> {
        self.cells
            .get(coord.cell_index())
            .expect("coordinates are always valid")
    }

    pub fn get_mut(&mut self, coord: &Coord) -> &mut Cell<T> {
        self.cells
            .get_mut(coord.cell_index())
            .expect("coordinates are always valid")
    }

    pub fn clear(&mut self, coord: &Coord) {
        self.get_mut(coord).value = None;
    }

    pub fn set(&mut self, coord: &Coord, value: T) {
        self.get_mut(coord).value = Some(value);
    }

    pub fn swap(&mut self, a: &Coord, b: &Coord) {
        let a_value_ptr = ptr::addr_of_mut!(self.get_mut(a).value);
        let b_value_ptr = ptr::addr_of_mut!(self.get_mut(b).value);
        // Can't take two mutable references of the cells array.
        // No sure is safe, but it seems to be what Vec::swap is doing
        unsafe { ptr::swap(a_value_ptr, b_value_ptr) };
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Cell<T>> {
        self.cells.iter_mut()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Cell<T>> {
        self.cells.iter()
    }
}
