use crate::shape::Shape;
use std::io::Read;

impl<'a> Shape<'a> {
    pub fn from_obj_file(
        name: &'a str,
        obj_path: &str,
        texture_path: &str,
    ) -> Result<Self, String> {
        const DEFAULT_COLOR: [f32; 3] = [0.0; 3];
        const DEFAULT_UV: [f32; 2] = [0.0; 2];

        fn parse_vertex<'s>(mut parts: impl Iterator<Item = &'s str>) -> Option<[f32; 3]> {
            let x = parts.next()?.parse::<f32>().ok()?;
            let y = parts.next()?.parse::<f32>().ok()?;
            let z = parts.next()?.parse::<f32>().ok()?;
            Some([x, y, z])
        }

        fn parse_face_index(token: &str) -> Option<u32> {
            let first = token.split('/').next()?;
            let one_based = first.parse::<u32>().ok()?;
            one_based.checked_sub(1)
        }

        let content = std::fs::read_to_string(obj_path)
            .map_err(|e| format!("Erreur lors de la lecture de '{obj_path}': {e}"))?;

        let mut positions: Vec<[f32; 3]> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        for line in content.lines() {
            let mut tokens = line.split_whitespace();
            let Some(kind) = tokens.next() else { continue };

            match kind {
                "v" => {
                    if let Some(pos) = parse_vertex(tokens) {
                        positions.push(pos);
                    }
                }
                "f" => {
                    let mut face = [0u32; 3];
                    let mut ok = true;

                    for (i, t) in tokens.take(3).enumerate() {
                        if let Some(idx) = parse_face_index(t) {
                            face[i] = idx;
                        } else {
                            ok = false;
                            break;
                        }
                    }

                    if ok {
                        indices.extend_from_slice(&face);
                    }
                }
                _ => {}
            }
        }

        let vertices_with_attrs = positions
            .iter()
            .copied()
            .map(|pos| (pos, DEFAULT_COLOR, DEFAULT_UV))
            .collect::<Vec<([f32; 3], [f32; 3], [f32; 2])>>();

        let indices_slice = if indices.is_empty() {
            None
        } else {
            Some(Box::from(indices.as_slice()))
        };

        Ok(Shape::new(
            name,
            Box::from(vertices_with_attrs),
            indices_slice,
            texture_path,
        ))
    }
}
