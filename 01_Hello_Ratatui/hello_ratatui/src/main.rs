use std::io::{stdout, Result};
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, KeyCode, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    style::Stylize, widgets::Paragraph, Terminal
};

fn main() -> Result<()> {
    // Setup
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    // Main Loop
    loop{
        // Draw UI
        terminal.draw(|frame| {
            let size = frame.size();
            frame.render_widget(Paragraph::new("Hello Ratatui 😉 (Press 'q' or 'Q' to quit)")
            .white()
            .on_blue(),
            size
                );
            }
        )?;

        // Handle Events
        if event::poll(std::time::Duration::from_millis(16))? { // 16ms ~ 60 FPS
            // Exit the loop if q or Q is pressed
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && (key.code == KeyCode::Char('q') || key.code == KeyCode::Char('Q')){
                    break;
                } 
            }
        }
    }

    // Cleanup
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}