extern crate gl;
extern crate glfw;

use std::ops::Deref;
use vatnar_linalg::Point2;

pub struct Polygon(pub Vec<Point2<f32>>);
impl Deref for Polygon {
    type Target = Vec<Point2<f32>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
/// Creates a `Polygon` from a list of `Point2::new(x, y)` expressions.
///
/// # Example
/// ```
/// use shader_preview::polygon;
///
/// let poly = polygon![
///     0.0, 0.0,
///     1.0, 0.0,
///     1.0, 1.0,
///     0.0, 1.0,
/// ];
/// assert_eq!(poly.len(), 4);
/// ```
#[macro_export]
macro_rules! polygon {
    ( $( $x:expr, $y:expr ),* $(,)? ) => {
        $crate::Polygon(vec![
            $(
                vatnar_linalg::Point2::<f32>::new($x, $y)
            ),*
        ])
    };
}

#[macro_export]
macro_rules! polygon_from {
    ($expr:expr) => {
        Polygon($expr.collect::<Vec<_>>())
    };
}

#[derive(Debug, Clone, Copy)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
}

impl From<Color> for (f32, f32, f32) {
    fn from(color: Color) -> Self {
        (color.r, color.g, color.b)
    }
}

impl From<(f32, f32, f32)> for Color {
    fn from(value: (f32, f32, f32)) -> Self {
        Color {
            r: value.0,
            g: value.1,
            b: value.2,
        }
    }
}
pub struct Mesh {
    vao: u32,
    _vbo: u32,
    vertex_count: usize,
    draw_mode: u32,
    pub color: Color,
}

// TODO from points nd lines and stuff
impl Mesh {
    pub fn from_polygon(polygon: Polygon, draw_mode: gl::types::GLenum, color: Color) -> Self {
        let vertices: Vec<f32> = polygon.iter().flat_map(|p| [p.x, p.y]).collect();

        // 1. Generate and bind a Vertex Array Object VAO
        let mut vao = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao); // Store ID to generated VAO in variable
            gl::BindVertexArray(vao); // Bind the VAO so opengl knows we setting up
        }

        // 2. Generate and bind a vertex buffer object VBO
        let mut vbo = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo); // Generate one vbo
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo); // Bind it as an array buffer
            gl::BufferData(
                // Upload the data to GPU
                gl::ARRAY_BUFFER,
                size_of_val(&vertices) as isize, // Size of data in bytes
                vertices.as_ptr() as *const _,   // Pointer to data
                gl::DYNAMIC_DRAW,                // Tell gpu we  update often
            );
        }

        // 3. Describe how the data in vbo is laid out
        unsafe {
            gl::VertexAttribPointer(
                0,                           // Attribute index (loc 0 in shader)
                2,                           // 2 Components per vertex (x, y)
                gl::FLOAT,                   // Data type is float
                gl::FALSE,                   // Dont normalize
                2 * size_of::<f32>() as i32, // Stride, 2 floats per vertex
                std::ptr::null(),            // offset: start at the beginning
            );
            gl::EnableVertexAttribArray(0); // enable specified attribute
        }
        Mesh {
            vao,
            _vbo: vbo,
            vertex_count: vertices.len() / 2,
            draw_mode,
            color,
        }
    }
    pub fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(self.draw_mode, 0, self.vertex_count as i32);
        }
    }
}
