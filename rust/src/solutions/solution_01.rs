fn part_1(input: &str) -> i32{
    let mut floor: i32 = 0;
    for i in input.chars() {
        if i == '(' {
            floor += 1;
        }
        else {
            floor += -1;
        }
    }
    return floor;
}

fn part_2(input: &str) -> i32{
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
            return count;
        }
    }

    return 0;
}

pub fn solve(input: String) -> (i32, i32) {
    return (part_1(input.as_ref()), part_2(input.as_ref()));
}