use chrono::Local;
use figlet_rs::FIGfont;
use std::{io, thread, time::Duration};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Paragraph},
    Terminal,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

fn main() -> Result<(), io::Error> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_app(&mut terminal);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = result {
        eprintln!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    // Load FIGfont for large text rendering
    let standard_font = match FIGfont::standard() {
        Ok(font) => font,
        Err(e) => {
            eprintln!("Failed to load standard font: {}", e);
            return Err(io::Error::new(io::ErrorKind::NotFound, "Failed to load font"));
        }
    };

    loop {
        terminal.draw(|f| {
            let size = f.size();

            // Create centered layout
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(40),
                        Constraint::Percentage(20),
                        Constraint::Percentage(40),
                    ]
                    .as_ref(),
                )
                .split(size);

            let inner_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Percentage(20),
                        Constraint::Percentage(60),
                        Constraint::Percentage(20),
                    ]
                    .as_ref(),
                )
                .split(chunks[1]);

            // Get current time
            let now = Local::now();
            let time_str = now.format("%H:%M:%S").to_string();

            // Generate big ASCII text
            let big_time = match standard_font.convert(&time_str) {
                Some(time) => time,
                None => {
                    // Fallback if conversion fails
                    eprintln!("Failed to convert time: '{}'", time_str);
                    return;
                }
            };

            // Create a styled paragraph for the clock, without the border
            let text = vec![big_time.to_string()];
            let clock = Paragraph::new(text.join("\n"))
                .block(Block::default()) 
                .alignment(Alignment::Center)
                .style(
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                );

            f.render_widget(clock, inner_chunks[1]);
        })?;

        // Handle input
        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    return Ok(());
                }
            }
        }

        thread::sleep(Duration::from_millis(100));
    }
}

