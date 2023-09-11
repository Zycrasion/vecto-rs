use vecto_rs::positional::Vector;

/// https://codinglab.huostravelblog.com/programming/random-number-generator/index.php
const NUMBERS : [f32; 64] = [
    54016.58056,36046.64701,68083.0081,17439.24685,12264.85691,42537.58248,76953.14525,59586.2037,29924.15091,92414.90439,29059.03177,62025.16413,52614.26174,73541.01481,43053.78006,39076.54495,49374.16549,35116.56838,90786.21963,33077.89504,77280.09441,30752.26747,61263.77304,76107.71859,51612.04636,80862.64984,55707.07469,48121.44394,40553.93507,42293.4667,22002.26181,56399.38,14846.59404,62639.59556,57793.33259,15516.86282,7491.73331,66188.78305,54051.61207,74522.07959,55934.24509,46848.47728,32107.77094,47758.57574,81618.47894,86969.2267,36139.12122,58274.63041,45252.54524,92133.54932,42866.45601,42448.39306,45671.50405,58680.32647,50984.72314,56367.04513,6642.74933,55230.39017,17995.65512,31987.95614,90946.83957,39923.86888,6097.30273,37608.00246
];

macro_rules! random {
    () => {
        {
            let ptr = Box::into_raw(Box::new(123)) as usize;
            let ptr = ptr % 64;

            NUMBERS[ptr]
        }
    };
}

macro_rules! vec3_test {
    ($name:ident, $op:tt) => {
        #[test]
        fn $name()
        {
            vec3_test!($op);
        }

    };
    ($op:tt) => {
        for _ in 0..100
        {
            let a = random!();
            let b = random!();
            let c = random!();
            let a2 = random!();
            let b2 = random!();
            let c2 = random!();

            let mut vec1 = Vector::new3(a, b, c);
            vec1 = vec1 $op Vector::new3(a2, b2, c2);

            assert_eq!(
                vec1,
                Vector::new3(a $op a2, b $op b2, c $op c2)
            )
        }
    }
}

vec3_test!(addition_test_vec3, +);
vec3_test!(subtraction_test_vec3, -);
vec3_test!(multiplication_test_vec3, *);
vec3_test!(division_test_vec3, /);