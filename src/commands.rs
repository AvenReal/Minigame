enum Direction {
    Left,
    Right,
    Up,
    Down,
}
pub enum Command {
    None,
    Invalid(String),
    Exit,
    Help(String),
    Move(u8, Direction),
    Repeat(u8, String),
    And(String, String),
}

impl Command {
    fn nb_args(self) -> u8 {
        return match self {
            Command::Invalid(_) => 1,
            Command::Help(_) => 1,
            Command::Move(_, _) => 2,
            Command::Repeat(_, _) => 2,
            Command::And(_, _) => 2,
            _ => 0,
        };
    }

    fn help_string(s: String) -> String {
        return match s.as_str() {
            "Invalid" => String::from("Throws an error : Invalid [error message : String]"),
            "Help" => String::from("Helps you with a command : Help [Command]"),
            "Move" => String::from(
                "Moves n cells in any directtion : Move [n : number] [{left, right, up, down} : Direction] ",
            ),
            "Repeat" => String::from("Repeat n times a command : Repeat [n : number] [Command]"),
            "And" => String::from("Executes 2 commands: And [Command] [Command]"),
            "None" => String::from("The empty command: "),
            "Exit" => String::from("Quites the game : Exit"),
            cmd => format!("No Command {} found", cmd),
        };
    }

    fn parse_command(command_str: &String) -> Command {
        fn invalid_args(cmd: &str, nb_needed: usize, nb_got: usize) -> Command {
            Command::Invalid(format!(
                "{} need {} arguments found {}",
                cmd, nb_needed, nb_got
            ))
        }
        let mut args: Vec<&str> = command_str.split(' ').collect();

        let cmd_type = args.remove(0).trim();
        let nb_args = args.len();

        return match cmd_type {
            "" => Command::None,
            "exit" => Command::Exit,
            "new" => {
                return if nb_args < 2 {
                    invalid_args(cmd_type, 2, nb_args)
                } else {
                    Command::New {
                        name: String::from(args[0]),
                        password: String::from(args[1]),
                    }
                };
            }
            cmd => Command::Invalid(format!("No command found: '{}'", cmd)),
        };
    }

    fn execute(&self) -> bool {
        match self {
            Command::Exit => return false,

            Command::Invalid(err_msg) => {
                println!("\x1B[0;31m{}\x1B[0m", err_msg);
            }
            Command::New { name, password } => {
                println!("New profile:\n\tName : {}\n\tPassword : {}", name, password)
            }
            Command::None => (),
        }

        return true;
    }
}
