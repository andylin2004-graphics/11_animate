use crate::color::Color;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;

pub struct Image {
    pub screen: Vec<Vec<Color>>,
    pub z_buffer: Vec<Vec<f32>>,
    pub height: usize,
    pub width: usize,
}

impl Image {
    pub fn new(image_width: usize, image_height: usize) -> Image {
        Image {
            screen: vec![vec![Color::new(); image_width]; image_height],
            z_buffer: vec![vec![f32::MIN; image_width]; image_height],
            width: image_width,
            height: image_height,
        }
    }

    pub fn plot(&mut self, x: i32, y: i32, mut z: f32, color: &Color) -> bool{
        z = (z as i32 * 10000) as f32 / 10000.0;
        if x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32{
            if z >= self.z_buffer[(self.height - 1) - y as usize][x as usize] {
                self.screen[(self.height - 1) - y as usize][x as usize].plot_color(color);
                self.z_buffer[(self.height - 1) - y as usize][x as usize] = z;
            }
            return true;
        }else{
            return false;
        }
    }

    fn create_data(&self) -> String {
        let mut result: String =
            format!("P3\n{} {}\n255\n", self.screen[0].len(), self.screen.len());

        for i in 0..self.screen.len() {
            for v in 0..self.screen[i].len() {
                result.push_str(&self.screen[i][v].to_string().to_owned());
                result.push_str("  ");
            }
            result.push_str("\n");
        }
        return result;
    }

    pub fn create_file(&self, file_name: &str) {
        let path = Path::new(&file_name);

        let mut file = match File::create(&path) {
            Err(error) => panic!("failed to create image file because {}", error),
            Ok(file) => file,
        };

        let result = self.create_data();

        match file.write_all(result.as_bytes()) {
            Err(error) => panic!("failed to write image file because {}", error),
            Ok(_) => {}
        };
    }

    pub fn clear(&mut self) {
        for i in 0..self.screen.len() {
            for v in 0..self.screen[0].len() {
                self.screen[i][v].reset_color();
                self.z_buffer[i][v] = f32::MIN;
            }
        }
    }

    pub fn display(&mut self) {
        let mut file_name: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect();
        file_name = format!("/tmp/imageDisplay{}.ppm", file_name);
        self.create_file(&file_name);
        Command::new("open")
            .arg(file_name)
            .spawn()
            .expect("failed to open image");
    }
}