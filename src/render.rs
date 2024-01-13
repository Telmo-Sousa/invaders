use crate::frame::Frame;
use crossterm::{
    cursor::MoveTo,
    style::{Color, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType, size},
    QueueableCommand,
};
use std::io::{Stdout, Write};

pub fn render(stdout: &mut Stdout, last_frame: &Frame, curr_frame: &Frame, force: bool) {
    if force {
        stdout.queue(SetBackgroundColor(Color::DarkBlue)).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
        stdout.queue(SetForegroundColor(Color::White)).unwrap();
    }

    // this gets the size of the terminal
    let (term_width, term_height) = size().unwrap();

    // this calculates the center position so the game is centered
    let center_x = (term_width - curr_frame.len() as u16) / 2;
    let center_y = (term_height - curr_frame[0].len() as u16) / 2;

    for (x, col) in curr_frame.iter().enumerate() {
        for (y, s) in col.iter().enumerate() {
            if *s != last_frame[x][y] || force {
                // Adjust the position to center the game
                let pos_x = center_x + x as u16;
                let pos_y = center_y + y as u16;

                stdout.queue(MoveTo(pos_x, pos_y)).unwrap();
                print!("{}", *s);
            }
        }
    }
    stdout.flush().unwrap();
}
