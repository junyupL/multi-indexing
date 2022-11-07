indexing_macro::file!{

use array_macro::*;

struct Matrix<T> {
    data: Vec<Vec<T>>,
}

impl<T> Matrix<T> {
    fn index2(&mut self, a: usize, b: usize) -> &mut T {
        &mut self.data[a][b]
    }
}

fn identity_matrix<const N: usize>() -> Matrix<i32> {
    let mut matrix = Matrix{data: v!(N, N, 0)};
    for i in 0..N {
        matrix[i, i] = 1;
    }
    matrix
}

fn main() {
    let array = arr!(1, 3, 6);
    let mut array2 = a!(3, 0);
    let reference = &mut array2;
    reference[][0] = 1;
    reference[][1] = 3;
    reference[][2] = 6;
    if array == array2 {
        //doesn't work yet
        //println!("{}", identity_matrix::<5>()[2, 3]);

        let x = identity_matrix::<5>()[2, 3];
        println!("{}", x);
    }
}

}