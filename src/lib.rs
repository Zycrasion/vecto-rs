#[macro_use]
pub(self) mod private_macros;

macro_rules! import {
    ($($module_name:tt )+) => {
        $(pub mod $module_name;)+

        pub mod prelude 
        {
            $(pub use crate::$module_name::*;)+
        }
    };
}

import!(bounding_box quadtree triangles line vec radians types);