use std::collections::HashMap;

fn main() {
    part1();
    part2();
}

fn part1() {
    let (scores, points) = scores();

    let input = include_str!("input.txt");
    let lines = input.split("\n");
    let mut my_points = 0;

    for line in lines {
        let vec = line.split(' ').collect::<Vec<&str>>();
        if let [his_move, my_move] = vec.as_slice() {
            let his_move_name = scores[his_move];
            let my_move_name = scores[my_move];

            my_points += points[my_move_name];

            if his_move_name == my_move_name {
                my_points += 3;
            } else if my_move_name == "rock" && his_move_name == "scissors" {
                my_points += 6;
            } else if my_move_name == "paper" && his_move_name == "rock" {
                my_points += 6;
            } else if my_move_name == "scissors" && his_move_name == "paper" {
                my_points += 6;
            }
        }
    }

    println!("Part1 (my total score) = {}", my_points);
}

fn part2() {
    let (scores, points) = scores();

    let input = include_str!("input.txt");
    let lines = input.split("\n");
    let mut my_points = 0;

    for line in lines {
        let vec = line.split(' ').collect::<Vec<&str>>();
        if let [his_move, my_move] = vec.as_slice() {
            let his_move_name = scores[his_move];
            
            // need to be equal
            if &"Y" == my_move {
                my_points += 3 + points[his_move_name];
            } else if my_move == &"X" { // i need to loose
                if his_move_name == "rock" {
                    my_points += points["scissors"];
                } else if his_move_name == "paper" {
                    my_points += points["rock"];
                }  else if his_move_name == "scissors" {
                    my_points += points["paper"];
                }
            } else if my_move == &"Z" { // i need to win
                my_points += 6;
                if his_move_name == "rock" {
                    my_points += points["paper"];
                } else if his_move_name == "paper" {
                    my_points += points["scissors"];
                }  else if his_move_name == "scissors" {
                    my_points += points["rock"];
                }
            }
        }
    }

    println!("Part2 (my total score) = {}", my_points);
}

fn scores() -> (HashMap<&'static str, &'static str>, HashMap<&'static str, i32>) {
    let mut scores = HashMap::new();
    let mut points = HashMap::new();

    points.insert("rock", 1);
    points.insert("paper", 2);
    points.insert("scissors", 3);

    scores.insert("A", "rock");
    scores.insert("B", "paper");
    scores.insert("C", "scissors");
    
    scores.insert("X", "rock");
    scores.insert("Y", "paper");
    scores.insert("Z", "scissors");

    return (scores, points)
}
