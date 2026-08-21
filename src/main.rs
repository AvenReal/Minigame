mod commands;
mod player;

fn main() {
    let mut inp = String::new();

    for i in 1..=128 {
        if i % 2 == 0 {
            println!("{} : {:b} -> {:b} ", i, 't' as u8, 't' as u8 / i);
        }
    }
    while commands::Command::parse_command(&inp).execute() {
        inp.clear();
        stdin().read_line(&mut inp);
    }
}
