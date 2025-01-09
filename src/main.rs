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

            // Dynamically calculate centered layout
            let clock_height = 10; // Estimated height of the ASCII art clock
            let clock_width = 60; // Estimated width of the ASCII art clock

            let vertical_padding = (size.height.saturating_sub(clock_height)) / 2;
            let horizontal_padding = (size.width.saturating_sub(clock_width)) / 2;

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length(vertical_padding), // Top padding
                        Constraint::Min(clock_height),       // Clock height
                        Constraint::Length(vertical_padding), // Bottom padding
                    ]
                    .as_ref(),
                )
                .split(size);

            let inner_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Length(horizontal_padding), // Left padding
                        Constraint::Min(clock_width),          // Clock width
                        Constraint::Length(horizontal_padding), // Right padding
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

            // Create a styled paragraph for the clock
            let text = vec![big_time.to_string()];
            let clock = Paragraph::new(text.join("\n"))
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

