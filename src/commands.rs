use crate::commands;
use crate::player;
use crate::screen;

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn from_str(s: &str) -> Option<Direction> {
        if s.cmp("left").is_eq() {
            Some(Direction::Left)
        } else if s.cmp("right").is_eq() {
            Some(Direction::Right)
        } else if s.cmp("up").is_eq() {
            Some(Direction::Up)
        } else if s.cmp("down").is_eq() {
            Some(Direction::Down)
        } else {
            None
        }
    }
}

pub enum Command {
    None,
    Invalid(String),
    Exit,
    Help(String),
    Move(u8, Direction),
    Repeat(u8, String),
}

pub fn parse_command(command_str: &String) -> Command {
    fn try_invalid(cmd: &str, arg_got: usize) -> Option<Command> {
        let arg_needed = Command::nb_args(cmd);
        if arg_got < arg_needed {
            return Some(Command::Invalid(format!(
                "{} need {} arguments found {}",
                cmd, arg_needed, arg_got
            )));
        }
        return None;
    }

    let mut args: Vec<&str> = command_str.split(' ').collect();

    let cmd_type = args.remove(0).trim();
    let nb_args = args.len();

    return match cmd_type {
        "" => Command::None,
        "Exit" => Command::Exit,
        "Help" => match try_invalid(cmd_type, nb_args) {
            Some(cmd) => cmd,
            None => Command::Help(String::from(args[0])),
        },
        "Move" => match try_invalid(cmd_type, nb_args) {
            Some(cmd) => cmd,
            None => Command::Move(
                match args[0].parse() {
                    Ok(n) => n,
                    Err(_) => {
                        return Command::Invalid(format!(
                            "Argument needed to be a number, got {}",
                            args[0]
                        ));
                    }
                },
                match Direction::from_str(args[1].trim()) {
                    Some(d) => d,
                    None => {
                        return Command::Invalid(format!(
                            "Argument needed to be a direction (either left, right, up or down), got {}",
                            args[1]
                        ));
                    }
                },
            ),
        },
        "Repeat" => match try_invalid(cmd_type, nb_args) {
            Some(cmd) => cmd,
            None => Command::Repeat(
                match args[0].parse() {
                    Ok(n) => n,
                    Err(_) => {
                        return Command::Invalid(format!(
                            "Argument needed to be a number, got {}",
                            args[0]
                        ));
                    }
                },
                args[1..].join(" "),
            ),
        },
        cmd => Command::Invalid(format!("No command found: '{}'", cmd)),
    };
}

impl Command {
    fn nb_args(s: &str) -> usize {
        return match s {
            "Invalid" => 1,
            "Help" => 1,
            "Move" => 2,
            "Repeat" => 2,
            _ => 0,
        };
    }

    fn help_string(s: &str) -> String {
        return match s {
            "Invalid" => String::from("Throws an error : Invalid [error message : String]"),
            "Help" => String::from("Helps you with a command : Help [Command]"),
            "Move" => String::from(
                "Moves n cells in a directtion : Move [n : number] [Direction : {left, right, up, down}]",
            ),
            "Repeat" => String::from("Repeat n times a command : Repeat [n : number] [Command]"),
            "And" => String::from("Executes 2 commands: And [Command] [Command]"),
            "None" => String::from("The empty command: "),
            "Exit" => String::from("Quites the game : Exit"),
            cmd => format!("No Command {} found", cmd),
        };
    }

    pub fn execute(&self, p: &mut player::Player) -> bool {
        
        match self {
            Command::Exit => return false,
            Command::Invalid(err_msg) => {
                screen::print_alert(err_msg);
            }

            Command::None => (),
            Command::Help(cmd) => screen::print_alert(&Command::help_string(cmd.trim())),
            Command::Move(n, direction) => match p.move_direction(*n, direction) {
                Ok(_) => (),
                Err(c) => return c.execute(p),
            },
            Command::Repeat(n, s) => {
                let c = parse_command(s);
                for i in 0..(*n as i32) {
                    c.execute(p);
                }
            }
        }

        return true;
    }

}
