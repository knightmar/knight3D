use crate::shape::Shape;
use std::fs::File;
use std::io::Read;

impl<'a> Shape<'a> {
    pub fn from_obj_file<'b>(name: &'a str, file: &str, texture_path: &str) -> Result<Self, String> {
        let mut vertices = Vec::<[f32; 3]>::new();

        if let Ok(mut file) = File::open(file) {
            let mut content = String::new();
            if file.read_to_string(&mut content).is_err() {
                return Err("Error while reading file".to_string());
            }

            content.split("\n").for_each(|line| {
                let line_splited = line.split(" ").collect::<Vec<&str>>();
                if line_splited[0].eq("v") {
                    vertices.push([
                        line_splited[1].parse::<f32>().unwrap(),
                        line_splited[2].parse::<f32>().unwrap(),
                        line_splited[3].parse::<f32>().unwrap(),
                    ]);

                    println!("Vertice found : {:#?}", vertices.last());
                }
            });
        }

        let color = [0.0f32; 3];
        let texture = [0.0f32; 2];

        let x = vertices
            .iter()
            .map(|x1| {
                return (x1.clone(), color.clone(), texture.clone());
            })
            .collect::<Vec<([f32; 3], [f32; 3], [f32; 2])>>();
        return Ok(Shape::new(name, Box::from(x), None, texture_path));
    }
}
