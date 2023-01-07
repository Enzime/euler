fn main() {
    for grid_width in 3.. {
        for grid_height in 1..=grid_width {
            let mut rectangles = 0;

            for rect_width in 1..=grid_width {
                for rect_height in 1..=grid_height {
                    rectangles += (grid_width - rect_width + 1) * (grid_height - rect_height + 1);
                }
            }

            let difference = rectangles as i32 - 2_000_000;

            if difference.abs() < 1_000 {
                println!("{}", grid_width * grid_height);
                return;
            }
        }
    }
}
