use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter)]
enum Direction {
    Down,
    Right
}

fn main() {
    let mut grid = [[0u128; 21]; 21];

    grid[grid.len() - 1][grid[0].len() - 1] = 1;

    for y in (0..grid.len()).rev() {
        for x in (0..grid[0].len()).rev() {
            if grid[y][x] != 0 {
                continue;
            }

            let mut paths = 0u128;

            for direction in Direction::iter() {
                let (mut dy, mut dx) = (0, 0);

                match direction {
                    Direction::Down => {
                        dy = 1;
                    },
                    Direction::Right => {
                        dx = 1;
                    }
                }

                let (new_y, new_x) = (y + dy, x + dx);

                if (0..grid.len()).contains(&new_y) && (0..grid[0].len()).contains(&new_x) {
                    // println!("{} {}", paths, grid[new_y as usize][new_x as usize]);
                    paths += grid[new_y as usize][new_x as usize];
                }
            }

            grid[y][x] = paths;
        }
    }

    println!("{}", grid[0][0]);
}
