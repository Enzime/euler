use std::fs;

fn print_grid(grid: &Vec<Vec<usize>>) {
    println!();

    for rows in grid.chunks(3) {
        for row in rows {
            for chunk in row.chunks(3) {
                for &cell in chunk {
                    if cell == 0 {
                        print!(" .");
                    } else {
                        print!(" {}", cell);
                    }
                }

                print!(" |");
            }

            println!();
        }

        println!("------------------------");
    }
}

fn compute_possibilities(grid: &Vec<Vec<usize>>, y: usize, x: usize) -> Vec<usize> {
    if grid[y][x] != 0 {
        return vec![];
    }

    let mut seen = [false; 10];

    // check column
    for z in 0..9 {
        if z != y {
            seen[grid[z][x]] = true;
        }
    }

    // check row
    for z in 0..9 {
        if z != x {
            seen[grid[y][z]] = true;
        }
    }

    // check box
    // use (y % 3, x % 3) to find the top left corner of the box and go through all of its cells,
    // maybe excluding the current col/row
    let y_start = y - (y % 3);
    let x_start = x - (x % 3);

    for i in y_start..y_start + 3 {
        for j in x_start..x_start + 3 {
            if i == y || j == x {
                continue;
            }

            seen[grid[i][j]] = true;
        }
    }

    (1..10).filter(|&n| !seen[n]).collect()
}

fn update_possibilities(possibilities: &mut Vec<Vec<Vec<usize>>>, y: usize, x: usize, value: usize) {
    possibilities[y][x].clear();

    for k in 0..9 {
        if let Some(pos) = possibilities[y][k].iter().position(|&n| n == value) {
            possibilities[y][k].swap_remove(pos);
        }

        if let Some(pos) = possibilities[k][x].iter().position(|&n| n == value) {
            possibilities[k][x].swap_remove(pos);
        }
    }

    let y_start = y - (y % 3);
    let x_start = x - (x % 3);

    for i in y_start..y_start + 3 {
        for j in x_start..x_start + 3 {
            if let Some(pos) = possibilities[i][j].iter().position(|&n| n == value) {
                possibilities[i][j].swap_remove(pos);
            }
        }
    }
}

fn is_valid_grid(grid: &Vec<Vec<usize>>) -> bool {
    let mut possibilities = vec![vec![vec![]; 9]; 9];

    for y in 0..9 {
        for x in 0..9 {
            possibilities[y][x] = compute_possibilities(&grid, y, x);

            // method 1. make sure every cell has a possibility
            if grid[y][x] == 0 && possibilities[y][x].len() == 0 {
                return false;
            }
        }
    }

    // method 2a. check every number is used or possible in every row
    for y in 0..9 {
        let mut possible = vec![false; 10];
        possible[0] = true;

        for x in 0..9 {
            if grid[y][x] != 0 {
                possible[grid[y][x]] = true;
                continue;
            }

            for &n in &possibilities[y][x] {
                possible[n] = true;
            }
        }

        if possible.iter().any(|n| !n) {
            return false;
        }
    }

    // method 2b. check every number is used or possible in every column
    for x in 0..9 {
        let mut possible = vec![false; 10];
        possible[0] = true;

        for y in 0..9 {
            if grid[y][x] != 0 {
                possible[grid[y][x]] = true;
                continue;
            }

            for &n in &possibilities[y][x] {
                possible[n] = true;
            }
        }

        if possible.iter().any(|n| !n) {
            return false;
        }
    }

    // method 2c. check every number is used or possible in every box
    for box_y in 0..3 {
        for box_x in 0..3 {
            let y_start = box_y * 3;
            let x_start = box_x * 3;

            let mut possible = vec![false; 10];
            possible[0] = true;

            for y in y_start..y_start + 3 {
                for x in x_start..x_start + 3 {
                    if grid[y][x] != 0 {
                        possible[grid[y][x]] = true;
                        continue;
                    }

                    for &n in &possibilities[y][x] {
                        possible[n] = true;
                    }
                }
            }

            if possible.iter().any(|n| !n) {
                return false;
            }
        }
    }

    return true;
}

fn solve_sudoku(grid: &mut Vec<Vec<usize>>, depth: usize) -> bool {
    if depth == 3 {
        return false;
    }

    let mut possibilities = vec![vec![vec![]; 9]; 9];

    for y in 0..9 {
        for x in 0..9 {
            possibilities[y][x] = compute_possibilities(&grid, y, x);
        }
    }

    let mut unsolved = grid.iter().map(|row| row.iter().filter(|&&c| c == 0).count()).sum::<usize>();

    'outer: while unsolved > 0 {
        if !is_valid_grid(grid) {
            return false;
        }

        let mut number_found = false;

        // method 1. check every square and see if any squares have only one possibility
        for y in 0..9 {
            for x in 0..9 {
                if possibilities[y][x].len() == 1 {
                    grid[y][x] = possibilities[y][x][0];
                    update_possibilities(&mut possibilities, y, x, grid[y][x]);
                    unsolved -= 1;
                    number_found = true;
                }
            }
        }

        // method 2a. check every row to see if any numbers have only one possible placement
        for y in 0..9 {
            let mut possible_squares = vec![vec![]; 10];

            for x in 0..9 {
                for &n in &possibilities[y][x] {
                    possible_squares[n].push((y, x));
                }
            }

            for n in 1..10 {
                if possible_squares[n].len() == 1 {
                    let (y, x) = possible_squares[n][0];
                    grid[y][x] = n;
                    update_possibilities(&mut possibilities, y, x, grid[y][x]);
                    unsolved -= 1;
                    number_found = true;
                }
            }
        }

        // method 2b. check every column to see if any numbers have only one possible placement
        for x in 0..9 {
            let mut possible_squares = vec![vec![]; 10];

            for y in 0..9 {
                for &n in &possibilities[y][x] {
                    possible_squares[n].push((y, x));
                }
            }

            for n in 1..10 {
                if possible_squares[n].len() == 1 {
                    let (y, x) = possible_squares[n][0];
                    grid[y][x] = n;
                    update_possibilities(&mut possibilities, y, x, grid[y][x]);
                    unsolved -= 1;
                    number_found = true;
                }
            }
        }

        // method 2c. check every box to see if any numbers have only one possible placement
        for box_y in 0..3 {
            for box_x in 0..3 {
                let y_start = box_y * 3;
                let x_start = box_x * 3;

                let mut possible_squares = vec![vec![]; 10];

                for y in y_start..y_start + 3 {
                    for x in x_start..x_start + 3 {
                        for &n in &possibilities[y][x] {
                            possible_squares[n].push((y, x));
                        }
                    }
                }

                for n in 1..10 {
                    if possible_squares[n].len() == 1 {
                        let (y, x) = possible_squares[n][0];
                        grid[y][x] = n;
                        update_possibilities(&mut possibilities, y, x, grid[y][x]);
                        unsolved -= 1;
                        number_found = true;
                    }
                }
            }
        }

        if number_found {
            continue 'outer;
        }

        // method 3. use backtracking to explore a few layers down
        for y in 0..9 {
            for x in 0..9 {
                for &n in &possibilities[y][x] {
                    let old_grid = grid.clone();

                    grid[y][x] = n;

                    if solve_sudoku(grid, depth + 1) {
                        return true;
                    }

                    *grid = old_grid;
                }
            }
        }

        return false;
    }

    return true;
}

fn main() {
    const FILENAME: &'static str = "hardest.txt";

    let grids = fs::read_to_string(FILENAME).unwrap()
                                            .lines()
                                            .collect::<Vec<_>>()
                                            .chunks(10)
                                            .map(|chunk| chunk[1..].to_vec())
                                            .map(|chunk|
                                                 chunk.into_iter()
                                                      .map(|line| line.chars()
                                                                      .map(|c| c.to_digit(10)
                                                                                .unwrap() as usize)
                                                                      .collect::<Vec<_>>())
                                                      .collect::<Vec<_>>())
                                            .collect::<Vec<_>>();

    let mut answer = 0;

    for (i, mut grid) in grids.into_iter().enumerate() {
        if solve_sudoku(&mut grid, 0) {
            answer += grid[0][0] * 100 + grid[0][1] * 10 + grid[0][2];
        } else {
            println!("#{}: unsolved", i);
            print_grid(&grid);
        }
    }

    println!("{}", answer);
}
