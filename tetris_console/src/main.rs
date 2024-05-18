use std::io::{stdout, Write};

use getch_rs::{Getch, Key};
use rand::Rng;

#[derive(Clone, Copy, Debug)]
enum Block {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
    Empty,
}

struct Position {
    x: i32,
    y: i32,
}

struct Tetromino {
    shape: Block,
    position: Position,
    rotation: i32,
}

impl Tetromino {
    fn new(shape: Block) -> Tetromino {
        Tetromino {
            shape: shape,
            position: Position { x: 0, y: 0 },
            rotation: 0,
        }
    }

    fn rotate(&mut self) {
        self.rotation = (self.rotation + 1) % 4;
    }

    fn move_left(&mut self) {
        self.position.x -= 1;
    }

    fn move_right(&mut self) {
        self.position.x += 1;
    }

    fn move_down(&mut self) {
        self.position.y += 1;
    }
}

const WIDTH: usize = 10;
const HEIGHT: usize = 20;

struct Game {
    grid: [[Block; WIDTH]; HEIGHT],
    current_tetromino: Tetromino,
}

impl Game {
    fn new() -> Self {
        let mut game = Game {
            grid: [[Block::Empty; WIDTH]; HEIGHT],
            current_tetromino: Game::random_tetromino(),
        };
        game.place_tetromino();
        game
    }

    fn random_tetromino() -> Tetromino {
        let shapes = [
            Block::I,
            Block::O,
            Block::T,
            Block::S,
            Block::Z,
            Block::J,
            Block::L,
        ];
        let mut rng = rand::thread_rng();
        let shape = shapes[rng.gen_range(0..shapes.len())];
        Tetromino::new(shape)
    }

    fn print_grid(&self, stdout: &mut std::io::Stdout) {
        for row in &self.grid {
            for cell in row {
                let symbol = match cell {
                    Block::Empty => '.',
                    _ => '#',
                };
                write!(stdout, "{} ", symbol).unwrap();
            }
            writeln!(stdout).unwrap();
        }
        stdout.flush().unwrap();
    }

    fn place_tetromino(&mut self) {
        let pos = &self.current_tetromino.position;
        self.grid[pos.y as usize][pos.x as usize] = self.current_tetromino.shape;
    }

    fn clear_grid(&mut self) {
        self.grid = [[Block::Empty; WIDTH]; HEIGHT];
    }

    fn update(&mut self) {
        self.clear_grid();
        self.place_tetromino();
    }

    fn move_left(&mut self) {
        self.current_tetromino.move_left();
        self.update();
    }

    fn move_right(&mut self) {
        self.current_tetromino.move_right();
        self.update();
    }

    fn move_down(&mut self) {
        self.current_tetromino.move_down();
        self.update();
    }

    fn rotate(&mut self) {
        self.current_tetromino.rotate();
        self.update();
    }
}

fn clear_screen(stdout: &mut std::io::Stdout) {
    write!(stdout, "\x1B[2J\x1B[1;1H").unwrap();
    stdout.flush().unwrap();
}

fn reset_cursor(stdout: &mut std::io::Stdout) {
    write!(stdout, "\x1B[1;1H").unwrap();
    stdout.flush().unwrap();
}

fn main() {
    let mut game = Game::new();
    let getch = Getch::new();
    let mut stdout = stdout();

    clear_screen(&mut stdout);
    game.print_grid(&mut stdout);

    loop {
        let key = getch.getch().unwrap();
        match key {
            Key::Char('a') | Key::Left => game.move_left(),
            Key::Char('d') | Key::Right => game.move_right(),
            Key::Char('s') | Key::Down => game.move_down(),
            Key::Char('w') | Key::Up => game.rotate(),
            Key::Char('q') | Key::Esc => break,
            _ => {}
        }

        reset_cursor(&mut stdout);
        game.print_grid(&mut stdout);
    }

    println!("Game over!");
}
