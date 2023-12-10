use std::{ops::Mul, fmt::Display};

use crate::{vec::{Vector, Vector4, VectorTrait}, trig::to_radians};

//  https://en.wikipedia.org/wiki/Transformation_matrix#Examples_in_3D_computer_graphics

#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
/// Matrice of order 4 x 4
/// 
/// Stored As Row Major
pub struct Mat4
{
    contents : [f32; 4 * 4]
}

impl Mat4
{
    /// Create a new Matrice
    pub fn new() -> Self
    {
        Self
        {
            contents : [0.; 4 * 4]
        }
    }

    /// Create an identity Matrice
    pub fn identity() -> Self
    {
        let mut a = Self::new();
        for i in 0..=3
        {
            a.change(i, i, 1.);
        }
        a
    }

    /// Checks if any of the elements are not a number
    pub fn is_nan(&self) -> bool
    {
        for ele in self.contents
        {
            if ele.is_nan()
            {
                return true;
            }
        }
        return false;
    }

    /// Transposes the matrice
    /// Doesn't change Self
    pub fn transpose(&self) -> Self
    {
        let mut b = Self::new();
        for i in 0..16
        {
            let (x, y) = Self::coords(i);
            b.change(x, y, self.get(y, x));
        }
        b
    }
    
    /// Change element at x, y (0 indexed)
    pub fn change(&mut self, x : usize, y : usize, val : f32)
    {
        let index = Self::index(x, y);
        self.contents[index] = val;
    }

    /// Get element at x, y (0 indexed)
    pub fn get(&self, x : usize, y : usize) -> f32
    {
        let index = Self::index(x, y);
        self.contents[index]
    }
    
    /// Get the index for an element located at x, y (0 indexed)
    pub fn index(x : usize, y : usize) -> usize
    {
        assert!(x < 4);
        assert!(y < 4);

        x + y * 4
    }

    /// Get the x, y (ij) location of an index
    pub fn coords(i : usize) -> (usize, usize)
    {
        (i % 4, i / 4)
    }

    /// Get a column
    pub fn get_col(&self, x : usize) -> Vector4
    {
        Vector4::new4(self.get(x, 0),self.get(x, 1),self.get(x, 2),self.get(x, 3))
    }

    /// Get a row
    pub fn get_row(&self, y : usize) -> Vector4
    {
        Vector4::new4(self.get(0, y),self.get(1, y),self.get(2, y),self.get(3, y))
    }
    
    /// Get Matrice As Row Major
    pub fn get_row_major(&self) -> Mat4
    {
        return self.clone()
    }

    /// Get Matrix As Column Major
    pub fn get_column_major(&self) -> Mat4
    {
        return self.transpose()
    }

    /// From Array (row major)
    pub const fn from_array(mat : [f32; 4 * 4]) -> Mat4
    {
        Mat4 { contents: mat }
    }

    /// From Vector4
    pub fn from_vec4(vec : &Vector4) -> Mat4
    {
        let mut mat = Mat4::new();
        mat.change(0, 0, vec.x);
        mat.change(1, 1, vec.y);
        mat.change(2, 2, vec.z);
        mat.change(3, 3, vec.w);
        mat
    }

    /// Create a rotation Matrix
    /// 
    /// http://www.songho.ca/opengl/gl_matrix.html
    pub fn rotation_matrix(angle : f32, axis : Vector) -> Mat4
    {
        let s = angle.sin();
        let c = angle.cos();
        let c_1 = 1. - c;

        let x = axis.x;
        let y = axis.y;
        let z = axis.z;

        Mat4::from_array([
            c_1 * x * x + c     , c_1 * x * y - s * z   , c_1 * x * z + s * y, 0.,
            c_1 * x * y + s * z , c_1 * y * y + c       , c_1 * y * z - s * x, 0.,
            c_1 * x * z - s * y , c_1 * y * z + s * x   , c_1 * z * z + c    , 0.,
            0.                  ,                  0.,                  0.   , 1.,
        ])
    }

    /// Rotate current matrix
    pub fn rotate(&mut self, angle : f32, axis : Vector)
    {
        *self = *self * Mat4::rotation_matrix(angle, axis);
    }

    /// Translate current matrix
    pub fn translate(&mut self, pos : Vector)
    {
        *self = *self * Mat4::new_translation(pos);
    }

    /// Translation Matrix
    pub fn new_translation(pos : Vector) -> Mat4
    {
        let mut transform = Mat4::identity();
        transform.change(3, 0, pos.x);
        transform.change(3, 1, pos.y);
        transform.change(3, 2, pos.z);
        
        transform
    }

    /// Perspective Matrix
    /// 
    /// fov is in degrees
    /// 
    /// https://www.scratchapixel.com/lessons/3d-basic-rendering/perspective-and-orthographic-projection-matrix/building-basic-perspective-projection-matrix.html
    pub fn new_perspective_matrix(width: f32, height: f32, fov : f32, near : f32, far : f32) -> Mat4
    {
        let aspect = width / height;
        let z_range = near - far;
        let tan_half_fov = to_radians(fov / 2.0).tan();
        let mat = Mat4::from_array(
            [
                1.0 / (tan_half_fov * aspect), 0., 0., 0.,
                0., 1. / tan_half_fov, 0., 0.,
                0., 0., (-near - far) / z_range, 2. * far * near / z_range,
                0., 0., 1., 0.
            ]
        );
        mat
    }

    /// Creates a matrix looking at vector to from Vector from
    pub fn look_at_rh(from : &Vector, to : &Vector, up : &Vector) -> Mat4
    {
        let forward = (*from - *to).normalized();
        let right = Vector::cross(up, &forward).normalized();
        let new_up = Vector::cross(&forward, &right);

        Mat4::from_array(
            [
                right.x, right.y, right.z, 0.,
                new_up.x, new_up.y, new_up.z, 0.,
                forward.x, forward.y, forward.z, 0.,
                from.x, from.y, from.z, 1.,
            ]
        )
    }

    /// get contents in array form
    pub fn get_contents(&self) -> [f32; 4 * 4]
    {
        self.contents
    }

    /// to vec4
    pub fn to_vec4(&self) -> Vector4
    {
        let mut vec4 = Vector4::default();
        vec4.x = self.get_row(0).sum();
        vec4.y = self.get_row(1).sum();
        vec4.z = self.get_row(2).sum();
        vec4.w = self.get_row(3).sum();
        vec4
    }

    /// to vec3
    pub fn to_vec3(&self) -> Vector
    {
        let mut vec3 = Vector::default();
        vec3.x = self.get_row(0).sum();
        vec3.y = self.get_row(1).sum();
        vec3.z = self.get_row(2).sum();
        vec3
    }
}


impl Mul for Mat4
{
    type Output = Mat4;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut c = Self::new();
        for i in 0..16usize
        {
            let (col_index, row_index) = Self::coords(i);
            
            let row = self.get_row(row_index);
            let col =  rhs.get_col(col_index);

            let result = row * col;
            let result = result.x + result.y + result.z + result.w;
            c.change(col_index, row_index, result);
        }
        return c;
    }
}

impl Mul<Vector4> for Mat4
{
    type Output = Mat4;

    fn mul(self, rhs: Vector4) -> Self::Output {
        let rhs = Mat4::from_vec4(&rhs);
        rhs * self
    }   
}

impl Display for Mat4
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut i = 1;
        for ele in self.contents
        {
            write!(f, "{ele} ")?;
            if i % 4 == 0
            {
                writeln!(f, "")?;
            }
            i += 1;
        }
        Ok(())
    }
}