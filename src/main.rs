mod player;
mod commands;
mod screen;

use std::io::stdin;

fn main() {
    let mut inp = String::new();

    let mut p = player::Player{
        x: 3,
        y: 3,
    };
    screen::clear();
    while commands::parse_command(&inp).execute(&mut p) {
        inp.clear();

        match stdin().read_line(&mut inp) {
            _ => (),
        };
    }
}
