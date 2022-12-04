fn main() {
    part1();
    part2();
}

fn part1() {
    let input = include_str!("input.txt");

    let mut sum = 0;
    for line in input.split("\n") {
        let line_lenght = line.len();
        let first_compartment = &line[..line_lenght/2];
        let second_compartment = &line[line_lenght/2..];

        let first_compartment_items = first_compartment.split("").filter(|&x| !x.is_empty());
        let second_compartment_items = second_compartment.split("").filter(|&x| !x.is_empty());

        let mut shared_items = vec![];

        for item_first in first_compartment_items {
            for item_second in second_compartment_items.clone() {
                if item_first == item_second {
                    shared_items.push(item_first);
                }
            }
        }
        shared_items.sort_unstable();
        shared_items.dedup();

        for shared in shared_items {
            let chars: Vec<char> = shared.chars().collect();
            let ascii = chars[0] as u32;

            sum += if chars[0].is_lowercase() {
                ascii - 96
            } else {
                ascii - 38
            };
        }
    }
    println!("Part1 = {}", sum);
}

fn part2() {
    let input = include_str!("input.txt");

    let mut groups: Vec<Vec<Vec<&str>>> = Vec::new();
    let mut current_group: Vec<Vec<&str>> = Vec::new();
    let mut common_per_groups: Vec<Vec<&str>> = Vec::new();

    let mut idx = 0;
    for line in input.split("\n") {
        if idx == 3 {
            groups.push(current_group.clone());
            current_group.clear();
            idx = 0;
        }

        current_group.push(line.split("").filter(|&x| !x.is_empty()).collect());
        idx += 1;
    }


    let mut group_number = 0;
    let mut sum = 0;
    for group in groups {
        common_per_groups.push(Vec::new());

        for item1 in group[0].iter() {
            for item2 in group[1].iter() {
                if item1 == item2 {
                    if group[2].contains(item1) && !common_per_groups[group_number].contains(item1) {
                        let char = item1.chars().collect::<Vec<char>>()[0];
                        sum += if char.is_lowercase() {
                            char as u32 - 96
                        } else {
                            char as u32 - 38
                        };

                        common_per_groups[group_number].push(item1);
                    }
                }
            }

        }

        group_number += 1;
    }

    println!("Part2 = {}", sum);
}
