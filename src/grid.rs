use rand::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Coord {
    pub x: isize,
    pub y: isize,
}

impl Coord {
    pub fn is_bellow(&self, other: &Coord) -> bool {
        self.y > other.y
    }

    pub fn is_lateral(&self, other: &Coord) -> bool {
        self.x != other.x
    }

    pub fn neighbors(&self) -> [Coord; 8] {
        [
            (self.x - 1, self.y - 1).into(),
            (self.x, self.y - 1).into(),
            (self.x + 1, self.y - 1).into(),
            (self.x - 1, self.y).into(),
            (self.x + 1, self.y).into(),
            (self.x - 1, self.y + 1).into(),
            (self.x, self.y + 1).into(),
            (self.x + 1, self.y + 1).into(),
        ]
    }

    pub fn directly_bellow(&self) -> Coord {
        (self.x, self.y + 1).into()
    }

    pub fn expand_area(&self, radious: impl Into<isize>) -> Vec<Coord> {
        let mut coords = Vec::new();
        let radious = radious.into();
        for j in (self.y - radious)..=(self.y + radious) {
            for i in (self.x - radious)..=(self.x + radious) {
                coords.push((i, j).into())
            }
        }
        coords
    }
}

impl From<(isize, isize)> for Coord {
    fn from(value: (isize, isize)) -> Self {
        Coord {
            x: value.0,
            y: value.1,
        }
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

    pub fn set(&mut self, value: T) {
        self.value = Some(value);
    }

    pub fn is_empty(&self) -> bool {
        self.value.is_none()
    }
}

#[derive(Clone)]
pub struct Grid<T> {
    cells: Vec<Cell<T>>,
    width: isize,
    height: isize,
}

impl<T> Grid<T> {
    pub fn new(width: isize, height: isize) -> Grid<T> {
        let mut cells = Vec::with_capacity((width * height) as usize);
        for y in 0..height {
            for x in 0..width {
                cells.push(Cell::empty((x, y).into()))
            }
        }

        Grid {
            cells,
            width,
            height,
        }
    }

    fn is_valid(&self, coord: &Coord) -> bool {
        coord.x >= 0 && coord.x < self.width && coord.y >= 0 && coord.y < self.height
    }

    fn cell_index(&self, coord: &Coord) -> Option<usize> {
        if self.is_valid(coord) {
            Some((coord.y * self.width + coord.x) as usize)
        } else {
            None
        }
    }

    pub fn is_empty(&self, coord: &Coord) -> bool {
        self.get(coord).map(|cell| cell.is_empty()).unwrap_or(false)
    }

    pub fn get(&self, coord: &Coord) -> Option<&Cell<T>> {
        match self.cell_index(coord) {
            Some(i) => Some(&self.cells[i]),
            None => None,
        }
    }

    pub fn get_mut(&mut self, coord: &Coord) -> Option<&mut Cell<T>> {
        match self.cell_index(coord) {
            Some(i) => Some(&mut self.cells[i]),
            None => None,
        }
    }

    pub fn get_random_mut(&mut self, coord: &[Coord]) -> Option<&mut Cell<T>> {
        coord
            .iter()
            .filter(|c| self.is_valid(c))
            .choose(&mut thread_rng())
            .and_then(|c| self.get_mut(c))
    }

    pub fn set(&mut self, coord: &Coord, value: T) {
        if let Some(cell) = self.get_mut(coord) {
            cell.set(value)
        }
    }

    pub fn get_all(&self) -> impl Iterator<Item = &'_ Cell<T>> {
        self.cells.iter()
    }
}
