use std::ops::Add;
use ndarray;
use rand;
use rand::Rng;

type TileArray = ndarray::Array2<Tile>;
// Index into a TileArray; arrays are indexed in (row (y), column (x)) order
type TileIndex = (usize, usize);

impl Add<Direction> for TileIndex {
    type Output = TileIndex;

    fn add(self, direction: Direction) -> TileIndex {
        let (y, x) = self;
        match direction {
            Direction::Up => (y - 1, x),
            Direction::Down => (y + 1, x),
            Direction::Left => (y, x - 1),
            Direction::Right => (y, x + 1),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct GameState {
    tiles: TileArray,
    snake_head: TileIndex,
    snake_tail: TileIndex,
    snake_dir: Direction,
}

impl GameState {
    pub fn new() -> Self {
        let width = 80;
        let height = 50;
        let mut tiles = ndarray::Array::from_elem((height, width), Tile::Floor);

        // Build level wall
        for x in 0..width {
            tiles[(0, x)] = Tile::Wall;
            tiles[(height - 1, x)] = Tile::Wall;
        }
        for y in 0..height {
            tiles[(y, 0)] = Tile::Wall;
            tiles[(y, width - 1)] = Tile::Wall;
        }

        // Place snake
        tiles[(3, 3)] = Tile::Snake(Direction::Right);
        tiles[(3, 4)] = Tile::Snake(Direction::Right);
        tiles[(3, 5)] = Tile::Snake(Direction::Right);
        let snake_head = (3, 5);
        let snake_tail = (3, 3);
        let snake_dir = Direction::Right;

        let mut game_state = Self {
            tiles: tiles,
            snake_head: snake_head,
            snake_tail: snake_tail,
            snake_dir: snake_dir,
        };
        game_state.spawn_food();

        game_state
    }

    fn spawn_food(&mut self) {
        let (height, width) = self.tiles.dim();
        let mut rng = rand::thread_rng();
        let mut index = (0, 0);
        // FIXME This will hang if the snake fills the entire playing field
        loop {
            index.0 = rng.gen_range(1, height - 1);
            index.1 = rng.gen_range(1, width - 1);
            if self.tiles[index] == Tile::Floor {
                break;
            };
        }
        self.tiles[index] = Tile::Food;
    }

    pub fn tiles(&self) -> &TileArray {
        &self.tiles
    }

    pub fn set_snake_dir(&mut self, new_snake_dir: Direction) {
        self.snake_dir = new_snake_dir;
    }

    pub fn update(&mut self) {
        // Move snake
        let new_snake_head = self.snake_head + self.snake_dir;
        let mut eat_food = false;
        // Handle collision
        match self.tiles[new_snake_head] {
            Tile::Wall | Tile::Snake(_) => {
                // New head collides with wall or snake, so game over
                // FIXME Implement game over, just return for now
                return;
            }
            Tile::Food => {
                // New head collides with food, so eat the food
                eat_food = true;
            }
            Tile::Floor => {} // No collision
        }
        self.tiles[self.snake_head] = Tile::Snake(self.snake_dir);
        self.tiles[new_snake_head] = Tile::Snake(self.snake_dir);
        self.snake_head = new_snake_head;
        if eat_food {
            // FIXME Adjust score
            self.spawn_food();
        } else if let Tile::Snake(snake_tail_dir) = self.tiles[self.snake_tail] {
            self.tiles[self.snake_tail] = Tile::Floor;
            self.snake_tail = self.snake_tail + snake_tail_dir;
        } else {
            panic!("Expected Snake(_) on tile at position {:?}, found {:?} instead",
                   self.snake_tail,
                   self.tiles[self.snake_tail]);
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Tile {
    Floor,
    Wall,
    Food,
    // A Snake tile contains the direction that part of the snake is moving in (pointing towards
    // the head of the snake)
    Snake(Direction),
}
