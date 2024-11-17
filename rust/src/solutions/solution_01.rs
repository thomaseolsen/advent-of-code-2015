fn part_1(input: &str) {
    let mut floor: i32 = 0;
    for i in input.chars() {
        if i == '(' {
            floor += 1;
        }
        else {
            floor += -1;
        }
    }
    println!("Floor: {:?}", floor);
}

fn part_2(input: &str) {
    let mut floor: i32 = 0;
    let mut count: i32 = 0;
    for i in input.chars() {
        count += 1;
        if i == '(' {
            floor += 1;
        }
        else {
            floor += -1;
        }

        if floor < 0 {
            println!("Basement: {:?}", count);
            break;
        }
    }
}

pub fn solve(input: String) {
    part_1(input.as_ref());
    part_2(input.as_ref());
}