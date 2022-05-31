use std::process::Command;
use crate::color::Color;
use crate::consts;
use crate::image::{Image, make_animation};
use crate::matrix::CurveType;
use crate::matrix::Matrix;
use crate::pest::Parser;
use crate::ReflectionValue;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};

#[derive(Parser)]
#[grammar = "mdl.pest"]
struct MDLParser;

#[derive(Debug)]
struct Constants {
    pub ambient_reflect: ReflectionValue,
    pub diffuse_reflect: ReflectionValue,
    pub specular_reflect: ReflectionValue,
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl Constants {
    fn new(
        ambient_red: f32,
        diffuse_red: f32,
        specular_red: f32,
        ambient_green: f32,
        diffuse_green: f32,
        specular_green: f32,
        ambient_blue: f32,
        diffuse_blue: f32,
        specular_blue: f32,
        red: f32,
        green: f32,
        blue: f32,
    ) -> Constants {
        Constants {
            ambient_reflect: ReflectionValue::new_values(ambient_red, ambient_green, ambient_blue),
            diffuse_reflect: ReflectionValue::new_values(diffuse_red, diffuse_green, diffuse_blue),
            specular_reflect: ReflectionValue::new_values(
                specular_red,
                specular_green,
                specular_blue,
            ),
            red,
            green,
            blue
        }
    }
}

pub fn parse(fname: &str) {
    let file = File::open(&fname).expect("Unable to open file");
    let mut reader = BufReader::new(file);
    let mut instructions = String::new();
    reader.read_to_string(&mut instructions).expect("Unable to read file");
    
    let commands = MDLParser::parse(Rule::IDENT_LIST, &instructions);
    let mut screen = Image::new(500, 500);
    let color = Color::new_color(0, 255, 0);
    let mut edges = Matrix::new(0, 0);
    let mut polygons = Matrix::new(0, 0);
    let mut cstack = vec![Matrix::new(0, 0); 0];
    let mut constants_store = HashMap::new();
    let mut basename = String::from("output");
    let mut vary_exists = false;
    let mut frames_exists = false;
    let mut frames: Vec<HashMap<&str, f32>> = vec![HashMap::new()];

    clean_animation_directory();
    cstack.push(Matrix::identity());
    // to get the frame rate
    for pair in commands.clone() {
        for command in pair {
            let error_message = command.as_str();
            match command.as_rule(){
                Rule::FRAMES_D => {
                    let mut command_contents = command.into_inner();
                    frames = vec![HashMap::new(); command_contents.next().unwrap().as_str().parse().expect(&*format!("Not a valid frame count at {}", error_message))];
                    frames_exists = true;
                }
                Rule::BASENAME_S => {
                    let mut command_contents = command.into_inner();
                    basename = command_contents.nth(1).unwrap().as_str().to_owned();
                }
                Rule::BASENAME => {
                    println!("WARNING: a default basename will be used instead because basename is missing at {}", error_message);
                }
                Rule::VARY_SDDDD => {
                    vary_exists = true;
                }
                _ => {}
            }
        }
    }
    // pass 1
    if vary_exists{
        if !frames_exists{
            println!("ERROR: vary used without frame numbers included");
            return;
        }else{
            for pair in commands.clone() {
                for command in pair {
                    let error_message = command.as_str();
                    match command.as_rule() {
                        Rule::VARY_SDDDD => {
                            let mut command_contents = command.into_inner();
                            let knob_name = command_contents.next().unwrap().as_str();
                            let start_frame: u32 = command_contents.next().unwrap().as_str().parse().expect(&*format!("Not a valid start frame number at {}", error_message));
                            let end_frame: u32 = command_contents.next().unwrap().as_str().parse().expect(&*format!("Not a valid end frame number at {}", error_message));
                            if end_frame < start_frame {
                                println!("ERROR: start frame number is greater than end frame number at {}", error_message);
                                return;
                            }
                            let start_value: f32 = command_contents.next().unwrap().as_str().parse().expect(&*format!("Not a valid start knob value at {}", error_message));
                            let end_value: f32 = command_contents.next().unwrap().as_str().parse().expect(&*format!("Not a valid end knob value at {}", error_message));
                            let frame_count = end_frame - start_frame;
                            let mut current_value = start_value;
                            let change_in_value = (end_value - start_value) / frame_count as f32;
                            for frame_num in start_frame..=end_frame{
                                frames[frame_num as usize].insert(knob_name, current_value);
                                current_value += change_in_value;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    // pass 2
    for frame_num in 0..frames.len(){
        for pair in commands.clone() {
            for command in pair {
                let error_message = command.as_str();
                match command.as_rule() {
                    Rule::CONSTANTS_SDDDDDDDDD => {
                        let mut command_contents = command.into_inner();
                        let name = command_contents.next().unwrap().as_str();
                        let constant = Constants::new(command_contents.next().unwrap().as_str().parse().expect(error_message), command_contents.next().unwrap().as_str().parse().expect(error_message), command_contents.next().unwrap().as_str().parse().expect(error_message), command_contents.next().unwrap().as_str().parse().expect(error_message), command_contents.next().unwrap().as_str().parse().expect(error_message), command_contents.next().unwrap().as_str().parse().expect(error_message), command_contents.next().unwrap().as_str().parse().expect(error_message), command_contents.next().unwrap().as_str().parse().expect(error_message), command_contents.next().unwrap().as_str().parse().expect(error_message), 0.0, 0.0, 0.0);
                        constants_store.insert(name, constant);
                    }
                    // Rule::CONSTANTS_SDDDDDDDDDDDD => {
                    //     let mut command_contents = command.into_inner();
                    //     let name = command_contents.next().unwrap().as_str();
                    //     let constant = Constants::new(command_contents.next().unwrap().as_str().parse().expect(error_message), command_contents.next().unwrap().as_str().parse().expect(error_message), command_contents.next().unwrap().as_str().parse().expect(error_message), command_contents.next().unwrap().as_str().parse().expect(error_message), command_contents.next().unwrap().as_str().parse().expect(error_message), command_contents.next().unwrap().as_str().parse().expect(error_message), command_contents.next().unwrap().as_str().parse().expect(error_message), command_contents.next().unwrap().as_str().parse().expect(error_message), command_contents.next().unwrap().as_str().parse().expect(error_message), command_contents.next().unwrap().as_str().parse().expect(error_message), command_contents.next().unwrap().as_str().parse().expect(error_message), command_contents.next().unwrap().as_str().parse().expect(error_message));
                    //     constants_store.insert(name, constant);
                    // }
                    Rule::PPUSH => {
                        cstack.push(cstack.last().unwrap().clone());
                    }
                    Rule::PPOP => {
                        cstack.pop();
                    }
                    Rule::MOVE_DDD | Rule::MOVE_DDDS => {
                        let mut command_contents = command.into_inner();
                        let mut translate = Matrix::make_translate_with_scale(
                            command_contents.next().unwrap().as_str().parse().expect(error_message),
                            command_contents.next().unwrap().as_str().parse().expect(error_message),
                            command_contents.next().unwrap().as_str().parse().expect(error_message),
                            if let Some(knob_name) = command_contents.next(){
                                if frames[frame_num].contains_key(knob_name.as_str()){
                                    *frames[frame_num].get(&*knob_name.as_str()).expect(error_message)
                                }else{
                                    0.0
                                }
                            }else{
                                1.0
                            }
                        );
                        translate.multiply_matrixes(&cstack.pop().unwrap());
                        cstack.push(translate);
                    }
                    Rule::ROTATE_SD | Rule::ROTATE_SDS => {
                        let mut command_contents = command.into_inner();
                        let rot_axis = command_contents.next().unwrap().as_str();
                        let mut rot_amount: f32 = command_contents.next().unwrap().as_str().parse().expect(error_message);
                        if let Some(knob_name) = command_contents.next(){
                            rot_amount *= if frames[frame_num].contains_key(knob_name.as_str()){
                                *frames[frame_num].get(knob_name.as_str()).expect(error_message)
                            }else {
                                0.0
                            }
                        }
                        match rot_axis {
                            "x" => {
                                let mut rot = Matrix::make_rot_x(rot_amount);
                                rot.multiply_matrixes(&cstack.pop().unwrap());
                                cstack.push(rot);
                            }
                            "y" => {
                                let mut rot = Matrix::make_rot_y(rot_amount);
                                rot.multiply_matrixes(&cstack.pop().unwrap());
                                cstack.push(rot);
                            }
                            "z" => {
                                let mut rot = Matrix::make_rot_z(rot_amount);
                                rot.multiply_matrixes(&cstack.pop().unwrap());
                                cstack.push(rot);
                            }
                            _ => {
                                panic!(
                                    "ERROR: Invalid input {} at {} for rotation: please use x, y, or z.",
                                    rot_axis, error_message
                                );
                            }
                        }
                    }
                    Rule::SCALE_DDD | Rule::SCALE_DDDS => {
                        let mut command_contents = command.into_inner();
                        let mut scale = Matrix::make_scale_with_scale(
                            command_contents.next().unwrap().as_str().parse().expect(error_message),
                            command_contents.next().unwrap().as_str().parse().expect(error_message),
                            command_contents.next().unwrap().as_str().parse().expect(error_message),
                            if let Some(knob_name) = command_contents.next(){
                                if frames[frame_num].contains_key(knob_name.as_str()){
                                    *frames[frame_num].get(&*knob_name.as_str()).expect(error_message)
                                }else{
                                    0.0
                                }
                            }else{
                                1.0
                            }
                        );
                        scale.multiply_matrixes(&cstack.pop().unwrap());
                        cstack.push(scale);
                    }
                    Rule::SPHERE_SDDDD => {
                        // println!("{:?}", command);
                        let mut command_contents = command.into_inner();
                        let lighting_constants = constants_store.get(command_contents.next().unwrap().as_str()).expect("Unable to get lighting constants");
                        polygons.add_sphere(
                            command_contents.next().unwrap().as_str().parse().expect(error_message),
                            command_contents.next().unwrap().as_str().parse().expect(error_message),
                            command_contents.next().unwrap().as_str().parse().expect(error_message),
                            command_contents.next().unwrap().as_str().parse().expect(error_message),
                            consts::STEP_3D,
                        );
                        polygons.multiply_matrixes(cstack.last().unwrap());
                        screen.draw_polygons(
                            &polygons,
                            &color,
                            &mut consts::VIEW.to_vec(),
                            &consts::AMBIENT_COLOR,
                            &mut consts::POINT_LIGHT_LOCATION.to_vec(),
                            &consts::POINT_LIGHT_COLOR,
                            &lighting_constants.ambient_reflect,
                            &lighting_constants.diffuse_reflect,
                            &lighting_constants.specular_reflect
                        );
    
                        polygons = Matrix::new(0, 0);
                    }
                    Rule::SPHERE_DDDD => {
                        let mut command_contents = command.into_inner();
                        polygons.add_sphere(
                            command_contents.next().unwrap().as_str().parse().expect(error_message),
                            command_contents.next().unwrap().as_str().parse().expect(error_message),
                            command_contents.next().unwrap().as_str().parse().expect(error_message),
                            command_contents.next().unwrap().as_str().parse().expect(error_message),
                            consts::STEP_3D,
                        );
                        polygons.multiply_matrixes(cstack.last().unwrap());
                        screen.draw_polygons(
                            &polygons,
                            &color,
                            &mut consts::VIEW.to_vec(),
                            &consts::AMBIENT_COLOR,
                            &mut consts::POINT_LIGHT_LOCATION.to_vec(),
                            &consts::POINT_LIGHT_COLOR,
                            &consts::AMBIENT_REFLECT,
                            &consts::DIFFUSE_REFLECT,
                            &consts::SPECULAR_REFLECT
                        );
    
                        polygons = Matrix::new(0, 0);
                    }
                    Rule::BOX_SDDDDDD => {
                        let mut command_contents = command.into_inner();
                        let lighting_constants = constants_store.get(command_contents.next().unwrap().as_str()).expect("Unable to get lighting constants");
                        polygons.add_box(
                            command_contents.next().unwrap().as_str().parse().expect(error_message),
                            command_contents.next().unwrap().as_str().parse().expect(error_message),
                            command_contents.next().unwrap().as_str().parse().expect(error_message),
                            command_contents.next().unwrap().as_str().parse().expect(error_message),
                            command_contents.next().unwrap().as_str().parse().expect(error_message),
                            command_contents.next().unwrap().as_str().parse().expect(error_message)
                        );
                        polygons.multiply_matrixes(cstack.last().unwrap());
    
                        screen.draw_polygons(
                            &polygons,
                            &color,
                            &mut consts::VIEW.to_vec(),
                            &consts::AMBIENT_COLOR,
                            &mut consts::POINT_LIGHT_LOCATION.to_vec(),
                            &consts::POINT_LIGHT_COLOR,
                            &lighting_constants.ambient_reflect,
                            &lighting_constants.diffuse_reflect,
                            &lighting_constants.specular_reflect
                        );
    
                        polygons = Matrix::new(0, 0);
                    }
                    Rule::BOX_DDDDDD => {
                        let mut command_contents = command.into_inner();
                        polygons.add_box(
                            command_contents.next().unwrap().as_str().parse().expect(error_message),
                            command_contents.next().unwrap().as_str().parse().expect(error_message),
                            command_contents.next().unwrap().as_str().parse().expect(error_message),
                            command_contents.next().unwrap().as_str().parse().expect(error_message),
                            command_contents.next().unwrap().as_str().parse().expect(error_message),
                            command_contents.next().unwrap().as_str().parse().expect(error_message)
                        );
                        polygons.multiply_matrixes(cstack.last().unwrap());
                        screen.draw_polygons(
                            &polygons,
                            &color,
                            &mut consts::VIEW.to_vec(),
                            &consts::AMBIENT_COLOR,
                            &mut consts::POINT_LIGHT_LOCATION.to_vec(),
                            &consts::POINT_LIGHT_COLOR,
                            &consts::AMBIENT_REFLECT,
                            &consts::DIFFUSE_REFLECT,
                            &consts::SPECULAR_REFLECT
                        );
    
                        polygons = Matrix::new(0, 0);
                    }
                    Rule::TORUS_SDDDDD => {
                        let mut command_contents = command.into_inner();
                        let lighting_constants = constants_store.get(command_contents.next().unwrap().as_str()).expect("Unable to get lighting constants");
                        polygons.add_torus(
                            command_contents.next().unwrap().as_str().parse().expect(error_message),
                            command_contents.next().unwrap().as_str().parse().expect(error_message),
                            command_contents.next().unwrap().as_str().parse().expect(error_message),
                            command_contents.next().unwrap().as_str().parse().expect(error_message),
                            command_contents.next().unwrap().as_str().parse().expect(error_message),
                            consts::STEP_3D
                        );
                        polygons.multiply_matrixes(cstack.last().unwrap());
                        screen.draw_polygons(
                            &polygons,
                            &color,
                            &mut consts::VIEW.to_vec(),
                            &consts::AMBIENT_COLOR,
                            &mut consts::POINT_LIGHT_LOCATION.to_vec(),
                            &consts::POINT_LIGHT_COLOR,
                            &lighting_constants.ambient_reflect,
                            &lighting_constants.diffuse_reflect,
                            &lighting_constants.specular_reflect
                        );
    
                        polygons = Matrix::new(0, 0);
                    }
                    Rule::TORUS_DDDDD => {
                        let mut command_contents = command.into_inner();
                        polygons.add_torus(
                            command_contents.next().unwrap().as_str().parse().expect(error_message),
                            command_contents.next().unwrap().as_str().parse().expect(error_message),
                            command_contents.next().unwrap().as_str().parse().expect(error_message),
                            command_contents.next().unwrap().as_str().parse().expect(error_message),
                            command_contents.next().unwrap().as_str().parse().expect(error_message),
                            consts::STEP_3D
                        );
                        polygons.multiply_matrixes(cstack.last().unwrap());
                        screen.draw_polygons(
                            &polygons,
                            &color,
                            &mut consts::VIEW.to_vec(),
                            &consts::AMBIENT_COLOR,
                            &mut consts::POINT_LIGHT_LOCATION.to_vec(),
                            &consts::POINT_LIGHT_COLOR,
                            &consts::AMBIENT_REFLECT,
                            &consts::DIFFUSE_REFLECT,
                            &consts::SPECULAR_REFLECT
                        );
    
                        polygons = Matrix::new(0, 0);
                    }
                    Rule::DISPLAY => {
                        if frames.len() <= 1{
                            screen.display();
                        }
                    }
                    Rule::SAVE_S => {
                        if frames.len() <= 1{
                            let mut command_contents = command.into_inner();
                            let filename = command_contents.next().unwrap().as_str();
                            screen.create_file(filename);
                            Command::new("magick")
                                .arg("convert")
                                .arg(filename)
                                .arg(filename)
                                .spawn()
                                .expect("failed to convert image to desired format");
                        }
                    }
                    Rule::LINE_DDDDDD => {
                        let mut command_contents = command.into_inner();
                        edges.add_edge(
                            command_contents.next().unwrap().as_str().parse().expect(error_message),
                            command_contents.next().unwrap().as_str().parse().expect(error_message),
                            command_contents.next().unwrap().as_str().parse().expect(error_message),
                            command_contents.next().unwrap().as_str().parse().expect(error_message),
                            command_contents.next().unwrap().as_str().parse().expect(error_message),
                            command_contents.next().unwrap().as_str().parse().expect(error_message),
                        );
                        edges.multiply_matrixes(cstack.last().unwrap());
                        screen.draw_lines(&edges, &color);
        
                        edges = Matrix::new(0, 0);
                    }
                    Rule::EOI | Rule::VARY_SDDDD | Rule::BASENAME_S | Rule::BASENAME | Rule::FRAMES_D => {}
                    _ => {
                        println!("{:?} was not implemented :/", command.as_rule());
                    }
                }
            }
        }
        if frames.len() > 1{
            render_reset_image_canvas(&basename, frame_num, &mut screen, &mut edges, &mut polygons, &mut cstack);
        }
    }
    if frames.len() > 1{
        make_animation(basename);
    }
}

fn render_reset_image_canvas(filename: &str, frame_num: usize, screen: &mut Image, edges: &mut Matrix, polygons: &mut Matrix, cstack: &mut Vec<Matrix>){
    let filename = "animation/".to_owned() + &filename + &*format!("{:04}", frame_num) + ".ppm";
    screen.create_file(&*filename);
    println!("Rendering {}...", filename);
    screen.clear();
    *edges = Matrix::new(0, 0);
    *polygons = Matrix::new(0, 0);
    *cstack = vec![Matrix::new(0, 0); 0];
    cstack.push(Matrix::identity());
}

fn clean_animation_directory(){
    Command::new("make")
        .arg("clean_anim")
        .spawn()
        .expect("ERROR: unable to delete files");
}