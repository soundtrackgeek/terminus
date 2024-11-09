use crossterm::{
    cursor::{Hide, MoveTo, Show},
    style::{Color, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use rand::Rng;
use std::io::{stdout, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

const MATRIX_CHARS: &str = "ﾊﾐﾋｰｳｼﾅﾓﾆｻﾜﾂｵﾘｱﾎﾃﾏｹﾒｴｶｷﾑﾕﾗｾﾈｽﾀﾇﾍ";
const TRANSPARENCY: f32 = 0.2;

pub struct MatrixRain {
    width: u16,
    height: u16,
    drops: Vec<Drop>,
    running: Arc<AtomicBool>,
}

struct Drop {
    x: u16,
    y: i16,
    speed: u16,
    length: u16,
}

impl MatrixRain {
    pub fn new(width: u16, height: u16) -> Self {
        let mut drops = Vec::new();
        let mut rng = rand::thread_rng();

        for _ in 0..width / 3 {
            drops.push(Drop {
                x: rng.gen_range(0..width),
                y: rng.gen_range(-20..0),
                speed: rng.gen_range(1..4),
                length: rng.gen_range(5..20),
            });
        }

        Self {
            width,
            height,
            drops,
            running: Arc::new(AtomicBool::new(true)),
        }
    }

    pub fn start(&mut self) {
        let running = self.running.clone();
        std::thread::spawn(move || {
            while running.load(Ordering::Relaxed) {
                Self::draw_frame(&mut stdout(), &mut self.drops, self.width, self.height);
                std::thread::sleep(Duration::from_millis(50));
            }
        });
    }

    pub fn stop(&self) {
        self.running.store(false, Ordering::Relaxed);
    }

    fn draw_frame(stdout: &mut std::io::Stdout, drops: &mut Vec<Drop>, width: u16, height: u16) {
        let mut rng = rand::thread_rng();

        for drop in drops.iter_mut() {
            // Clear previous position with dark green
            if drop.y >= 0 {
                stdout.execute(MoveTo(drop.x, drop.y as u16)).unwrap();
                stdout
                    .execute(SetForegroundColor(Color::DarkGreen))
                    .unwrap();
                print!(" ");
            }

            // Update position
            drop.y += drop.speed as i16;

            // Draw new position with bright green
            if drop.y >= 0 && drop.y < height as i16 {
                stdout.execute(MoveTo(drop.x, drop.y as u16)).unwrap();
                stdout.execute(SetForegroundColor(Color::Green)).unwrap();
                print!(
                    "{}",
                    MATRIX_CHARS
                        .chars()
                        .nth(rng.gen_range(0..MATRIX_CHARS.len()))
                        .unwrap()
                );
            }

            // Reset if off screen
            if drop.y > height as i16 {
                drop.y = -drop.length as i16;
                drop.x = rng.gen_range(0..width);
            }
        }
        stdout.flush().unwrap();
    }
}
