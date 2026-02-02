use crate::texture::Texture;

struct Material {
    pub ambient: Texture,
    pub specular: Texture,
    pub shininess: u32,
}

impl Material {
    pub fn new(ambient_path: &str, specular_path: &str, shininess: u32) -> Self {
        Self {
            ambient: Texture::new(ambient_path).unwrap(),
            specular: Texture::new(specular_path).unwrap(),
            shininess,
        }
    }

    pub fn bind() {

    }
}
