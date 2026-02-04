use crate::objects::shape::mesh::Vertex;
use crate::objects::shape::Shape;
use std::collections::HashMap;
use crate::objects::material::Material;

impl Shape {
    pub fn from_obj_file(name: String, obj_path: &str, material: Material) -> Result<Self, String> {
        let content = std::fs::read_to_string(obj_path)
            .map_err(|e| format!("Erreur lors de la lecture de '{obj_path}': {e}"))?;
        let mut temp_positions = Vec::new();
        let mut temp_uvs = Vec::new();
        let mut temp_normals = Vec::new();

        let mut final_vertices = Vec::new();
        let mut final_indices = Vec::new();

        let mut vertex_to_index = HashMap::new();

        for line in content.lines() {
            let mut tokens = line.split_whitespace();
            let Some(kind) = tokens.next() else { continue };

            match kind {
                "v" => {
                    let pos = Self::parse_f32_3(&mut tokens)?;
                    temp_positions.push(pos);
                }
                "vt" => {
                    let uv = Self::parse_f32_2(&mut tokens)?;
                    temp_uvs.push(uv);
                }
                "vn" => {
                    let norm = Self::parse_f32_3(&mut tokens)?;
                    temp_normals.push(norm);
                }
                "f" => {
                    let mut face_indices = Vec::new();
                    for token in tokens {
                        let triplet = Self::parse_triplet(token);

                        let &mut idx = vertex_to_index.entry(triplet).or_insert_with(|| {
                            let i = final_vertices.len() as u32;

                            let pos = temp_positions
                                .get(triplet.0 as usize)
                                .copied()
                                .unwrap_or([0.0; 3]);
                            let uv = triplet
                                .1
                                .and_then(|i| temp_uvs.get(i as usize))
                                .copied()
                                .unwrap_or([0.0; 2]);
                            let norm = triplet.2.and_then(|i| temp_normals.get(i as usize)).copied().unwrap_or([0.0; 3]);

                            final_vertices.push(Vertex {
                                position: pos,
                                color: [-1.0, -1.0, -1.0],
                                tex_coords: uv,
                                normal: norm,
                            });
                            i
                        });
                        face_indices.push(idx);
                    }

                    for i in 1..(face_indices.len() - 1) {
                        final_indices.push(face_indices[0]);
                        final_indices.push(face_indices[i]);
                        final_indices.push(face_indices[i + 1]);
                    }
                }
                _ => {}
            }
        }

        Ok(Shape::new_from_vertex(
            name,
            final_vertices,
            if final_indices.is_empty() {
                None
            } else {
                Some(final_indices)
            },
            material,
        ))
    }

    fn parse_f32_3(tokens: &mut std::str::SplitWhitespace) -> Result<[f32; 3], String> {
        let mut next = || {
            tokens
                .next()
                .and_then(|t| t.parse().ok())
                .ok_or("Format float invalide")
        };
        Ok([next()?, next()?, next()?])
    }

    fn parse_f32_2(tokens: &mut std::str::SplitWhitespace) -> Result<[f32; 2], String> {
        let mut next = || {
            tokens
                .next()
                .and_then(|t| t.parse().ok())
                .ok_or("Format float invalide")
        };
        Ok([next()?, next()?])
    }

    // Parse "1/2/3" or "1//3" or "1"
    fn parse_triplet(token: &str) -> (u32, Option<u32>, Option<u32>) {
        let parts: Vec<&str> = token.split('/').collect();

        let v_idx = parts[0].parse::<u32>().unwrap_or(1) - 1;
        let vt_idx = parts
            .get(1)
            .and_then(|&s| s.parse::<u32>().ok())
            .map(|i| i - 1);
        let vn_idx = parts
            .get(2)
            .and_then(|&s| s.parse::<u32>().ok())
            .map(|i| i - 1);

        (v_idx, vt_idx, vn_idx)
    }
}
