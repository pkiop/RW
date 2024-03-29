use crate::random::random_range;
use std::collections::VecDeque;

pub type Position = (usize, usize);

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

#[derive(Debug)]
pub struct SnakeGame {
    pub width: usize,
    pub height: usize,
    pub snake: VecDeque<Position>, // first vector item is Head, last is Tail
    pub direction: Direction,
    next_direction: Direction,
    pub food: Position,
    pub finished: bool,
}

impl SnakeGame {
    // constructor fn
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            snake: [((width - 2).max(0), height / 2)].into_iter().collect(),
            next_direction: Direction::Left,
            direction: Direction::Left,
            food: (2.min(width - 1), height / 2),
            finished: false,
        }
    }

    pub fn change_direction(&mut self, direction: Direction) {
        if self.finished {
            return;
        }

        match (&self.direction, direction) {
            (Direction::Top, Direction::Top)
            | (Direction::Top, Direction::Bottom)
            | (Direction::Right, Direction::Right)
            | (Direction::Right, Direction::Left)
            | (Direction::Bottom, Direction::Bottom)
            | (Direction::Bottom, Direction::Top)
            | (Direction::Left, Direction::Left)
            | (Direction::Left, Direction::Right) => {}
            (_, direction) => self.next_direction = direction,
        }
    }

    pub fn is_valid(&self, (x, y): Position) -> bool {
        x < self.width && y < self.height
    }

    pub fn tick(&mut self) {
        if self.finished && self.snake.len() == 0 {
            return;
        }

        self.direction = self.next_direction;
        // let head = self.snake.get(0); // for memory safty (if length 0, snake[0] => occur error)
        let (x, y) = self.snake[0]; // for memory safty (if length 0, snake[0] => occur error)
        let new_head = match &self.direction {
            Direction::Top => (x, y - 1),
            Direction::Right => (x + 1, y),
            Direction::Bottom => (x, y + 1),
            Direction::Left => (x - 1, y),
        };

        if !self.is_valid(new_head) || self.snake.contains(&new_head) {
            self.finished = true;
        } else {
            if new_head != self.food {
                self.snake.pop_back();
            } else {
                let free_positions = (0..self.height)
                    .flat_map(|y| (0..self.width).map(move |x| (x, y)))
                    .filter(|pos| !self.snake.contains(pos))
                    .collect::<Vec<_>>();

                if free_positions.is_empty() {
                    self.finished = true;
                    return;
                }
                self.food = free_positions[random_range(0, free_positions.len())];
            }
            self.snake.push_front(new_head);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::SnakeGame;

    #[test]
    fn test() {
        println!("run test");
        println!("{:?}", SnakeGame::new(10, 10));
    }
    #[test]
    fn test2() {
        println!("run test");
        println!("{:?}", SnakeGame::new(10, 10));
    }
}
