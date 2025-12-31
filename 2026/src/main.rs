use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::Duration;

const VERSION: &str = "2026.0.0";

const LOGO: &str = r#"
  ██╗  ██╗ █████╗ ██████╗ ██████╗ ██╗   ██╗
  ██║  ██║██╔══██╗██╔══██╗██╔══██╗╚██╗ ██╔╝
  ███████║███████║██████╔╝██████╔╝ ╚████╔╝ 
  ██╔══██║██╔══██║██╔═══╝ ██╔═══╝   ╚██╔╝  
  ██║  ██║██║  ██║██║     ██║        ██║   
  ╚═╝  ╚═╝╚═╝  ╚═╝╚═╝     ╚═╝        ╚═╝   
  ███╗   ██╗███████╗██╗    ██╗    ██╗   ██╗███████╗ █████╗ ██████╗ 
  ████╗  ██║██╔════╝██║    ██║    ╚██╗ ██╔╝██╔════╝██╔══██╗██╔══██╗
  ██╔██╗ ██║█████╗  ██║ █╗ ██║     ╚████╔╝ █████╗  ███████║██████╔╝
  ██║╚██╗██║██╔══╝  ██║███╗██║      ╚██╔╝  ██╔══╝  ██╔══██║██╔══██╗
  ██║ ╚████║███████╗╚███╔███╔╝       ██║   ███████╗██║  ██║██║  ██║
  ╚═╝  ╚═══╝╚══════╝ ╚══╝╚══╝        ╚═╝   ╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝
"#;

const YEAR_2026: &str = r#"
    ╔══════════════════════════════════════════════════════════╗
    ║   ██████╗  ██████╗ ██████╗  ██████╗                      ║
    ║   ╚════██╗██╔═████╗╚════██╗██╔════╝                      ║
    ║    █████╔╝██║██╔██║ █████╔╝███████╗                      ║
    ║   ██╔═══╝ ████╔╝██║██╔═══╝ ██╔═══██╗                     ║
    ║   ███████╗╚██████╔╝███████╗╚██████╔╝                     ║
    ║   ╚══════╝ ╚═════╝ ╚══════╝ ╚═════╝                      ║
    ╚══════════════════════════════════════════════════════════╝
"#;

const HELP_TEXT: &str = r#"
  Available Commands:
  ─────────────────────────────────────────
  /help     - Show this help message
  /new      - Start a new celebration
  /wish     - Send a new year wish
  /countdown- Show countdown animation
  /clear    - Clear the screen
  /version  - Show version info
  /exit     - Exit the program
  ─────────────────────────────────────────
"#;

fn main() -> std::io::Result<()> {
    let mut stdout = stdout();
    execute!(stdout, Clear(ClearType::All), MoveTo(0, 0), Hide)?;
    print_loading_animation(&mut stdout)?;
    execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;
    print_startup_sequence(&mut stdout)?;
    print_logo(&mut stdout)?;
    print_version_info(&mut stdout)?;
    print_init_sequence(&mut stdout)?;
    execute!(stdout, Show)?;
    run_session(&mut stdout)?;
    Ok(())
}

fn print_loading_animation(stdout: &mut std::io::Stdout) -> std::io::Result<()> {
    let frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    let msg = "Starting hny...";
    println!();
    println!();
    for _ in 0..3 {
        for frame in frames.iter() {
            execute!(
                stdout,
                MoveTo(2, 2),
                SetForegroundColor(Color::Cyan),
                Print(format!("  {} {}", frame, msg)),
                ResetColor
            )?;
            stdout.flush()?;
            sleep(Duration::from_millis(100));
        }
    }
    execute!(
        stdout,
        MoveTo(2, 2),
        SetForegroundColor(Color::Green),
        Print("  ✓ Ready!          "),
        ResetColor
    )?;
    stdout.flush()?;
    sleep(Duration::from_millis(300));
    Ok(())
}

fn run_session(stdout: &mut std::io::Stdout) -> std::io::Result<()> {
    println!();
    print_colored_line(stdout, "  Type /help for available commands, or just say something!", Color::DarkGrey)?;
    println!();
    loop {
        execute!(
            stdout,
            SetForegroundColor(Color::Magenta),
            Print("  hny> "),
            ResetColor
        )?;
        stdout.flush()?;
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let input = input.trim();
        if input.is_empty() {
            continue;
        }
        match input.to_lowercase().as_str() {
            "/help" | "/h" => {
                print_help(stdout)?;
            }
            "/new" | "/n" => {
                execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;
                print_newyear_generation(stdout)?;
                print_year(stdout)?;
                print_final_message(stdout)?;
            }
            "/wish" | "/w" => {
                print_wish_prompt(stdout)?;
            }
            "/countdown" | "/c" => {
                print_countdown(stdout)?;
            }
            "/clear" => {
                execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;
                print_logo(stdout)?;
            }
            "/version" | "/v" => {
                print_version_detail(stdout)?;
            }
            "/exit" | "/quit" | "/q" => {
                print_goodbye(stdout)?;
                break;
            }
            _ if input.starts_with('/') => {
                execute!(
                    stdout,
                    SetForegroundColor(Color::Red),
                    Print(format!("  Unknown command: {}\n", input)),
                    Print("  Type /help for available commands.\n\n"),
                    ResetColor
                )?;
            }
            _ => {
                print_response(stdout, input)?;
            }
        }
    }
    Ok(())
}

fn print_help(stdout: &mut std::io::Stdout) -> std::io::Result<()> {
    execute!(stdout, SetForegroundColor(Color::Cyan))?;
    for line in HELP_TEXT.lines() {
        execute!(stdout, Print(line), Print("\n"))?;
    }
    execute!(stdout, ResetColor, Print("\n"))?;
    Ok(())
}

fn print_wish_prompt(stdout: &mut std::io::Stdout) -> std::io::Result<()> {
    execute!(
        stdout,
        SetForegroundColor(Color::Yellow),
        Print("\n  Enter your wish for 2026: "),
        ResetColor
    )?;
    stdout.flush()?;
    let mut wish = String::new();
    std::io::stdin().read_line(&mut wish)?;
    let wish = wish.trim();
    if !wish.is_empty() {
        println!();
        execute!(stdout, SetForegroundColor(Color::Green))?;
        typewriter(stdout, "  ✨ Your wish has been sent to the universe! ✨\n", 30)?;
        execute!(
            stdout,
            SetForegroundColor(Color::Cyan),
            Print(format!("  \"{}\" - saved to ~/.wishes/2026\n\n", wish)),
            ResetColor
        )?;
    }
    Ok(())
}

fn print_countdown(stdout: &mut std::io::Stdout) -> std::io::Result<()> {
    println!();
    let countdown = ["5", "4", "3", "2", "1"];
    for num in countdown.iter() {
        execute!(
            stdout,
            SetForegroundColor(Color::Yellow),
            Print(format!("  {} ", num)),
            ResetColor
        )?;
        stdout.flush()?;
        sleep(Duration::from_millis(800));
    }
    println!();
    execute!(
        stdout,
        SetForegroundColor(Color::Rgb { r: 255, g: 215, b: 0 }),
        Print("\n  🎆 HAPPY NEW YEAR 2026! 🎆\n\n"),
        ResetColor
    )?;
    Ok(())
}

fn print_version_detail(stdout: &mut std::io::Stdout) -> std::io::Result<()> {
    let info = format!(r#"
  ┌─────────────────────────────────────┐
  │  hny v{}                      │
  │  Happy New Year CLI                 │
  │  Claude Code Tribute Edition        │
  │                                     │
  │  Author: rxxuzi                     │
  │  Built with Rust + crossterm        │
  └─────────────────────────────────────┘
"#, VERSION);
    execute!(
        stdout,
        SetForegroundColor(Color::Cyan),
        Print(info),
        Print("\n"),
        ResetColor
    )?;
    Ok(())
}

fn print_response(stdout: &mut std::io::Stdout, input: &str) -> std::io::Result<()> {
    let responses = [
        "Happy New Year to you too! 🎉",
        "May 2026 bring you joy and success!",
        "Wishing you all the best in the new year!",
        "Here's to an amazing 2026! ✨",
        "Cheers to new beginnings! 🥂",
    ];
    let idx = input.len() % responses.len();
    println!();
    execute!(
        stdout,
        SetForegroundColor(Color::Green),
        Print("  "),
    )?;
    typewriter(stdout, responses[idx], 25)?;
    execute!(stdout, Print("\n\n"), ResetColor)?;
    Ok(())
}

fn print_goodbye(stdout: &mut std::io::Stdout) -> std::io::Result<()> {
    println!();
    execute!(stdout, SetForegroundColor(Color::Magenta))?;
    typewriter(stdout, "  See you next year! Happy 2026! 🎊\n\n", 30)?;
    execute!(stdout, ResetColor)?;
    Ok(())
}

fn typewriter(stdout: &mut std::io::Stdout, text: &str, delay_ms: u64) -> std::io::Result<()> {
    for c in text.chars() {
        execute!(stdout, Print(c))?;
        stdout.flush()?;
        sleep(Duration::from_millis(delay_ms));
    }
    Ok(())
}

fn print_colored_line(stdout: &mut std::io::Stdout, text: &str, color: Color) -> std::io::Result<()> {
    execute!(
        stdout,
        SetForegroundColor(color),
        Print(text),
        Print("\n"),
        ResetColor
    )
}

fn print_startup_sequence(stdout: &mut std::io::Stdout) -> std::io::Result<()> {
    let messages = [
        ("╭─────────────────────────────────────────────────────────────╮", Color::DarkCyan),
        ("│  Welcome to Happy New Year CLI                             │", Color::Cyan),
        ("│  A Claude Code Tribute by rxxuzi                           │", Color::DarkCyan),
        ("╰─────────────────────────────────────────────────────────────╯", Color::DarkCyan),
    ];
    println!();
    for (msg, color) in messages.iter() {
        print_colored_line(stdout, msg, *color)?;
        sleep(Duration::from_millis(100));
    }
    println!();
    sleep(Duration::from_millis(500));
    Ok(())
}

fn print_logo(stdout: &mut std::io::Stdout) -> std::io::Result<()> {
    let gradient_colors = [
        Color::Rgb { r: 255, g: 100, b: 100 },
        Color::Rgb { r: 255, g: 150, b: 100 },
        Color::Rgb { r: 255, g: 200, b: 100 },
        Color::Rgb { r: 150, g: 255, b: 150 },
        Color::Rgb { r: 100, g: 200, b: 255 },
        Color::Rgb { r: 150, g: 150, b: 255 },
        Color::Rgb { r: 200, g: 100, b: 255 },
        Color::Rgb { r: 255, g: 100, b: 200 },
        Color::Rgb { r: 255, g: 150, b: 150 },
        Color::Rgb { r: 255, g: 200, b: 150 },
        Color::Rgb { r: 200, g: 255, b: 200 },
        Color::Rgb { r: 150, g: 220, b: 255 },
    ];
    for (i, line) in LOGO.lines().enumerate() {
        let color = gradient_colors[i % gradient_colors.len()];
        execute!(stdout, SetForegroundColor(color), Print(line), Print("\n"))?;
        stdout.flush()?;
        sleep(Duration::from_millis(80));
    }
    execute!(stdout, ResetColor)?;
    println!();
    sleep(Duration::from_millis(300));
    Ok(())
}

fn print_version_info(stdout: &mut std::io::Stdout) -> std::io::Result<()> {
    execute!(
        stdout,
        SetForegroundColor(Color::DarkGrey),
        Print(format!("  hny v{} ", VERSION)),
        SetForegroundColor(Color::Yellow),
        Print("* "),
        SetForegroundColor(Color::DarkGrey),
        Print("Happy New Year Edition"),
        Print("\n"),
        ResetColor
    )?;
    println!();
    sleep(Duration::from_millis(500));
    Ok(())
}

fn print_init_sequence(stdout: &mut std::io::Stdout) -> std::io::Result<()> {
    let steps = [
        ("› Initializing new year celebration...", 300),
        ("› Loading memories from 2025...", 400),
        ("› Generating hopes and dreams for 2026...", 500),
        ("› Preparing session...", 200),
    ];
    for (step, delay) in steps.iter() {
        execute!(stdout, SetForegroundColor(Color::Cyan))?;
        typewriter(stdout, step, 15)?;
        execute!(
            stdout,
            SetForegroundColor(Color::Green),
            Print(" done\n"),
            ResetColor
        )?;
        sleep(Duration::from_millis(*delay));
    }
    println!();
    sleep(Duration::from_millis(500));
    Ok(())
}

fn print_newyear_generation(stdout: &mut std::io::Stdout) -> std::io::Result<()> {
    println!();
    execute!(
        stdout,
        SetForegroundColor(Color::Magenta),
        Print("  Generating a Happy New Year greeting...\n"),
        ResetColor
    )?;
    let width = 40;
    execute!(stdout, Print("  ["))?;
    for i in 0..width {
        let progress = (i as f32 / width as f32 * 100.0) as u8;
        let color = Color::Rgb {
            r: 255 - progress * 2,
            g: progress * 2,
            b: 100,
        };
        execute!(stdout, SetForegroundColor(color), Print("#"))?;
        stdout.flush()?;
        sleep(Duration::from_millis(30));
    }
    execute!(
        stdout,
        ResetColor,
        Print("] "),
        SetForegroundColor(Color::Green),
        Print("100%\n"),
        ResetColor
    )?;
    println!();
    sleep(Duration::from_millis(500));
    execute!(stdout, SetForegroundColor(Color::Yellow))?;
    typewriter(stdout, "  2025 has been saved in your memories at ~/memories/2025.hny\n", 20)?;
    execute!(stdout, ResetColor)?;
    sleep(Duration::from_millis(300));
    execute!(stdout, SetForegroundColor(Color::Cyan))?;
    typewriter(stdout, "  New hopes saved to: ~/.wishes/2026\n", 20)?;
    execute!(stdout, ResetColor)?;
    println!();
    sleep(Duration::from_millis(500));
    Ok(())
}

fn print_year(stdout: &mut std::io::Stdout) -> std::io::Result<()> {
    let golden = Color::Rgb { r: 255, g: 215, b: 0 };
    for line in YEAR_2026.lines() {
        execute!(
            stdout,
            SetForegroundColor(golden),
            Print(line),
            Print("\n")
        )?;
        stdout.flush()?;
        sleep(Duration::from_millis(100));
    }
    execute!(stdout, ResetColor)?;
    println!();
    sleep(Duration::from_millis(500));
    Ok(())
}

fn print_final_message(stdout: &mut std::io::Stdout) -> std::io::Result<()> {
    let messages = [
        ("  +============================================================+", Color::Rgb { r: 255, g: 100, b: 100 }),
        ("  |                                                            |", Color::Rgb { r: 255, g: 150, b: 100 }),
        ("  |      *** Happy New Year 2026! ***                          |", Color::Rgb { r: 255, g: 200, b: 0 }),
        ("  |                                                            |", Color::Rgb { r: 150, g: 255, b: 150 }),
        ("  |   May your year be filled with joy, success, and code!     |", Color::Rgb { r: 100, g: 200, b: 255 }),
        ("  |                                                            |", Color::Rgb { r: 150, g: 150, b: 255 }),
        ("  |   Kotoshi mo yoroshiku onegaishimasu!                      |", Color::Rgb { r: 255, g: 150, b: 200 }),
        ("  |                                                            |", Color::Rgb { r: 200, g: 100, b: 255 }),
        ("  +============================================================+", Color::Rgb { r: 255, g: 100, b: 150 }),
    ];
    for (msg, color) in messages.iter() {
        execute!(
            stdout,
            SetForegroundColor(*color),
            Print(msg),
            Print("\n"),
            ResetColor
        )?;
        sleep(Duration::from_millis(150));
    }
    println!();
    execute!(
        stdout,
        SetForegroundColor(Color::DarkGrey),
        Print("  "),
    )?;
    typewriter(stdout, "Made with <3 by rxxuzi | Claude Code Tribute Edition", 30)?;
    execute!(stdout, Print("\n"), ResetColor)?;
    println!();
    sleep(Duration::from_millis(1000));
    execute!(
        stdout,
        SetForegroundColor(Color::Green),
        Print("  Press Enter to exit...\n"),
        ResetColor
    )?;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(())
}