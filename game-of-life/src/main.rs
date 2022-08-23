use std::{io::stdout, time::Duration};

use crossterm::cursor::MoveTo;
use crossterm::terminal::EnterAlternateScreen;
use crossterm::{
    event::{poll, read, Event},
    execute,
    style::Print,
    terminal::{Clear, ClearType, LeaveAlternateScreen},
};
use game::Universe;

mod game;

fn main() -> Result<(), std::io::Error> {
    let mut uni = Universe::new(10, 10);
    uni.set_cells(&[
        (4, 4),
        (5, 4),
        (6, 4),
        (5, 5),
        (6, 5),
        (7, 7),
        (7, 8),
        (7, 9),
    ]);

    execute!(stdout(), EnterAlternateScreen)?;
    let mut count = 1;
    let mut cycle = format!("Cycle {}", count);
    loop {
        if poll(Duration::from_millis(1000))? {
            match read()? {
                Event::Key(_) => break,
                _ => {}
            }
        } else {
            execute!(
                stdout(),
                Clear(ClearType::All),
                MoveTo(0, 0),
                Print(&uni),
                Print(&cycle),
                Print("\nPress enter to exit ...")
            )?;
            count += 1;
            cycle = format!("Cycle {}", count);
            uni.process();
        }
    }

    execute!(stdout(), LeaveAlternateScreen)?;
    Ok(())
}
