use crossterm::{
    cursor::MoveTo,
    style::{Color, SetForegroundColor},
    ExecutableCommand,
};
use rand::Rng;
use std::io::{stdout, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

const MATRIX_CHARS: &str = "ﾊﾐﾋｰｳｼﾅﾓﾆｻﾜﾂｵﾘｱﾎﾃﾏｹﾒｴｶｷﾑﾕﾗｾﾈｽﾀﾇﾍ";

pub struct MatrixRain {
    width: u16,
    height: u16,
    drops: Arc<parking_lot::Mutex<Vec<Drop>>>,
    running: Arc<AtomicBool>,
}

struct Drop {
    x: u16,
    y: i16, // Changed to i16 to allow negative values
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
                y: -(rng.gen_range(5..20) as i16), // Negative starting position
                speed: rng.gen_range(1..4),
                length: rng.gen_range(5..20),
            });
        }

        Self {
            width,
            height,
            drops: Arc::new(parking_lot::Mutex::new(drops)),
            running: Arc::new(AtomicBool::new(true)),
        }
    }

    pub fn start(&self) {
        let running = self.running.clone();
        let drops = self.drops.clone();
        let width = self.width;
        let height = self.height;

        std::thread::spawn(move || {
            while running.load(Ordering::Relaxed) {
                Self::draw_frame(&mut stdout(), &mut drops.lock(), width, height);
                std::thread::sleep(Duration::from_millis(50));
            }
        });
    }

    pub fn stop(&self) {
        self.running.store(false, Ordering::Relaxed);
    }

    fn draw_frame(stdout: &mut std::io::Stdout, drops: &mut Vec<Drop>, width: u16, height: u16) {
        let mut rng = rand::thread_rng();
        let matrix_chars: Vec<char> = MATRIX_CHARS.chars().collect();

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
                let char_idx = rng.gen_range(0..matrix_chars.len());
                print!("{}", matrix_chars[char_idx]);
            }

            // Reset if off screen
            if drop.y >= height as i16 {
                drop.y = -(drop.length as i16);
                drop.x = rng.gen_range(0..width);
            }
        }
        stdout.flush().unwrap();
    }
}
