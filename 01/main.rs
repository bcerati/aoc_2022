fn main() {
    part1();
    part2();
}

fn part1() {
    let input = include_str!("input.txt");
    let split = input.split("\n");
    let mut max_elve = 0;

    let mut current_elve_sum_cal = 0;
    for item in split {
        if item.len() == 0 {
            if current_elve_sum_cal > max_elve {
                max_elve = current_elve_sum_cal;
            }
            current_elve_sum_cal = 0;
        } else {
            current_elve_sum_cal += item.parse::<i32>().unwrap();
        }
    }

    println!("Part1 = {}", max_elve);
}

fn part2() {
    let input = include_str!("input.txt");
    let split = input.split("\n");
    let mut most_cl = Vec::new();

    let mut current_elve_sum_cal = 0;
    for item in split {
        if item.len() == 0 {
            if most_cl.len() < 3 {
                most_cl.push(current_elve_sum_cal);
            } else {
                let min_vec = most_cl.iter().min();
                match min_vec {
                    Some(min) => {
                        if min < &current_elve_sum_cal {
                            let idx = most_cl.iter().position(|x| *x == *min).unwrap();
                            most_cl.remove(idx);
                            most_cl.push(current_elve_sum_cal);
                        }
                    },
                    None => ()
                }

            }

            current_elve_sum_cal = 0;
        } else {
            current_elve_sum_cal += item.parse::<i32>().unwrap();
        }
    }

    println!("Part2 = {}", most_cl.iter().sum::<i32>());
}
