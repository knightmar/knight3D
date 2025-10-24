use crate::shape::Shape;
use std::fs::File;
use std::io::Read;

impl<'a> Shape<'a> {
    pub fn from_obj_file<'b>(
        name: &'a str,
        file: &str,
        texture_path: &str,
    ) -> Result<Self, String> {
        let mut vertices = Vec::<[f32; 3]>::new();

        if let Ok(mut file) = File::open(file) {
            let mut content = String::new();
            if file.read_to_string(&mut content).is_err() {
                return Err("Error while reading file".to_string());
            }

            content.lines().for_each(|line| {
                let line_splited: Vec<&str> = line.split_whitespace().collect();
                if let Some(&"v") = line_splited.get(0) {
                    if line_splited.len() >= 4 {
                        if let (Ok(x), Ok(y), Ok(z)) = (
                            line_splited[1].parse::<f32>(),
                            line_splited[2].parse::<f32>(),
                            line_splited[3].parse::<f32>(),
                        ) {
                            vertices.push([x, y, z]);
                        }
                    }
                }
            });
        }

        let color = [0.0f32; 3];
        let texture = [0.0f32; 2];

        let x = vertices
            .iter()
            .map(|&pos| (pos, color, texture))
            .collect::<Vec<([f32; 3], [f32; 3], [f32; 2])>>();
        return Ok(Shape::new(name, Box::from(x), None, texture_path));
    }
}
