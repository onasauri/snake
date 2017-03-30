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

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn reverse(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq)]
pub enum Tile {
    Floor,
    Wall,
    Food,
    // Snake contains the optional directions towards the previous and next snake segments.  The
    // tail has only a next direction (so a Snake(None, Some(_))) while the head has only a
    // previous direction (so a Snake(Some(_), None)); all other segments have both defined.
    Snake(Option<Direction>, Option<Direction>),
}

#[derive(Serialize, Deserialize)]
pub struct GameState {
    level_width: usize,
    level_height: usize,
    tiles: TileArray,
    snake_head_idx: TileIndex,
    snake_tail_idx: TileIndex,
    snake_dir: Direction,
    snake_alive: bool,
    score: u32,
    highscore: u32,
}

impl GameState {
    pub fn new(level_width: usize, level_height: usize, highscore: u32) -> Self {
        let mut tiles = ndarray::Array::from_elem((level_height, level_width), Tile::Floor);

        // Build level wall
        for x in 0..level_width {
            tiles[(0, x)] = Tile::Wall;
            tiles[(level_height - 1, x)] = Tile::Wall;
        }
        for y in 0..level_height {
            tiles[(y, 0)] = Tile::Wall;
            tiles[(y, level_width - 1)] = Tile::Wall;
        }

        // Place snake
        tiles[(3, 3)] = Tile::Snake(None, Some(Direction::Right));
        tiles[(3, 4)] = Tile::Snake(Some(Direction::Left), Some(Direction::Right));
        tiles[(3, 5)] = Tile::Snake(Some(Direction::Left), None);
        let snake_head_idx = (3, 5);
        let snake_tail_idx = (3, 3);
        let snake_dir = Direction::Right;
        let snake_alive = true;
        let score = 0;

        let mut game_state = GameState {
            level_width: level_width,
            level_height: level_height,
            tiles: tiles,
            snake_head_idx: snake_head_idx,
            snake_tail_idx: snake_tail_idx,
            snake_dir: snake_dir,
            snake_alive: snake_alive,
            score: score,
            highscore: highscore,
        };
        game_state.spawn_food();

        game_state
    }

    pub fn reset(&mut self) {
        *self = GameState::new(self.level_width, self.level_height, self.highscore);
    }

    pub fn level_size(&self) -> (usize, usize) {
        (self.level_width, self.level_height)
    }

    pub fn snake_alive(&self) -> bool {
        self.snake_alive
    }

    fn spawn_food(&mut self) {
        let mut rng = rand::thread_rng();
        let mut index;
        // FIXME This will hang if the snake fills the entire playing field
        loop {
            index = (rng.gen_range(1, self.level_height - 1),
                     rng.gen_range(1, self.level_width - 1));
            if self.tiles[index] == Tile::Floor {
                break;
            };
        }
        self.tiles[index] = Tile::Food;
    }

    pub fn tiles(&self) -> &TileArray {
        &self.tiles
    }

    fn get_snake_prev(&self, index: TileIndex) -> Result<Direction, String> {
        if let Tile::Snake(Some(prev), _) = self.tiles[index] {
            Ok(prev)
        } else {
            Err(format!("Expected Snake(Some(_), _) on tile at {:?}, but found {:?}",
                        index,
                        self.tiles[index]))
        }
    }

    fn get_snake_next(&self, index: TileIndex) -> Result<Direction, String> {
        if let Tile::Snake(_, Some(next)) = self.tiles[index] {
            Ok(next)
        } else {
            Err(format!("Expected Snake(_, Some(_)) on tile at {:?}, but found {:?}",
                        index,
                        self.tiles[index]))
        }
    }

    pub fn update(&mut self, input: Option<Direction>) -> Result<(), String> {
        // Don't do anything if the snake is dead
        if !self.snake_alive {
            return Ok(());
        }

        // Handle input
        if let Some(new_snake_dir) = input {
            self.snake_dir = new_snake_dir;
        }

        // Move snake
        let new_snake_head_idx = self.snake_head_idx + self.snake_dir;
        let new_snake_tail_idx = self.snake_tail_idx + self.get_snake_next(self.snake_tail_idx)?;
        let mut eat_food = false;
        // Check for collision
        match self.tiles[new_snake_head_idx] {
            Tile::Wall | Tile::Snake(..) => {
                // New head collides with wall or snake, so game over
                self.snake_alive = false;
                println!("Game over!");
                println!("Your score: {}", self.score);
                if self.score > self.highscore {
                    println!("*** New highscore! ***");
                    self.highscore = self.score;
                } else {
                    println!("Highscore: {}", self.highscore);
                }
                return Ok(());
            }
            Tile::Food => {
                // New head collides with food, so eat the food
                eat_food = true;
            }
            Tile::Floor => {} // No collision
        }
        // Move snake head
        self.tiles[self.snake_head_idx] =
            Tile::Snake(Some(self.get_snake_prev(self.snake_head_idx)?),
                        Some(self.snake_dir));
        self.tiles[new_snake_head_idx] = Tile::Snake(Some(self.snake_dir.reverse()), None);
        self.snake_head_idx = new_snake_head_idx;
        // Spawn new food or move snake tail
        if eat_food {
            self.score += 10;
            self.spawn_food();
        } else {
            self.tiles[self.snake_tail_idx] = Tile::Floor;
            self.tiles[new_snake_tail_idx] =
                Tile::Snake(None, Some(self.get_snake_next(new_snake_tail_idx)?));
            self.snake_tail_idx = new_snake_tail_idx;
        }

        Ok(())
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new(40, 30, 0)
    }
}
