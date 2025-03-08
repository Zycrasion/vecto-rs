macro_rules! import {
    ($($module_name:tt )+) => {
        $(pub mod $module_name;)+

        pub mod prelude 
        {
            $(pub use crate::$module_name::*;)+
        }
    };
}

import!(types vector angle);