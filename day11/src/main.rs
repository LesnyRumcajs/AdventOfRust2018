const WIDTH: usize = 300;
const HEIGHT: usize = 300;
const GRID_SN: i32 = 9005;//9005;

struct Candidate {
    x: usize,
    y: usize,
    size: usize,
    power: i32
}

fn main() {
    let mut grid = [[0; HEIGHT]; WIDTH];

    for x in 1..WIDTH + 1 {
        for y in 1..HEIGHT + 1 {
            grid[x - 1][y - 1] = calculate_power_level(x, y);
        }
    }

    let mut candidate = Candidate {x: 0, y:0 , size: 0, power: 0};

    let mut areas = [[0; HEIGHT]; WIDTH];
    for size in 1..WIDTH {
        process_grid(&mut grid, &mut areas, size);

        update_candidate(&mut candidate, &mut areas, size)
    }

    println!("max_power: {} [{}:{}:{}]", candidate.power, candidate.x, candidate.y, candidate.size);
}

fn process_grid(grid: &mut [[i32; 300]; 300], areas: &mut [[i32; 300]; 300], size: usize) {
    for x in 1..WIDTH + 1 {
        for y in 1..HEIGHT + 1 {
            if x > WIDTH - size + 1 || y > HEIGHT - size + 1 {
                areas[x - 1][y - 1] = 0;
            } else {
                let mut total_power = 0;
                for xx in x..x + size {
                    for yy in y..y + size {
                        total_power += grid[yy - 1][xx - 1];
                    }
                }
                areas[x - 1][y - 1] = total_power;
            }
        }
    }
}

fn update_candidate(candidate: &mut Candidate, areas: &mut [[i32; 300]; 300], size: usize) -> () {
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            if areas[y][x] > candidate.power {
                candidate.power = areas[y][x];
                candidate.x = x + 1;
                candidate.y = y + 1;
                candidate.size = size;
            }
        }
    }
}

fn calculate_power_level(x: usize, y: usize) -> i32 {
    let rack_id = x as i32 + 10;

    let mut power_level = rack_id * y as i32;

    power_level += GRID_SN;

    power_level *= rack_id;

    power_level = power_level % 1000 / 100;

    power_level -= 5;

    power_level
}
