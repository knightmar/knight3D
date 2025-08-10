use gl::types::{GLint, GLsizei, GLuint};
use image::ImageReader;
use std::ffi::c_void;

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
            .expect("err when decode texture");
        let mut texture_id: GLuint = 0;

        unsafe {
            gl::GenTextures(1, &mut texture_id);

            gl::BindTexture(gl::TEXTURE_2D, texture_id);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as GLint,
                img.width() as GLsizei,
                img.height() as GLsizei,
                0,
                gl::RGB,
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
}
