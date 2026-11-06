use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use redb::{Database, TableDefinition};
use std::{
    io::{self, Write},
    time::{Duration, Instant},
};

const TABLE_SCORES: TableDefinition<&str, u32> = TableDefinition::new("glox_bubble_scores");

#[derive(Clone, Copy, PartialEq)]
enum Entity {
    Empty,
    Player,    // 'B' (Blinky)
    Bubble,    // 'o'
    Enemy,     // 'E'
    Wall,      // '#'
}

const WIDTH: usize = 22;
const HEIGHT: usize = 12;

struct GameState {
    grid: [[Entity; WIDTH]; HEIGHT],
    player_x: usize,
    player_y: usize,
    score: u32,
    high_score: u32,
    game_over: bool,
}

impl GameState {
    fn new(high_score: u32) -> Self {
        let mut grid = [[Entity::Empty; WIDTH]; HEIGHT];
        
        // Build perimeter walls
        for x in 0..WIDTH {
            grid[0][x] = Entity::Wall;
            grid[HEIGHT - 1][x] = Entity::Wall;
        }
        for y in 0..HEIGHT {
            grid[y][0] = Entity::Wall;
            grid[y][WIDTH - 1] = Entity::Wall;
        }

        // Place initial tactical hazards and blocks
        grid[3][5] = Entity::Bubble;
        grid[3][10] = Entity::Bubble;
        grid[6][15] = Entity::Bubble;
        grid[2][18] = Entity::Enemy;

        GameState {
            grid,
            player_x: 2,
            player_y: 2,
            score: 0,
            high_score,
            game_over: false,
        }
    }

    fn move_player(&mut self, dx: isize, dy: isize) {
        let nx = (self.player_x as isize + dx) as usize;
        let ny = (self.player_y as isize + dy) as usize;

        if nx >= WIDTH || ny >= HEIGHT {
            return;
        }

        match self.grid[ny][nx] {
            Entity::Wall => {} // Blocked by barrier
            Entity::Bubble => {
                self.score += 50;
                self.grid[ny][nx] = Entity::Empty;
                self.player_x = nx;
                self.player_y = ny;
            }
            Entity::Enemy => {
                self.game_over = true;
            }
            Entity::Empty => {
                self.player_x = nx;
                self.player_y = ny;
            }
        }

        if self.score > self.high_score {
            self.high_score = self.score;
        }
    }
}

fn load_high_score() -> u32 {
    let db = match Database::create("glox_bubble.redb") {
        Ok(db) => db,
        Err(_) => return 0,
    };
    let read_txn = match db.begin_read() {
        Ok(txn) => txn,
        Err(_) => return 0,
    };
    let table = match read_txn.open_table(TABLE_SCORES) {
        Ok(t) => t,
        Err(_) => return 0,
    };
    table.get("high_score").ok().flatten().map(|v| v.value()).unwrap_or(0)
}

fn save_high_score(score: u32) {
    if let Ok(db) = Database::create("glox_bubble.redb") {
        if let Ok(write_txn) = db.begin_write() {
            {
                if let Ok(mut table) = write_txn.open_table(TABLE_SCORES) {
                    let _ = table.insert("high_score", score);
                }
            }
            let _ = write_txn.commit();
        }
    }
}

fn main() -> Result<(), io::Error> {
    let initial_high = load_high_score();
    
    // Setup terminal raw mode for instant mobile key response in Termux
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let mut game = GameState::new(initial_high);
    let tick_rate = Duration::from_millis(100);
    let mut last_tick = Instant::now();

    loop {
        // Render Viewport
        write!(stdout, "\x1B[2J\x1B[1;1H")?;
        writeln!(stdout, "=== GLOX-BUBBLE (CEMENT BOARD eZINE) ===")?;
        writeln!(stdout, "Score: {} | High Score: {} | W/A/S/D to Move, Q to Quit", game.score, game.high_score)?;
        
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let ch = if x == game.player_x && y == game.player_y {
                    'B'
                } else {
                    match game.grid[y][x] {
                        Entity::Empty => '.',
                        Entity::Player => 'B',
                        Entity::Bubble => 'o',
                        Entity::Enemy => 'E',
                        Entity::Wall => '#',
                    }
                };
                write!(stdout, "{} ", ch)?;
            }
            writeln!(stdout)?;
        }
        
        stdout.flush()?;

        if game.game_over {
            save_high_score(game.high_score);
            writeln!(stdout, "\nCRASH! Final Score: {}. Press any key to exit.", game.score)?;
            stdout.flush()?;
            let _ = event::read();
            break;
        }

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('w') | KeyCode::Char('W') | KeyCode::Up => game.move_player(0, -1),
                    KeyCode::Char('s') | KeyCode::Char('S') | KeyCode::Down => game.move_player(0, 1),
                    KeyCode::Char('a') | KeyCode::Char('A') | KeyCode::Left => game.move_player(-1, 0),
                    KeyCode::Char('d') | KeyCode::Char('D') | KeyCode::Right => game.move_player(1, 0),
                    KeyCode::Char('q') | KeyCode::Char('Q') => {
                        save_high_score(game.high_score);
                        break;
                    }
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }

    disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen)?;
    Ok(())
}
