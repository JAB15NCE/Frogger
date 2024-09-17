use clap::{App, Arg};
use crossterm::{
    cursor,
    event::{read, Event, KeyCode, KeyEvent},
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal,
};
use rand::Rng;
use std::io::{stdout, Write};
use std::{thread, time};

const WIDTH: u16 = 20;
const HEIGHT: u16 = 10;
const FROG_CHAR: char = '0';
const MAX_LIVES: u8 = 3;
const NUM_OBSTACLES: usize = 5;
const FRAME_RATE: u64 = 100; // Milliseconds

struct Frog {
    x: u16,
    y: u16,
    lives: u8,
}

impl Frog {
    fn new(x: u16, y: u16, lives: u8) -> Self {
        Frog { x, y, lives }
    }

    fn move_up(&mut self) {
        if self.y > 0 {
            self.y -= 1;
        }
    }

    fn move_down(&mut self) {
        if self.y < HEIGHT - 1 {
            self.y += 1;
        }
    }

    fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    fn move_right(&mut self) {
        if self.x < WIDTH - 1 {
            self.x += 1;
        }
    }
}

struct Obstacle {
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    speed: i16,
}

impl Obstacle {
    fn new(x: u16, y: u16, width: u16, height: u16, speed: i16) -> Self {
        Obstacle {
            x,
            y,
            width,
            height,
            speed,
        }
    }

    fn draw(&self) {
        execute!(
            stdout(),
            cursor::MoveTo(self.x, self.y),
            SetBackgroundColor(Color::Red),
            SetForegroundColor(Color::White),
            Print("#"),
            ResetColor
        )
        .unwrap();
    }

    fn clear(&self) {
        execute!(
            stdout(),
            cursor::MoveTo(self.x, self.y),
            Print(" "),
        )
        .unwrap();
    }

    fn r#move(&mut self) {
        self.x = ((self.x as i32) + self.speed as i32).rem_euclid(WIDTH as i32) as u16;
    }
}

fn generate_obstacles() -> Vec<Obstacle> {
    let mut obstacles = Vec::new();
    let mut rng = rand::thread_rng();

    for _ in 0..NUM_OBSTACLES {
        let x = rng.gen_range(0..WIDTH);
        let y = rng.gen_range(1..HEIGHT - 1);
        let width = rng.gen_range(1..4);
        let height = 1;
        let speed = rng.gen_range(-2..=2);
        obstacles.push(Obstacle::new(x, y, width, height, speed));
    }

    obstacles
}

fn draw_frog(frog: &Frog) {
    execute!(
        stdout(),
        cursor::MoveTo(frog.x, frog.y),
        Print(FROG_CHAR),
    )
    .unwrap();
}

fn clear_frog(frog: &Frog) {
    execute!(
        stdout(),
        cursor::MoveTo(frog.x, frog.y),
        Print(" "),
    )
    .unwrap();
}

fn draw_obstacles(obstacles: &[Obstacle]) {
    for obstacle in obstacles {
        obstacle.draw();
    }
}

fn clear_obstacles(obstacles: &[Obstacle]) {
    for obstacle in obstacles {
        obstacle.clear();
    }
}

fn check_collision(frog: &Frog, obstacles: &[Obstacle]) -> bool {
    for obstacle in obstacles {
        // Check if frog's position overlaps with any obstacle
        if frog.x >= obstacle.x
            && frog.x < obstacle.x + obstacle.width
            && frog.y >= obstacle.y
            && frog.y < obstacle.y + obstacle.height
        {
            return true; // Collision detected
        }
    }
    false // No collision detected
}

fn handle_collision(frog: &mut Frog, obstacles: &[Obstacle]) {
    if check_collision(frog, obstacles) {
        // Decrease frog's lives
        frog.lives -= 1;
        // Reset frog's position
        frog.x = WIDTH / 2;
        frog.y = HEIGHT - 1;
    }
}

fn main() {
    let _matches = App::new("Frogger")
        .arg(
            Arg::with_name("difficulty")
                .short('d')
                .long("difficulty")
                .takes_value(true)
                .possible_values(&["easy", "medium", "hard"])
                .default_value("medium")
                .help("Sets the difficulty level: easy, medium, hard"),
        )
        .get_matches();

    let mut frog = Frog::new(WIDTH / 2, HEIGHT - 1, MAX_LIVES);
    let mut obstacles = generate_obstacles();

    execute!(
        stdout(),
        terminal::Clear(terminal::ClearType::All),
        cursor::Hide
    )
    .unwrap();

    loop {
        draw_frog(&frog);
        draw_obstacles(&obstacles);

        if let Ok(Event::Key(KeyEvent { code, .. })) = read() {
            match code {
                KeyCode::Char('w') | KeyCode::Up => frog.move_up(),
                KeyCode::Char('s') | KeyCode::Down => frog.move_down(),
                KeyCode::Char('a') | KeyCode::Left => frog.move_left(),
                KeyCode::Char('d') | KeyCode::Right => frog.move_right(),
                KeyCode::Char('q') => break,
                _ => {}
            }
        }

        clear_frog(&frog);
        clear_obstacles(&obstacles);

        for obstacle in &mut obstacles {
            obstacle.r#move();
        }

        handle_collision(&mut frog, &obstacles);

        thread::sleep(time::Duration::from_millis(FRAME_RATE));
    }

    execute!(
        stdout(),
        terminal::Clear(terminal::ClearType::All),
        cursor::Show
    )
    .unwrap();
}

