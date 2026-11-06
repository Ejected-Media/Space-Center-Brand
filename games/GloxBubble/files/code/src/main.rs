use std::io::{self, Write};
use std::time::Duration;
// Simple grid-based CLI representation for Termux

#[derive(Clone, Copy, PartialEq)]
enum Entity {
    Empty,
    Player,    // 'B' (Blinky)
    Bubble,    // 'o'
    Enemy,     // 'E'
    Wall,      // '#'
}

const WIDTH: usize = 20;
const HEIGHT: usize = 12;

struct GameState {
    grid: [[Entity; WIDTH]; HEIGHT],
    player_x: usize,
    player_y: usize,
    score: u32,
}

impl GameState {
    fn new() -> Self {
        let mut grid = [[Entity::Empty; WIDTH]; HEIGHT];
        
        // Boundaries
        for x in 0..WIDTH {
            grid[0][x] = Entity::Wall;
            grid[HEIGHT - 1][x] = Entity::Wall;
        }
        for y in 0..HEIGHT {
            grid[y][0] = Entity::Wall;
            grid[y][WIDTH - 1] = Entity::Wall;
        }

        // Place some initial bubbles and an enemy
        grid[5][5] = Entity::Bubble;
        grid[5][10] = Entity::Bubble;
        grid[3][15] = Entity::Enemy;

        GameState {
            grid,
            player_x: 2,
            player_y: 2,
            score: 0,
        }
    }

    fn render(&self) {
        print!("\x1B[2J\x1B[1;1H"); // Clear screen
        println!("=== BUBBLE TROUBLE (TERMUX CLI) ===");
        println!("Score: {} | Controls: W/A/S/D to move, Q to quit", self.score);
        
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let ch = if x == self.player_x && y == self.player_y {
                    'B'
                } else {
                    match self.grid[y][x] {
                        Entity::Empty => '.',
                        Entity::Player => 'B',
                        Entity::Bubble => 'o',
                        Entity::Enemy => 'E',
                        Entity::Wall => '#',
                    }
                };
                print!("{} ", ch);
            }
            println!();
        }
    }
}

fn main() {
    let mut game = GameState::new();
    println!("Starting Bubble Trouble CLI prototype. Press Q to exit.");
    
    // Simple loop illustration for Termux terminal interaction
    loop {
        game.render();
        
        print!("Move (w/a/s/d): ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_ok() {
            let cmd = input.trim().to_lowercase();
            let (mut nx, mut ny) = (game.player_x, game.player_y);
            
            match cmd.as_str() {
                "w" => ny = ny.saturating_sub(1),
                "s" => ny = (ny + 1).min(HEIGHT - 1),
                "a" => nx = nx.saturating_sub(1),
                "d" => nx = (nx + 1).min(WIDTH - 1),
                "q" => break,
                _ => continue,
            }

            // Collision check
            match game.grid[ny][nx] {
                Entity::Wall => {} // Blocked
                Entity::Bubble => {
                    // Push bubble logic simulation
                    game.player_x = nx;
                    game.player_y = ny;
                    game.score += 100;
                }
                Entity::Enemy => {
                    println!("Ouch! Hit an enemy.");
                    break;
                }
                _ => {
                    game.player_x = nx;
                    game.player_y = ny;
                }
            }
        }
    }
}
