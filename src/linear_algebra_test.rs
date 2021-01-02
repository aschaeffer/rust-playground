use ndarray::{arr1, arr2, Array1};

fn add_matrices() {
    let a = arr2(&[[1, 2, 3],
        [4, 5, 6]]);

    let b = arr2(&[[6, 5, 4],
        [3, 2, 1]]);

    let sum = &a + &b;

    println!("{}", a);
    println!("+");
    println!("{}", b);
    println!("=");
    println!("{}", sum);
}

fn multiply_matrices() {
    let a = arr2(&[[1, 2, 3],
        [4, 5, 6]]);

    let b = arr2(&[[6, 3],
        [5, 2],
        [4, 1]]);

    println!("{}", a);
    println!("*");
    println!("{}", b);
    println!("=");
    println!("{}", a.dot(&b));
}


fn multiply_scalar_with_vector_with_matrix() {
    let scalar = 4;

    let vector = arr1(&[1, 2, 3]);

    let matrix = arr2(&[[4, 5, 6],
        [7, 8, 9]]);

    println!("{}", scalar);
    println!("*");
    println!("{}", vector);
    println!("=");

    let new_vector: Array1<_> = scalar * vector;
    println!("{}", new_vector);

    println!("{}", matrix);
    println!("*");
    println!("{}", new_vector);
    println!("=");

    let new_matrix = matrix.dot(&new_vector);
    println!("{}", new_matrix);
}

pub fn linear_algebra_tests() {
    add_matrices();
    multiply_matrices();
    multiply_scalar_with_vector_with_matrix();
}
