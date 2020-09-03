pub trait Average<A = Self>: Sized {
    fn average<I: Iterator<Item = A>>(iter: I) -> Self;
}

macro_rules! average_for_types {
    ( $( $x:ident ),* ) => {$(
        impl Average for $x {
            fn average<I: Iterator<Item = Self>>(iter: I) -> Self {
                let sums = iter
                    .enumerate()
                    .fold((0, Self::default()), |acc, v| (acc.0 + 1, acc.1 + v.1));
                sums.1 / (sums.0 as $x)
            }
        }
        )*}
}

average_for_types!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, usize, f32, f64);
