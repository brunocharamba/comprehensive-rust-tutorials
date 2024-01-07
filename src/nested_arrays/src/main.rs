fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut transposed: [[i32; 3]; 3] = [[0; 3]; 3];

    for i in 0..=2 {
        for j in 0..=2 {
            transposed[i][j] = matrix[j][i];
        }
    }

    transposed
}

#[test]
fn transpose_works() {
    let matrix = [
        [101, 102, 103],
        [201, 202, 203],
        [301, 302, 303],
    ];

    let transposed_matrix = transpose(matrix);

    assert_eq!(transposed_matrix, [
        [101, 201, 301],
        [102, 202, 302],
        [103, 203, 303],
    ])
}


fn main() {
    //Hard-code both functions to operate on 3 × 3 matrices.
    let matrix = [
        [101, 102, 103],
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix: {:#?}", matrix);
    let transposed = transpose(matrix);
    println!("transposed: {:#?}", transposed);
}
