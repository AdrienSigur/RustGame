
use macroquad::prelude::*;


// Player Struct to create a new player
pub struct Player {
    pub name: String,
    pub health: i32,
    pub attack: i32,
    pub x: f32,
    pub y: f32,
}



impl Player {
    // Basic method for creating a new character
    pub fn new(name: &str, health: i32, attack: i32 , x : f32 , y : f32) -> Self {
        Self {
            name: name.to_string(),
            health,
            attack,
            x ,
            y
        }
    }
    // Enable the character to move up , down , right and left with keybind
    pub fn move_character(&mut self) {

        let speed = 15.0;

        match get_last_key_pressed() {
            Some(KeyCode::Right ) => {
                self.x += speed;
            }
            Some(KeyCode::Left) => {
                self.x -= speed;
            }
            Some(KeyCode::Down) => {
                self.y += speed + 10.0;

            }
            Some(KeyCode::Up) => {
                self.y -= speed + 20.0;

            }

            _ => {}
        }
    }

    // Character Design with the color customizable
    pub fn character(&self , color: Color) {
        draw_circle(self.x  , self.y , 8.0, color);
        draw_line(self.x , self.y + 8.0 , self.x , self.y + 20.0 , 2.0 , color);
        draw_line(self.x - 10.0 , self.y + 13.0 , self.x + 10.0 , self.y + 13.0 , 2.0 , color );
        draw_line(self.x - 1.0, self.y + 17.0, self.x +10.0 , self.y + 30.0 , 2.0 , color );
        draw_line(self.x + 1.0 , self.y + 17.0 , self.x - 10.0 , self.y + 30.0 , 2.0 , color );

    }


}