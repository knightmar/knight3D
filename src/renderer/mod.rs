use crate::shape::Shape;

mod color_renderer;

trait Renderer<'a> {
    fn init_buffers(&mut self);

    fn render(&self);
    fn get_obj_list(&self) -> Vec<Shape<'a>>;

    /// This method takes the name of the shaders and compiles them into a program shader before storing it in the struct's field
    /// # Arguments: names of the shaders
    /// # Effect: Returns a ColorRenderer completed with a new shader program
    fn init(vertex_shader_name: &str, fragment_shader_name: &str) -> Self;
}
