use crate::Coordinate;

// A trait for types that can be used as indices in a grid.
pub trait GridIndex {
    fn x(&self) -> isize;
    fn y(&self) -> isize;
}

#[macro_export]
macro_rules! implement_conversion {
    ( $t:ty ) => {
        impl GridIndex for ($t, $t) {
            fn x(&self) -> isize {
                self.0 as isize
            }

            fn y(&self) -> isize {
                self.1 as isize
            }
        }
        impl GridIndex for &($t, $t) {
            fn x(&self) -> isize {
                self.0 as isize
            }

            fn y(&self) -> isize {
                self.1 as isize
            }
        }
    };
}

implement_conversion!(isize);
implement_conversion!(usize);
implement_conversion!(i8);
implement_conversion!(u8);
implement_conversion!(i16);
implement_conversion!(u16);
implement_conversion!(i32);
implement_conversion!(u32);
implement_conversion!(i64);
implement_conversion!(u64);
implement_conversion!(i128);
implement_conversion!(u128);

impl GridIndex for Coordinate {
    fn x(&self) -> isize {
        self.0
    }

    fn y(&self) -> isize {
        self.1
    }
}
impl GridIndex for &Coordinate {
    fn x(&self) -> isize {
        self.0
    }

    fn y(&self) -> isize {
        self.1
    }
}
