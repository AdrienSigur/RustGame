mod titlescreen;
mod character;
mod decoration;


use character::Player;
use macroquad::prelude::*;
use crate::decoration::{Decor, LandChunk};


#[macroquad::main("MyGame")]
async fn main() {

    let mut score : i32 = 0;
    let mut player = Player::new("Eidowardo", 30, 05, 40.0, 360.0);
    let void : i32 = 0 ;
    let mut last_score  = get_time() ;

    let grass = Decor::new(0.0, 403.0, 3.0);
    let dirt = Decor::new(0.0, 405.0, 3.0);
    let terrain = LandChunk::new(dirt, grass);

    let grass1 = Decor::new(180.0, 500.0, 3.0);
    let dirt1 = Decor::new(180.0, 502.0, 3.0);
    let terrain2 = LandChunk::new(dirt1, grass1);

    let grass2 = Decor::new(430.0, 400.0, 3.0);
    let dirt2 = Decor::new(430.0, 402.0, 3.0);
    let terrain3 = LandChunk::new(dirt2, grass2);


    loop {
        clear_background(BLUE);


        terrain.draw(180.0 , 180.0 , 200.0);
        terrain2.draw(250.0 , 250.0 , 150.0);
        terrain3.draw(150.0 , 150.0 , 250.0);


        player.move_character();  // Déplace le personnage
        player.character(RED);




        let current_time = get_time();


        // if player.y + 30.0 >= decor.y  {
        //     player.y = decor.y - 30.0;
        // }

        if current_time - last_score >= 1.0 {
            score += 1;
            last_score = current_time;
        }

        doubleword(20.0 , 25.0 , &score , "Time" , BLACK);

        doubleword(150.0 , 25.0 , &void , "Score" , BLACK );

        // arrow(50.0, 50.0 );

        next_frame().await
    }
}


fn arrow(x : f32, y : f32 )  {

    draw_line(x + 40.0 , x , x+10.0, y, 1.0, RED);
    draw_line(x -20.0 , x  , x + 10.0 , y  , 1.0, RED);
     draw_line(x + 10.0  ,20.0 , x + 10.0  , y  , 1.0, BLUE  );
}

fn doubleword (x : f32, y : f32, score: &i32, text : &str , color : Color ){

    draw_text(text, x , y , 25.0, color);
    draw_text(&score.to_string(), x + 60.0, y, 25.0, color);
}

