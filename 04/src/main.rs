fn main() {
    part1();
    part2();
}

fn part1() {
    let input = include_str!("input.txt");
    let lines = input.split("\n").filter(|&l| !l.is_empty());

    let mut result = 0;
    for line in lines {
        let assignments = line.split(",").collect::<Vec<&str>>();
        let first_assignment = assignments[0];
        let second_assignment = assignments[1];

        let first_ranges = first_assignment.split("-").collect::<Vec<&str>>();
        let first_start = first_ranges[0].parse::<i32>().unwrap();
        let first_end = first_ranges[1].parse::<i32>().unwrap();

        let second_ranges = second_assignment.split("-").collect::<Vec<&str>>();
        let second_start = second_ranges[0].parse::<i32>().unwrap();
        let second_end = second_ranges[1].parse::<i32>().unwrap();

        // second should be inside first
        if first_end - first_start > second_end - second_start {
            if second_start >= first_start && second_end <= first_end{
                result += 1;
            }
        } else if second_end - second_start > first_end - first_start { // first should be inside second
            if first_start >= second_start &&first_end <= second_end{
                result += 1;
            }
        } else { // same size
            if first_assignment == second_assignment {
                result += 1;
            }
        }
    }

    println!("Part1 = {}", result);
}

fn part2() {
    let input = include_str!("input.txt");
    let lines = input.split("\n").filter(|&l| !l.is_empty());

    let mut result = 0;
    for line in lines {
        let assignments = line.split(",").collect::<Vec<&str>>();
        let first_assignment = assignments[0];
        let second_assignment = assignments[1];

        let first_ranges = first_assignment.split("-").collect::<Vec<&str>>();
        let first_start = first_ranges[0].parse::<i32>().unwrap();
        let first_end = first_ranges[1].parse::<i32>().unwrap();

        let second_ranges = second_assignment.split("-").collect::<Vec<&str>>();
        let second_start = second_ranges[0].parse::<i32>().unwrap();
        let second_end = second_ranges[1].parse::<i32>().unwrap();

        if first_start >= second_start && first_start <= second_end{ 
            result += 1;
        } else if second_start >= first_start && second_start <= first_end{ 
            result += 1;
        } else if first_end >= second_start && first_end <= second_end { 
            result += 1;
        } else if second_end >= first_start && second_end <= first_end { 
            result += 1;
        }
    }

    println!("Part2 = {}", result);
}
