use crossterm::event::read;

fn main() {
    loop {
        println!("{:?}", read().unwrap());
    }
}
