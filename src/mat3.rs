use std::{ops::Mul, fmt::Display};

use crate::vec::{Vector, VectorTrait};

//  https://en.wikipedia.org/wiki/Transformation_matrix#Examples_in_3D_computer_graphics

#[derive(Copy, Clone, Debug, Default, PartialEq)]
/// Matrice of order 3 x 3
/// 
/// Stored As Row Major
pub struct Mat3
{
    contents : [f64; 3 * 3]
}

impl Mat3
{
    /// Create a new Matrice
    pub fn new() -> Self
    {
        Self
        {
            contents : [0.; 3 * 3]
        }
    }

    /// Create an identity Matrice
    pub fn identity() -> Self
    {
        let mut a = Self::new();
        for i in 0..=2
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
        for i in 0..9
        {
            let (x, y) = Self::coords(i);
            b.change(x, y, self.get(y, x));
        }
        b
    }
    
    /// Change element at x, y (0 indexed)
    pub fn change(&mut self, x : usize, y : usize, val : f64)
    {
        let index = Self::index(x, y);
        self.contents[index] = val;
    }

    /// Get element at x, y (0 indexed)
    pub fn get(&self, x : usize, y : usize) -> f64
    {
        let index = Self::index(x, y);
        self.contents[index]
    }
    
    /// Get the index for an element located at x, y (0 indexed)
    pub fn index(x : usize, y : usize) -> usize
    {
        assert!(x < 3);
        assert!(y < 3);

        x + y * 3
    }

    /// Get the x, y (ij) location of an index
    pub fn coords(i : usize) -> (usize, usize)
    {
        (i % 3, i / 3)
    }

    /// Get a column
    pub fn get_col(&self, x : usize) -> Vector
    {
        Vector::new3(self.get(x, 0),self.get(x, 1),self.get(x, 2))
    }

    /// Get a row
    pub fn get_row(&self, y : usize) -> Vector
    {
        Vector::new3(self.get(0, y),self.get(1, y),self.get(2, y))
    }
    
    /// Get Matrice As Row Major
    pub fn get_row_major(&self) -> Mat3
    {
        return self.clone()
    }

    /// Get Matrix As Column Major
    pub fn get_column_major(&self) -> Mat3
    {
        return self.transpose()
    }

    /// From Array (row major)
    pub const fn from_array(mat : [f64; 3 * 3]) -> Mat3
    {
        Mat3 { contents: mat }
    }

    /// From Vector
    pub fn from_vec(vec : &Vector) -> Mat3
    {
        let mut mat = Mat3::new();
        mat.change(0, 0, vec.x);
        mat.change(1, 1, vec.y);
        mat.change(2, 2, vec.z);
        mat
    }

    /// Create a rotation Matrix
    /// 
    /// http://www.songho.ca/opengl/gl_matrix.html
    pub fn rotation_matrix(angle : f64, axis : Vector) -> Mat3
    {
        let s = angle.sin();
        let c = angle.cos();
        let c_1 = 1. - c;

        let x = axis.x;
        let y = axis.y;
        let z = axis.z;

        Mat3::from_array([
            c_1 * x * x + c     , c_1 * x * y - s * z   , c_1 * x * z + s * y,
            c_1 * x * y + s * z , c_1 * y * y + c       , c_1 * y * z - s * x,
            c_1 * x * z - s * y , c_1 * y * z + s * x   , c_1 * z * z + c    ,
        ])
    }

    /// Rotate current matrix
    pub fn rotate(&mut self, angle : f64, axis : Vector)
    {
        *self = *self * Mat3::rotation_matrix(angle, axis);
    }

    /// Translate current matrix
    pub fn translate(&mut self, pos : Vector)
    {
        *self = *self * Mat3::new_translation(pos);
    }

    /// Translation Matrix
    pub fn new_translation(pos : Vector) -> Mat3
    {
        let mut transform = Mat3::identity();
        transform.change(3, 0, pos.x);
        transform.change(3, 1, pos.y);
        transform.change(3, 2, pos.z);
        transform
    }

    /// get contents in array form
    pub fn get_contents(&self) -> [f64; 3 * 3]
    {
        self.contents
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

    /// Scales matrix
    pub fn scale(&mut self, vec : Vector)
    {
        *self = *self * vec;
    }
}


impl Mul for Mat3
{
    type Output = Mat3;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut c = Self::new();
        for i in 0..9usize
        {
            let (col_index, row_index) = Self::coords(i);
            
            let row = self.get_row(row_index);
            let col =  rhs.get_col(col_index);

            let result = row * col;
            let result = result.x + result.y + result.z;
            c.change(col_index, row_index, result);
        }
        return c;
    }
}

impl Mul<Vector> for Mat3
{
    type Output = Mat3;

    fn mul(self, rhs: Vector) -> Self::Output {
        let rhs = Mat3::from_vec(&rhs);
        rhs * self
    }   
}

impl Display for Mat3
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