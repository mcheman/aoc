use std::fs;
use rayon::prelude::*;


pub fn one() -> Vec<i32> {
    let file = fs::read_to_string("2022/input/1").expect("Can't find file");

    let mut elves = vec![];

    elves = file.split("\n\n").map(|s| {
        s.lines()
            .map(|calories| calories.parse::<i32>())
            .filter_map(Result::ok)
            .sum::<i32>()
    }).collect();

    elves.sort_by(|a, b| b.cmp(a));

    elves
}

fn two(first_half: bool) {
    let file = fs::read_to_string("2022/input/2").expect("Can't find file");

    let mut score = 0;
    for s in file.split("\n") {
        let mut round = vec![];
        for a in s.split(" ") {
            round.push(a);
        }
        let you = round.pop().expect("you");
        let them = round.pop().expect("them");



        if first_half {
            score += if you == "X" {1} else if you == "Y" {2} else {3};
            // tie
            if (you == "X" && them == "A") || (you == "Y" && them == "B") || (you == "Z" && them == "C") {
                score += 3;
            }
            else if (you == "X" && them == "B") || (you == "Y" && them == "C") || (you == "Z" && them == "A") {
                score += 0
            } else {
                score += 6;
            }
        } else {
            if you == "X" {
                score += 0;
                if them == "A" {
                    score += 3;
                } else if them == "B" {
                    score += 1;
                } else {
                    score += 2;
                }
            } else if you == "Y" {
                score += 3;
                if them == "A" {
                    score += 1;
                } else if them == "B" {
                    score += 2;
                } else {
                    score += 3;
                }
            } else {
                score += 6;
                if them == "A" {
                    score += 2;
                } else if them == "B" {
                    score += 3;
                } else {
                    score += 1;
                }

            }


        }

    }
    println!("{}", score);
}
