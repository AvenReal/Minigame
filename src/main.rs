mod player;
mod commands;
mod screen;

use std::io::stdin;

fn main() {
    let mut inp = String::new();

    while commands::parse_command(&inp).execute() {
        inp.clear();

        match stdin().read_line(&mut inp) {
            _ => (),
        };
    }
}
