use crate::{
    commands::{self, Command},
    screen,
};

pub struct Player {
    pub x: u8,
    pub y: u8,
}

impl Player {
    pub fn move_to(&mut self, y: i8, x: i8) -> Result<(), commands::Command> {
        return match screen::get_screen_size() {
            Ok(size) => {
                let (h, w) = size;
                if y > h as i8 || x > w as i8 || y < 1 || x < 1 {
                    Err(Command::Invalid(format!(
                        "y : {}, x : {} is out of screen, curent size : height : {} width : {} ",
                        y, x, h as u8, w as u8
                    )))
                } else {
                    screen::print_on_screen(self.y as usize, self.x as usize, " ");
                    self.x = x as u8;
                    self.y = y as u8;
                    screen::print_on_screen(self.y as usize, self.x as usize, "☻");
                    Ok(())
                }
            }
            Err(c) => Err(c),
        };
    }

    pub fn move_direction(
        &mut self,
        n: u8,
        direction: &commands::Direction,
    ) -> Result<(), commands::Command> {
        let dy: i8 = match direction {
            commands::Direction::Up => -1,
            commands::Direction::Down => 1,
            _ => 0,
        };
        let dx: i8 = match direction {
            commands::Direction::Left => -1,
            commands::Direction::Right => 1,
            _ => 0,
        };

        self.move_to(self.y as i8 + dy * n as i8, self.x as i8 + dx * n as i8)
    }
}
