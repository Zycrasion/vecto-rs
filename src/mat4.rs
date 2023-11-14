use std::{ops::Mul, fmt::Display};

use crate::vec::{Vector, Vector4, VectorTrait};

//  https://en.wikipedia.org/wiki/Transformation_matrix#Examples_in_3D_computer_graphics

#[derive(Copy, Clone, Debug, Default)]
/// Matrice of order 4 x 4
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

impl Display for Mat4
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for ele in self.contents
        {
            writeln!(f, "{ele}")?;
        }
        Ok(())
    }
}