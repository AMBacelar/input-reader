mod parse_input;

use crossterm::{
    event::{self, poll, Event, KeyCode},
    execute,
    terminal::enable_raw_mode,
};
use fps_counter::*;
use gilrs::{self, Gilrs};
use std::io;
use std::time::{Duration, Instant};
use tui::{
    backend::CrosstermBackend,
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

const MAX_MS_FOR_60FPS: f32 = 16.7;

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;
    let fps_count = FPSCounter::default();

    let app = run_app(fps_count, &mut terminal);

    if let Err(err) = app {
        println!("{:?}", err);
    }

    terminal.clear()?;
    terminal.show_cursor()?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}

// I just want to create a widget that is a block, with the text inside, updated very "frame"

fn run_app(
    mut fps_count: FPSCounter,
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
) -> io::Result<()> {
    let mut gilrs = Gilrs::new().unwrap();
    let mut active_gamepad = None;
    let mut current_frame_text: String = Default::default();
    let mut elapsed: Duration = Default::default();
    let mut elapsed_as_milliseconds: f32 = elapsed.as_millis() as f32;
    loop {
        let start = Instant::now();
        while let Some(gilrs::Event {
            id,
            event: _,
            time: _,
        }) = gilrs.next_event()
        {
            // println!("{:?} New event from {}: {:?}", time, id, event);
            active_gamepad = Some(id);
        }
        if let Some(gamepad) = active_gamepad.map(|id| gilrs.gamepad(id)) {
            current_frame_text = parse_input::parse(gamepad);
            // println!("{}", current_frame_text);
        }
        let wait_duration = (MAX_MS_FOR_60FPS - elapsed_as_milliseconds) as u64;
        let fps = format!("{}fps", fps_count.tick());
        let debug_line = format!("Frame took how long?: {}ms, waited for {}ms [Note: 16.7ms is how long it needs to be for 60fps]", elapsed_as_milliseconds, wait_duration);
        let text = vec![
            Spans::from(vec![Span::raw(&current_frame_text)]),
            Spans::from(vec![Span::raw(fps)]),
            Spans::from(vec![Span::raw(debug_line)]),
        ];
        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default().title("Input reader").borders(Borders::ALL);
            let paragraph = Paragraph::new(text)
                .block(block)
                .wrap(tui::widgets::Wrap { trim: true });
            f.render_widget(paragraph, size);
        })?;

        if poll(Duration::from_secs(0))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc => return Ok(()),
                    _ => {}
                }
            }
        }
        elapsed = start.elapsed();
        elapsed_as_milliseconds = elapsed.as_millis() as f32;
        if elapsed_as_milliseconds < MAX_MS_FOR_60FPS {
            std::thread::sleep(Duration::from_millis(wait_duration));
        }
    }
}
