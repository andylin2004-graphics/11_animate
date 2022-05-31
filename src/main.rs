mod color;
mod draw;
mod gmath;
mod image;
mod matrix;
mod parser;
mod reflect;
use color::Color;
use image::Image;
use std::time::Instant;
use matrix::CurveType;
use matrix::Matrix;
use parser::parse;
use reflect::ReflectionValue;
use std::env;

extern crate pest;
#[macro_use]
extern crate pest_derive;

pub mod consts {
    use crate::color::Color;
    use crate::reflect::ReflectionValue;

    pub const AMBIENT_COLOR: Color = Color::new_color(50, 50, 50);
    pub const AMBIENT_REFLECT: ReflectionValue = ReflectionValue::new_values(0.1, 0.1, 0.1);
    pub const DIFFUSE_REFLECT: ReflectionValue = ReflectionValue::new_values(0.5, 0.5, 0.5);
    pub const SPECULAR_REFLECT: ReflectionValue = ReflectionValue::new_values(0.5, 0.5, 0.5);
    pub const POINT_LIGHT_LOCATION: [f32; 3] = [0.5, 0.75, 1.0];
    pub const POINT_LIGHT_COLOR: Color = Color::new_color(255, 255, 255);
    pub const VIEW: [f32; 3] = [0.0, 0.0, 1.0];
    pub const STEP_2D: i32 = 100;
    pub const STEP_3D: i32 = 100;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "art"{
        parse("face.mdl");
    }else{
        let time = Instant::now();
        parse("simple_anim.mdl");
        println!("Render finished in {:?}", time.elapsed())
    }
}
