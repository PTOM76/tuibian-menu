use std::io::stdout;
use color_eyre::Result;
use crossterm::event::{
    self, 
    Event, 
    KeyCode, 
    MouseEventKind, 
    EnableMouseCapture, 
    DisableMouseCapture,
};
use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, List, ListItem, BorderType},
    style::{Style, Color, Modifier},
    Terminal,
};
use std::io;
use crossterm::{
    execute, 
    cursor::{Hide, Show}, 
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode, disable_raw_mode},
};

const OPTIONS: [&str; 6] = ["vim", "fmtui", "btop", "nmtui", "", "quit"];

/// main
fn main() -> Result<()> {
    color_eyre::install()?;

    let options = OPTIONS;
    let mut choice = 0;

    loop {
        let mut stdout = stdout();
        
        execute!(stdout, EnterAlternateScreen, Hide, EnableMouseCapture)?;
        enable_raw_mode()?;

        let backend = CrosstermBackend::new(&mut stdout);
        let mut terminal = Terminal::new(backend)?;

        loop {
             let area = draw(&mut terminal, &options, choice)?;
            if input(options.len(), &mut choice, area)? {
                break;
            }
        }

        disable_raw_mode()?;
        drop(terminal);
        execute!(stdout, LeaveAlternateScreen, Show, DisableMouseCapture)?;

        match options[choice] {
            "vim" => { std::process::Command::new("vim").status()?; }
            "fmtui" => { std::process::Command::new("fmtui").status()?; }
            "btop" => { std::process::Command::new("btop").status()?; }
            "nmtui" => { std::process::Command::new("nmtui").status()?; }
            "quit" => { break; }
            _ => {}
        }
    }

    Ok(())
}

/// draw
///
/// # Arguments
/// * `terminal` - Reference to terminal object
/// * `options` - Slice of option strings to display
/// * `choice` - choice index
///
/// # Returns
/// is successful?
fn draw(terminal: &mut Terminal<CrosstermBackend<&mut io::Stdout>>, options: &[&str], choice: usize) -> Result<ratatui::layout::Rect> {
    let mut area = ratatui::layout::Rect::default();

    terminal.draw(|f| {
        let size = f.area();
        area = size;

        let items: Vec<ListItem> = options
            .iter()
            .enumerate()
            .map(|(i, text)| {
                let prefix = if i == choice { "> " } else { "  " };
                ListItem::new(format!("{}{}", prefix, text))
            })
            .collect();

        let list = List::new(items).block(Block::default()
            .borders(Borders::ALL)
            .title("──┤ Tuibian ├──")
            .border_style(Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD))
            .border_type(BorderType::Rounded),
        );
        f.render_widget(list, area);
    })?;
    Ok(area)
}

/// input
///
/// # Arguments
/// * `options_len` - Length of options
/// * `choice` - Mutable reference to choice index
//// # Returns
/// is exit flag?
fn input(options_len: usize, choice: &mut usize, list_area: ratatui::layout::Rect) -> Result<bool> {
    if event::poll(std::time::Duration::from_millis(200))? {
        let ev = event::read()?;
        if let Event::Key(key) = ev {
            match key.code {
                KeyCode::Up => {
                    loop {
                        *choice = if *choice == 0 { options_len - 1 } else { *choice - 1 };
                        if !OPTIONS[*choice].is_empty() {
                            break;
                        }
                    }
                },
                KeyCode::Down => {
                    loop {
                        *choice = if *choice == options_len - 1 { 0 } else { *choice + 1 };
                        if !OPTIONS[*choice].is_empty() {
                            break;
                        }
                    }
                }            
                KeyCode::Enter | KeyCode::Esc => return Ok(true),
                _ => {}
            }
        }

        if let Event::Mouse(btn) = ev {
            if let MouseEventKind::Down(_) = btn.kind {
                let x = btn.column;
                let y = btn.row;

                if x >= list_area.x && x < list_area.x + list_area.width && y >= list_area.y && y < list_area.y + list_area.height {
                    let index = (y - list_area.y - 1) as usize;
                    if index < options_len && !OPTIONS[index].is_empty() {
                        *choice = index;
                        return Ok(true);
                    }
                }
            }
        }
    }
    Ok(false)
}
