use crossterm::event::{read, Event, KeyCode, KeyEventKind};
use std::fs::read_to_string;

fn main() {
    let file_content = read_to_string("typing.txt").unwrap();
    let mut user_input = String::new();

    loop {
        println!("{file_content}");
        println!("{user_input}_");

        if let Event::Key(key_event) = read().unwrap() {
            if key_event.kind == KeyEventKind::Press {
                match key_event.code {
                    KeyCode::Backspace => {
                        user_input.pop();
                    }
                    KeyCode::Esc => break,
                    KeyCode::Char(c) => {
                        user_input.push(c);
                    }
                    _ => {}
                }
            }
        }
    }
}
