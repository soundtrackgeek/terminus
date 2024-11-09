use chrono::Local;
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    style::{Color, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType, SetTitle},
    ExecutableCommand,
};
use std::io::{self, stdout, Write};
use std::{thread, time::Duration};

use crate::matrix::MatrixRain;

const LOGO: &str = r#"
╔══════════════════════════════════════════════════════════════════════╗
║  ████████╗███████╗██████╗ ███╗   ███╗██╗███╗   ██╗██╗   ██╗███████╗  ║
║  ╚══██╔══╝██╔════╝██╔══██╗████╗ ████║██║████╗  ██║██║   ██║██╔════╝  ║
║     ██║   █████╗  ██████╔╝██╔████╔██║██║██╔██╗ ██║██║   ██║███████╗  ║
║     ██║   ██╔══╝  ██╔══██╗██║╚██╔╝██║██║██║╚██╗██║██║   ██║╚════██║  ║
║     ██║   ███████╗██║  ██║██║ ╚═╝ ██║██║██║ ╚████║╚██████╔╝███████║  ║
║     ╚═╝   ╚══════╝╚═╝  ╚═╝╚═╝     ╚═╝╚═╝╚═╝  ╚═══╝ ╚═════╝ ╚══════╝  ║
║                                                                      ║
║                  ADVANCED AI INTERFACE SYSTEM v2.1                   ║
╚══════════════════════════════════════════════════════════════════════╝"#;

fn type_effect(text: &str, delay: u64) {
    for c in text.chars() {
        print!("{}", c);
        stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(delay));
    }
    println!();
}

pub fn boot_sequence() {
    let mut stdout = stdout();
    stdout.execute(SetTitle("TERMINUS")).unwrap();
    stdout.execute(SetBackgroundColor(Color::Black)).unwrap();
    stdout.execute(SetForegroundColor(Color::Green)).unwrap();
    stdout.execute(Clear(ClearType::All)).unwrap();
    stdout.execute(Hide).unwrap();

    // Start Matrix animation
    let (width, height) = crossterm::terminal::size().unwrap();
    let matrix = MatrixRain::new(width, height);
    matrix.start();

    println!("{}", LOGO);
    thread::sleep(Duration::from_secs(1));

    let messages = [
        "INITIALIZING TERMINUS CORE SYSTEMS...",
        &format!("SYSTEM TIME: {}", Local::now().format("%Y-%m-%d %H:%M:%S")),
        "LOADING NEURAL ARCHITECTURE...............[OK]",
        "INITIALIZING LANGUAGE PROCESSORS..........[OK]",
        "ESTABLISHING QUANTUM PROTOCOLS............[OK]",
        "CALIBRATING AI RESPONSE MATRIX...........[OK]",
        "ACTIVATING SECURITY PROTOCOLS............[OK]",
        "SYNCHRONIZING NEURAL NETWORKS............[OK]",
        "\nTERMINUS READY - AWAITING INPUT SEQUENCE",
        "\n=======================================",
    ];

    for msg in messages.iter() {
        type_effect(msg, 30);
        thread::sleep(Duration::from_millis(500));
    }

    // Stop Matrix animation and show cursor
    matrix.stop();
    stdout.execute(Show).unwrap();

    // Make cursor blink
    stdout
        .execute(MoveTo(0, height - 1))
        .unwrap()
        .execute(SetForegroundColor(Color::Green))
        .unwrap();
    print!("▋");
    stdout.flush().unwrap();
}

pub fn show_menu() -> io::Result<String> {
    let mut stdout = stdout();
    stdout.execute(SetForegroundColor(Color::Green)).unwrap();

    println!("\nTERMINUS COMMAND INTERFACE");
    println!("------------------------");
    println!("1. Enter prompt");
    println!("2. Select model");
    println!("3. Set system message");
    println!("4. Show system message");
    println!("5. Add memory entry");
    println!("6. Show memory");
    println!("7. Toggle memory usage");
    println!("8. Edit memory");
    println!("9. Exit");
    println!("\nEnter your choice (1-9): ");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice)?;
    Ok(choice.trim().to_string())
}
