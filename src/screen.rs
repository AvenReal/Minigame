use std::{io::{self, Write}, thread::sleep, time::Duration};

use crate::commands;
use terminal_size::{Height, Width, terminal_size};

pub fn print_on_screen(y: usize, x: usize, message: &str) {
    print!("\x1B[{};{}H{}", y, x, message);
    io::stdout().flush();
    move_cursor_down();
}

/// .
pub fn print_alert(message: &str) {
    
    let n = message.len();
    print_on_screen(5, n / 2, message);
    sleep(Duration::new(3, 0));
    print_on_screen(5, n / 2, &" ".repeat(n));
}

pub fn clear(){
    let (h, w) = match get_screen_size(){
        Ok((h, w)) => (h, w),
        Err(_) => return,
    };

    for i in 1..=h {
        println!("\x1B[{i};1H{}", &" ".repeat(w as usize));
    }
}

pub fn move_cursor_down(){
    let h = match get_screen_size() {
        Ok((h, _)) => h,
        Err(_) => return ,
    };
    print!("\x1B[{};{}H{}", h-2, 1, ">_: ");
    io::stdout().flush();
}

pub fn clear_command_line() {
    let (h, w) = match get_screen_size() {
        Ok((h, w)) => (h, w),
        Err(_) => return ,
    };
    print_on_screen((h-2) as usize, 1, &" ".repeat(w as usize));
    
}

/// Retur the screen size as (Height : u16, Width : u16).
///
/// # Errors
///
/// This function will return an error if the screen size is unable to be read.
pub fn get_screen_size() -> Result<(u16, u16), commands::Command> {
    return if let Some((Width(w), Height(h))) = terminal_size() {
        Ok((h, w))
    } else {
        Err(commands::Command::Invalid(String::from("Unable to get the terminal size !")))
    }
}
