

mod commands;
mod screen;

fn main() {
    let mut inp = String::new();

    for i in 1..=128 {
        if i % 2 == 0 {
            println!("{} : {:b} -> {:b} ", i, 't' as u8, 't' as u8 / i);
        }
    }
    while commands::parse_command(&inp).execute() {
        inp.clear();
        stdin().read_line(&mut inp);
        
    }
}
