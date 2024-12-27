pub struct XYCoordinates {
    pos_x: u32,
    pos_y: u32,
    max_x: u32,
    max_y: u32,
}

use getch_rs::Key;

impl XYCoordinates {
    pub fn new(max_xy: (u32, u32)) -> Self {
        Self {
            pos_x: 0,
            pos_y: 0,
            max_x: max_xy.0,
            max_y: max_xy.1,
        }
    }

    pub fn update(&mut self, input: Key) {
        enum Direction {
            Up,
            MaxUp,
            Down,
            MaxDown,
            Left,
            MaxLeft,
            Right,
            MaxRight,
            Undefined,
        }

        let direc = match input {
            Key::Up => Direction::Up,
            Key::Char('k') => Direction::Up,
            Key::Char('K') => Direction::MaxUp,
            Key::Down => Direction::Down,
            Key::Char('j') => Direction::Down,
            Key::Char('J') => Direction::MaxDown,
            Key::Left => Direction::Left,
            Key::Char('h') => Direction::Left,
            Key::Char('H') => Direction::MaxLeft,
            Key::Right => Direction::Right,
            Key::Char('l') => Direction::Right,
            Key::Char('L') => Direction::MaxRight,
            _ => Direction::Undefined,
        };

        match (direc, (self.pos_x, self.max_x), (self.pos_y, self.max_y)) {
            (Direction::Undefined, ..) => (),
            (Direction::MaxUp, ..) => self.pos_y = 0,
            (Direction::MaxDown, _, (_, y)) => self.pos_y = y,
            (Direction::MaxLeft, ..) => self.pos_x = 0,
            (Direction::MaxRight, (_, x), _) => self.pos_x = x,
            (Direction::Up, _, (0, _)) => (),
            (Direction::Up, ..) => self.pos_y -= 1,
            (Direction::Down, .., (py, my)) if py == my => (),
            (Direction::Down, ..) => self.pos_y += 1,
            (Direction::Left, (0, _), _) => (),
            (Direction::Left, ..) => self.pos_x -= 1,
            (Direction::Right, (px, mx), _) if px == mx => (),
            (Direction::Right, ..) => self.pos_x += 1,
        }
    }

    pub fn get_xy(&self) -> (u32, u32) {
        (self.pos_x, self.pos_y)
    }

    pub fn set_xy(&mut self, xy: (u32, u32)) {
        match (xy, self.max_x, self.max_y) {
            ((a, _), b, _) if a > b => panic!("Invalid X number (set: {a}, max: {b})"),
            ((_, a), _, b) if a > b => panic!("Invalid Y number (set: {a}, max: {b})"),
            (pair, ..) => {
                self.pos_x = pair.0;
                self.pos_y = pair.1
            }
        }
    }
}

impl std::fmt::Display for XYCoordinates {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "x: {}, y: {}", self.pos_x, self.pos_y)
    }
}
