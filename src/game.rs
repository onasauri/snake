use ndarray;
use rand;
use rand::Rng;

type TileArray = ndarray::Array2<Tile>;
// Index into a TileArray; arrays are indexed in (row (y), column (x)) order
type TileIndex = (usize, usize);

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
        game_state.toggle_walls();
        game_state.spawn_food();

        game_state
    }

    fn swap_tile(&mut self, i: TileIndex, tile1: Tile, tile2: Tile) {
        let tile = self.tiles[i];
        if tile == tile1 {
            self.tiles[i] = tile2;
        } else if tile == tile2 {
            self.tiles[i] = tile1;
        }
    }

    pub fn toggle_walls(&mut self) {
        // Build level wall
        let (w, h) = (self.level_width, self.level_height);
        for x in 0..w {
            self.swap_tile((0, x), Tile::Floor, Tile::Wall);
            self.swap_tile((h - 1, x), Tile::Floor, Tile::Wall);
        }
        for y in 1..h - 1 {
            self.swap_tile((y, 0), Tile::Floor, Tile::Wall);
            self.swap_tile((y, w - 1), Tile::Floor, Tile::Wall);
        }
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
            index = (rng.gen_range(0, self.level_height), rng.gen_range(0, self.level_width));
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

    fn add_dir_to_index(&self, (y, x): TileIndex, dir: Direction) -> TileIndex {
        match dir {
            Direction::Up => ((y + self.level_height - 1) % self.level_height, x),
            Direction::Down => ((y + 1) % self.level_height, x),
            Direction::Left => (y, (x + self.level_width - 1) % self.level_width),
            Direction::Right => (y, (x + 1) % self.level_width),
        }
    }

    pub fn update(&mut self, input: Option<Direction>) -> Result<(), String> {
        // Don't do anything if the snake is dead
        if !self.snake_alive {
            return Ok(());
        }

        // Handle input
        if let Some(new_snake_dir) = input {
            // Reversing direction would instantly crash the snake into itself, so don't allow it
            if new_snake_dir != self.snake_dir.reverse() {
                self.snake_dir = new_snake_dir;
            }
        }

        // Move snake
        let new_snake_head_idx = self.add_dir_to_index(self.snake_head_idx, self.snake_dir);
        let new_snake_tail_idx =
            self.add_dir_to_index(self.snake_tail_idx, self.get_snake_next(self.snake_tail_idx)?);
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
