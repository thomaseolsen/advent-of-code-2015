fn get_dimensions(present: &str) -> [i32; 3] {
    let mut present_dimensions: [i32; 3] = [0; 3];
    let mut i: usize = 0;
    for dimension in present.split("x") {
        present_dimensions[i] = dimension.parse::<i32>().unwrap();
        i += 1;
    }
    return present_dimensions;
}

fn get_areas(present_dimensions: [i32; 3]) -> [i32; 3] {
    let mut present_areas: [i32; 3] = [0; 3];
    present_areas[0] = present_dimensions[0] * present_dimensions[1];
    present_areas[1] = present_dimensions[1] * present_dimensions[2];
    present_areas[2] = present_dimensions[0] * present_dimensions[2];
    return present_areas;
}

fn get_perimeters(present_dimensions: [i32; 3]) -> [i32; 3] {
    let mut present_perimeters: [i32; 3] = [0; 3];
    present_perimeters[0] = (2 * present_dimensions[0]) + (2 * present_dimensions[1]);
    present_perimeters[1] = (2 * present_dimensions[1]) + (2 * present_dimensions[2]);
    present_perimeters[2] = (2 * present_dimensions[0]) + (2 * present_dimensions[2]);
    return present_perimeters;
}

fn get_volume(present_dimensions: [i32; 3]) -> i32 {
    return present_dimensions[0] * present_dimensions[1] * present_dimensions[2];
}

fn part_1(lines: &Vec<&str>) -> i32 {
    let mut total_needed: i32 = 0;
    for present in lines {
        if !present.is_empty() {
            let present_dimensions: [i32; 3] = get_dimensions(present);
            let present_areas: [i32; 3] = get_areas(present_dimensions);
            total_needed += (2 * present_areas[0])
                + (2 * present_areas[1])
                + (2 * present_areas[2])
                + present_areas.iter().min().unwrap();
        }
    }
    return total_needed;
}

fn part_2(lines: &Vec<&str>) -> i32 {
    let mut total_needed: i32 = 0;
    for present in lines {
        if !present.is_empty() {
            let present_dimensions: [i32; 3] = get_dimensions(present);
            let present_perimeters: [i32; 3] = get_perimeters(present_dimensions);
            total_needed += present_perimeters.iter().min().unwrap()
                + get_volume(present_dimensions);
        }
    }
    return total_needed;
}

pub fn solve(input: Vec<&str>) -> (i32, i32) {
    return (part_1(input.as_ref()), part_2(input.as_ref()));
}
