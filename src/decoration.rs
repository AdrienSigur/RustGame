use macroquad::color::{Color, BROWN};
use macroquad::math::Rect;
use macroquad::prelude::{draw_line, GREEN};
use macroquad::shapes::draw_rectangle;

pub struct Decor {
    pub x : f32,
    pub y : f32,
    pub z : f32,

}

impl Decor {
    pub fn new(x : f32 , y : f32 , z : f32 ) -> Self {
        Self{
            x ,
            y,
            z,
        }
    }

    pub(crate) fn line(&self, x2 : f32, y2 : f32, color : Color ) {
        draw_line(self.x , self.y , x2 , y2 , self.z , color);
    }


    pub(crate) fn rect(&self, width : f32, height : f32, color : Color) {
        draw_rectangle(self.x, self.y, self.x + width, self.y + height, color);
    }



}

pub struct LandChunk {
    pub dirt : Decor ,
    pub grass : Decor,

}

impl LandChunk {
    pub fn new(dirt: Decor, grass: Decor) -> Self {
        Self { dirt, grass }
    }

    // Fonction pour dessiner le terrain
    pub fn draw(&self, line_size: f32, width_rect: f32, height_rect: f32) {
        draw_rectangle(
            self.dirt.x, self.dirt.y,
            width_rect, height_rect, BROWN
        );

        // Dessiner l'herbe (ligne verte au-dessus)
        draw_line(
            self.grass.x, self.grass.y,
            self.grass.x + line_size, self.grass.y,
            self.grass.z, GREEN
        );
    }
}



