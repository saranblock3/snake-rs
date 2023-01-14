use crate::direction::Direction;

pub struct Snake {
    pub head: (i32, i32),
    tail: Vec<(i32, i32)>,
    direction: Direction,
    growing: bool,
}

impl Snake {
    pub fn new(width: i32, height: i32) -> Snake {
        Snake {
            head: (width, height),
            tail: vec![],
            direction: Direction::Up,
            growing: false,
        }
    }

    pub fn slither(&mut self) {
        self.tail.push(self.head);
        self.head = match self.direction {
            Direction::Up => (self.head.0, self.head.1-1),
            Direction::Left => (self.head.0-1, self.head.1),
            Direction::Down => (self.head.0, self.head.1+1),
            Direction::Right => (self.head.0+1, self.head.1),
        };
        if !self.growing {
            self.tail.remove(0);
        } else {
            self.growing = false;
        }
    }

    pub fn contains_point(&self, x: i32, y: i32) -> bool {
        if self.head == (x, y) || self.tail.contains(&(x, y)) {
            return true;
        }
        return false;
    }

    pub fn tail_contains_point(&self, x: i32, y: i32) -> bool {
        return self.tail.contains(&(x, y));
    }

    pub fn set_direction(&mut self, direction: Direction) {
        if !self.direction.opposite(&direction) {
            self.direction = direction;
        }
    }

    pub fn set_growing(&mut self, growing: bool) {
        self.growing = growing;
    }
}