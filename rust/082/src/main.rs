use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs;

fn main() {
    const INPUT_FILE: &str = "matrix.txt";

    let matrix = fs::read_to_string(INPUT_FILE).unwrap()
                                               .trim()
                                               .split("\n")
                                               .map(|line| line.split(",")
                                                               .map(|cell| cell.parse::<usize>().unwrap())
                                                               .collect::<Vec<_>>())
                                               .collect::<Vec<_>>();

    let rows = matrix.len();
    let columns = matrix[0].len();

    let vertex_id = |row: usize, column: usize| row * columns + column + 1;
    let matrix_coords = |vertex: usize| ((vertex - 1) / columns, (vertex - 1) % columns);

    // directed graph
    let mut edges_of = vec![vec![]; rows * columns + 2];

    for row in 0..rows {
        // create a start and end vertex connecting to first and last columns respectively
        edges_of[0].push(vertex_id(row, 0));
        edges_of[vertex_id(row, columns - 1)].push(rows * columns + 1);

        for column in 0..columns {
            let v = vertex_id(row, column);

            if row > 0 {
                edges_of[v].push(vertex_id(row - 1, column));
            }

            if row < rows - 1 {
                edges_of[v].push(vertex_id(row + 1, column));
            }

            if column < columns - 1 {
                edges_of[v].push(vertex_id(row, column + 1));
            }
        }
    }

    let mut distance_of = vec![std::usize::MAX; rows * columns + 2];
    let mut queue = BinaryHeap::new();

    distance_of[0] = 0;
    queue.push(Reverse((0, 0)));

    let mut is_finalized = vec![false; rows * columns + 2];

    while let Some(Reverse((distance, v))) = queue.pop() {
        is_finalized[v] = true;

        for &u in &edges_of[v] {
            if is_finalized[u] {
                continue;
            }

            let mut new_distance = distance;

            if u < rows * columns + 1 {
                let (i, j) = matrix_coords(u);
                new_distance += matrix[i][j];
            }

            if new_distance < distance_of[u] {
                queue.push(Reverse((new_distance, u)));
                distance_of[u] = new_distance;
            }
        }
    }

    println!("{}", distance_of[rows * columns + 1]);
}
