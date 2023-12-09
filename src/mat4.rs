use std::{ops::Mul, fmt::Display, f32::consts::PI};

use crate::vec::{Vector, Vector4, VectorTrait};

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

    /// Translate the matrice by a vector
    pub fn translate(&mut self, translation: Vector)
    {
        self.change(3, 0, translation.x);
        self.change(3, 1, translation.y);
        self.change(3, 2, translation.z);
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

    /// Transform Matrix
    pub fn new_transform(pos : Vector) -> Mat4
    {
        let mut transform = Mat4::new();
        transform.change(0, 3, pos.x);
        transform.change(1, 3, pos.y);
        transform.change(2, 3, pos.z);
        transform.change(3, 3, 1.);
        
        transform
    }

    /// Perspective Matrix
    /// https://www.scratchapixel.com/lessons/3d-basic-rendering/perspective-and-orthographic-projection-matrix/building-basic-perspective-projection-matrix.html
    pub fn set_perspective_matrix(&mut self, fov : f32, near : f32, far : f32)
    {
        let scale = 1. / (fov * 0.5 * PI / 180.).tan();
        self.change(0, 0, scale);
        self.change(1, 1, scale);
        self.change(2, 2, -far / (far - near));
        self.change(3, 2, -far * near / (far - near));
        self.change(2, 3, -1.);
        self.change(3, 3, 0.);
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
                from.x, from.y, from.z, 0.,
            ]
        )
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