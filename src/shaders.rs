use gl::types::{GLenum, GLuint};
use std::ffi::CString;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;
use std::ptr::null;

pub struct Shader {
    pub name: String,
    content: String,
}

impl Shader {
    pub fn new<T: ToString>(name: T) -> io::Result<Shader> {
        let name = name.to_string();
        let mut file = File::open(Path::new(&format!("shaders/{}.glsl", name)))?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let content = content.trim().to_string();

        Ok(Shader { name, content })
    }

    pub fn init_shader(&self, shader_type: GLenum) -> GLuint {
        unsafe {
            let shader = gl::CreateShader(shader_type);
            let c_str =
                CString::new(self.content.clone()).expect("Shader content contains null bytes");
            gl::ShaderSource(shader, 1, &c_str.as_ptr(), null());
            gl::CompileShader(shader);

            shader
        }
    }
}
