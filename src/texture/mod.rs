use gl::types::{GLint, GLsizei, GLuint};
use image::ImageReader;
use std::ffi::c_void;

#[derive(Clone, Copy)]
pub struct Texture {
    pub width: u32,
    pub height: u32,
    pub nr_channels: u8,
    pub texture_id: GLuint,
}

impl Texture {
    pub fn new(path: &str) -> Result<Self, String> {
        let img = ImageReader::open(path)
            .expect("err when opening file")
            .decode()
            .expect("err when decode texture")
            .flipv();

        let format = match img.color() {
            image::ColorType::Rgb8 => gl::RGB,
            image::ColorType::Rgba8 => gl::RGBA,
            _ => gl::RGB,
        };

        let mut texture_id: GLuint = 0;

        unsafe {
            gl::GenTextures(1, &mut texture_id);

            gl::BindTexture(gl::TEXTURE_2D, texture_id);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                format as GLint,
                img.width() as GLsizei,
                img.height() as GLsizei,
                0,
                format,
                gl::UNSIGNED_BYTE,
                img.as_bytes().as_ptr() as *const c_void,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        Ok(Texture {
            width: img.width(),
            height: img.height(),
            nr_channels: img.color().channel_count(),
            texture_id,
        })
    }
    pub fn from_gltf_image(gltf_image: &gltf::image::Data) -> Result<Self, String> {
        let (format, nr_channels) = match gltf_image.format {
            gltf::image::Format::R8G8B8 => (gl::RGB, 3),
            gltf::image::Format::R8G8B8A8 => (gl::RGBA, 4),
            gltf::image::Format::R8 => (gl::RED, 1),
            gltf::image::Format::R8G8 => (gl::RG, 2),
            _ => (gl::RGBA, 4), // Fallback raisonnable
        };

        let mut texture_id: GLuint = 0;

        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR_MIPMAP_LINEAR as i32,
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                format as GLint,
                gltf_image.width as GLsizei,
                gltf_image.height as GLsizei,
                0,
                format,
                gl::UNSIGNED_BYTE,
                gltf_image.pixels.as_ptr() as *const c_void,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        Ok(Texture {
            width: gltf_image.width,
            height: gltf_image.height,
            nr_channels,
            texture_id,
        })
    }

    pub fn default_white() -> Self {
        let mut texture_id: GLuint = 0;
        let white_pixel: [u8; 4] = [255, 255, 255, 255];

        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as GLint,
                1,
                1,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                white_pixel.as_ptr() as *const c_void,
            );
        }

        Texture {
            width: 1,
            height: 1,
            nr_channels: 4,
            texture_id,
        }
    }
}
