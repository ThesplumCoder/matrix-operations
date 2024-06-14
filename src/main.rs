fn main() {
    let matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];

    matrix_printer(&matrix);
}

fn matrix_printer(matrix: &[[i8; 3]; 3]) {
    println!("[");
    for row in matrix {
        println!("[");
        for column in row {
            println!("{column}");
        }
        println!("]");
    }
    println!("]");
}
