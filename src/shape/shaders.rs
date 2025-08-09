use gl::types::{GLenum, GLuint};
use std::ffi::CString;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;
use std::ptr::null;

/// This struct represents a shader used in a shape.
/// Those shaders are a file, located inside /shaders folder, they are process by reading the file, and compiling it in the opengl format.
/// # Fields :
/// - name : the name of the shader
/// - content : the string literal contained in the shader file
pub struct Shader {
    pub name: String,
    content: String,
}

impl Shader {
    /// The constructor reads the file in /shaders based on the name provided, then builds an instance of the struct with its content
    /// # Arguments :
    /// - name : the name of the shader (used to determine the file location)
    /// # Effects :
    /// Returns a Shader struct with the file content inside
    /// # Errors :
    /// May return an error (using rust's error system) if there were an error when reading the file (not found, encoding errors, etc..)
    pub fn new<T: ToString>(name: T) -> io::Result<Shader> {
        let name = name.to_string();
        let mut file = File::open(Path::new(&format!("shaders/{}.glsl", name)))?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let content = content.trim().to_string();

        Ok(Shader { name, content })
    }

    /// This method needs to be called after the shader is created via the constructor.
    /// It takes the content red by the constructor and stored in the content field, before compiling it to the opengl standard
    /// It's also performing some basic error checking by calling panic! (bad, need to rewrite)
    pub fn init_shader(&self, shader_type: GLenum) -> GLuint {
        unsafe {
            let shader = gl::CreateShader(shader_type);
            let c_str =
                CString::new(self.content.clone()).expect("Shader content contains null bytes");
            gl::ShaderSource(shader, 1, &c_str.as_ptr(), null());
            gl::CompileShader(shader);

            let mut success = gl::FALSE as gl::types::GLint;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as gl::types::GLint {
                let mut len = 0;
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
                let mut buffer = Vec::with_capacity(len as usize);
                buffer.set_len((len as usize) - 1);
                gl::GetShaderInfoLog(
                    shader,
                    len,
                    std::ptr::null_mut(),
                    buffer.as_mut_ptr() as *mut gl::types::GLchar,
                );
                panic!(
                    "Erreur de compilation du shader {}: {}",
                    self.name,
                    String::from_utf8_lossy(&buffer)
                );
            }

            shader
        }
    }
}
