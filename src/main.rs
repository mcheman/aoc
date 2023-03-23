use std::fs;

fn one(first_half: bool) {
    let file = fs::read_to_string("inputs/1").expect("Can't find file");

    let mut elves = vec![];

    for s in file.split("\n\n") {
        let mut total = 0;
        for calories in s.split("\n") {
            total += match calories.parse::<i32>() {
                Ok(num) => num,
                Err(_) => 0
            };
        }
        elves.push(total);
    }

    elves.sort();

    if first_half {
        println!("{}", elves.get(elves.len() - 1).expect("elves is empty"));
    } else {
        let total = elves.get(elves.len() - 1).expect("elves is empty") +
                        elves.get(elves.len() - 2).expect("elves is empty") +
                        elves.get(elves.len() - 3).expect("elves is empty");
        println!("{}", total);
    }

}

fn two(first_half: bool) {
    let file = fs::read_to_string("inputs/2").expect("Can't find file");

    let mut score = 0;
    for s in file.split("\n") {
        let mut round = vec![];
        for a in s.split(" ") {
            round.push(a);
        }
        let you = round.pop().expect("you");
        let them = round.pop().expect("them");r



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

fn main() {
    //one(false);
    two(false);
}
