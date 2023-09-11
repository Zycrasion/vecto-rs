#[macro_export]
macro_rules! vec_vec_op
{
    ($name:ident, $method:ident, $op:tt) =>
    {
        impl ops::$name for Vector
        {
            type Output = Vector;

            fn $method(self, rhs : Self) -> Self::Output
            {
                Vector::new3(self.x $op rhs.x, self.y $op rhs.y, self.z $op rhs.z)
            }
        }
    }
}

#[macro_export]
macro_rules! vec_vec_assign_op
{
    ($name:ident, $method:ident, $op:tt) =>
    {
        impl ops::$name for Vector
        {
            fn $method(&mut self, rhs : Self)
            {
                self.x $op rhs.x;
                self.y $op rhs.y;
                self.z $op rhs.z;
            }
        }
    }
}

#[macro_export]
macro_rules! vec_f32_op
{
    ($name:ident, $method:ident, $op:tt) =>
    {
        impl ops::$name<f32> for Vector
        {
            type Output = Vector;

            fn $method(self, rhs : f32) -> Self::Output
            {
                Vector::new3(self.x $op rhs, self.y $op rhs, self.z $op rhs)
            }
        }
    }
}

#[macro_export]
macro_rules! vec_f32_assign_op
{
    ($name:ident, $method:ident, $op:tt) =>
    {
        impl ops::$name<f32> for Vector
        {
            fn $method(&mut self, rhs : f32)
            {
                self.x $op rhs;
                self.y $op rhs;
                self.z $op rhs;
            }
        }
    }
}