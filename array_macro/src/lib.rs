
//array type and array literal syntax:
//a!(size1, ..., sizeN-1, sizeN, expr) = [...[[expr; sizeN]; sizeN-1];...;size1]
//[...] expr will not work with multi indexing macro, so use a! instead.
#[macro_export]
macro_rules! a {
    ($n:expr, $x:expr $(,)?) => { [$x; $n] };
    ($n:expr, $($x:expr),+ $(,)?) => {
        [
            
        a!($(
            $x,
        )*)
            
            ; $n]
    };
}

//same as a! but for vec!
#[macro_export]
macro_rules! v {
    ($n:expr, $x:expr $(,)?) => { vec!($x; $n) };
    ($n:expr, $($x:expr),+ $(,)?) => {
        vec!(
            
        v!($(
            $x,
        )*)
            
            ; $n)
    };
}

//same as [expr, expr2, expr3, ...], but
//[...] expr will not work with multi indexing macro, so use arr! instead.
#[macro_export]
macro_rules! arr {
    () => { [] };
    ($($x:expr),+ $(,)?) => {
        [    
        $(
            $x,
        )*]
    };
}


pub type A<const N: usize, T> = [T; N];

pub fn a<const N: usize, T: Copy>(expr: T) -> A<N, T> {
    [expr; N]
}