use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io::{self, Write},
    time::{Duration, Instant},
};

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
    game_over: bool,
}

impl GameState {
    fn new() -> Self {
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
                // Simple push / collect logic simulation
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
    }
}

fn main() -> Result<(), io::Error> {
    // Setup terminal raw mode for instant mobile key response
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let mut game = GameState::new();
    let tick_rate = Duration::from_millis(100);
    let mut last_tick = Instant::now();

    loop {
        // Render Viewport
        write!(stdout, "\x1B[2J\x1B[1;1H")?;
        writeln!(stdout, "=== GLOX-BUBBLE (TERMUX OS) ===")?;
        writeln!(stdout, "Score: {} | Use W/A/S/D to Move, Q to Quit", game.score)?;
        
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
            writeln!(stdout, "")?;
        }
        
        stdout.flush()?;

        if game.game_over {
            writeln!(stdout, "\nGAME OVER! Press any key to exit.")?;
            stdout.flush()?;
            let _ = event::read();
            break;
        }

        // Handle non-blocking keyboard input
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('w') | KeyCode::Char('W') => game.move_player(0, -1),
                    KeyCode::Char('s') | KeyCode::Char('S') => game.move_player(0, 1),
                    KeyCode::Char('a') | KeyCode::Char('A') => game.move_player(-1, 0),
                    KeyCode::Char('d') | KeyCode::Char('D') => game.move_player(1, 0),
                    KeyCode::Char('q') | KeyCode::Char('Q') => break,
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }

    // Restore terminal state on exit
    disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen)?;
    Ok(())
}
