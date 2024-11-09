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
    protected_areas: Arc<parking_lot::Mutex<Vec<(u16, u16, u16, u16)>>>, // (x1, y1, x2, y2)
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
            protected_areas: Arc::new(parking_lot::Mutex::new(Vec::new())),
        }
    }

    pub fn protect_area(&self, x1: u16, y1: u16, x2: u16, y2: u16) {
        self.protected_areas.lock().push((x1, y1, x2, y2));
    }

    fn is_position_protected(x: u16, y: u16, protected_areas: &[(u16, u16, u16, u16)]) -> bool {
        protected_areas
            .iter()
            .any(|(x1, y1, x2, y2)| x >= *x1 && x <= *x2 && y >= *y1 && y <= *y2)
    }

    fn draw_frame(
        stdout: &mut std::io::Stdout,
        drops: &mut Vec<Drop>,
        width: u16,
        height: u16,
        protected_areas: &[(u16, u16, u16, u16)],
    ) {
        let mut rng = rand::thread_rng();
        let matrix_chars: Vec<char> = MATRIX_CHARS.chars().collect();

        for drop in drops.iter_mut() {
            let x = drop.x;
            let y = drop.y;

            // Skip drawing if position is protected
            if y >= 0 && Self::is_position_protected(x, y as u16, protected_areas) {
                drop.y += drop.speed as i16;
                continue;
            }

            // Clear previous position
            if y >= 0 && !Self::is_position_protected(x, y as u16, protected_areas) {
                stdout.execute(MoveTo(x, y as u16)).unwrap();
                stdout
                    .execute(SetForegroundColor(Color::DarkGreen))
                    .unwrap();
                print!(" ");
            }

            // Update position
            drop.y += drop.speed as i16;

            // Draw new position
            if y >= 0
                && y < height as i16
                && !Self::is_position_protected(x, y as u16, protected_areas)
            {
                stdout.execute(MoveTo(x, y as u16)).unwrap();
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

    pub fn start(&self) {
        let running = self.running.clone();
        let drops = self.drops.clone();
        let protected_areas = self.protected_areas.clone();
        let width = self.width;
        let height = self.height;

        std::thread::spawn(move || {
            while running.load(Ordering::Relaxed) {
                Self::draw_frame(
                    &mut stdout(),
                    &mut drops.lock(),
                    width,
                    height,
                    &protected_areas.lock(),
                );
                std::thread::sleep(Duration::from_millis(50));
            }
        });
    }

    pub fn stop(&self) {
        self.running.store(false, Ordering::Relaxed);
    }
}
