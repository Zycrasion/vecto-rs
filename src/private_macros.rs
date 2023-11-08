#[doc(hidden)]
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

#[doc(hidden)]
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

#[doc(hidden)]
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

#[doc(hidden)]
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

#[doc(hidden)]
#[macro_export]
macro_rules! vec4_vec_op
{
    ($name:ident, $method:ident, $op:tt) =>
    {
        impl ops::$name for Vector4
        {
            type Output = Vector4;

            fn $method(self, rhs : Self) -> Self::Output
            {
                Vector4::new4(self.x $op rhs.x, self.y $op rhs.y, self.z $op rhs.z, self.w $op rhs.w)
            }
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! vec4_vec_assign_op
{
    ($name:ident, $method:ident, $op:tt) =>
    {
        impl ops::$name for Vector4
        {
            fn $method(&mut self, rhs : Self)
            {
                self.x $op rhs.x;
                self.y $op rhs.y;
                self.z $op rhs.z;
                self.w $op rhs.w;
            }
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! vec4_f32_op
{
    ($name:ident, $method:ident, $op:tt) =>
    {
        impl ops::$name<f32> for Vector4
        {
            type Output = Vector4;

            fn $method(self, rhs : f32) -> Self::Output
            {
                Vector4::new4(self.x $op rhs, self.y $op rhs, self.z $op rhs, self.w $op rhs)
            }
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! vec4_f32_assign_op
{
    ($name:ident, $method:ident, $op:tt) =>
    {
        impl ops::$name<f32> for Vector4
        {
            fn $method(&mut self, rhs : f32)
            {
                self.x $op rhs;
                self.y $op rhs;
                self.z $op rhs;
                self.w $op rhs;
            }
        }
    }
}