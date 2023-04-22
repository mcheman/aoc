use std::collections::{BTreeSet, HashMap, HashSet};
use std::fs;
use std::num::Wrapping;
use rayon::prelude::*;


//24!
// can simulate step by step, check current positions and yield set of positions that can be moved into for next time step. if those positions contain the end state, we're done



#[derive(Copy, Clone, Eq, Hash, PartialEq, Debug)]
struct Pos {
    x: u8,
    y: u8,
}

// use these for example input
// const INPUT_FILE: &str = "2022/input/24.example";
// const WIDTH: u8 = 8;
// const HEIGHT: u8 = 6;

// use these for regular input
const INPUT_FILE: &str = "2022/input/24";
const WIDTH: u8 = 122;
const HEIGHT: u8 = 27;



// contains bitfield of tiles
type Map = [[u8; HEIGHT as usize]; WIDTH as usize];

fn load_map(filename: &str) -> Map {
    let file = fs::read_to_string(filename).expect("Can't find file");

    let mut map: Map = [[0; HEIGHT as usize]; WIDTH as usize];

    let mut x: u8 = 0;
    let mut y: u8 = 0;
    for char in file.chars() {
        let tile = match char {
            '.' => 0,
            '#' => 1,
            '^' => 2,
            'v' => 4,
            '<' => 8,
            '>' => 16,
            '\n' => {
                x = 0;
                y += 1;
                continue;
            }
            c => {
                panic!("Found unknown character '{c}'");
            }
        };

        map[x as usize][y as usize] = tile;
        x += 1;
    }

    map
}

fn print(map: &Map) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            print!("{}", match map[x as usize][y as usize] {
                0 => '.',
                1 => '#',
                2 => '^',
                4 => 'v',
                8 => '<',
                16 => '>',
                _ => '*' // contains more than one tile
            });
        }
        println!();
    }
}

fn print_nodes(nodes: &HashSet<Pos>) {
    println!("{:?}", nodes);
}

fn next_map(map: &Map) -> Map {
    let mut new_map: Map = [[0; HEIGHT as usize]; WIDTH as usize];

    for y in 0..HEIGHT as usize {
        // TODO assuming that the start and end spaces cannot have blizzards in them (inputs do not exhibit this)
        // let space_before_left_wall = if map[0][y] == 0 { 0 } else { 1 };
        // let space_before_right_wall = (if map[(WIDTH - 1) as usize][y] == 0 { WIDTH - 1 } else { WIDTH - 2 }) as usize;
        let space_before_left_wall = 1;
        let space_before_right_wall = (WIDTH - 2) as usize;

        for x in 0..WIDTH as usize {
            if map[x][y] == 0 { // skip over empty spaces, nothing to update
                continue;
            }

            if map[x][y] == 1 { // copy over walls directly
                new_map[x][y] = 1;
                continue;
            }


            let space_before_top_wall = 1;
            let space_before_bottom_wall = (HEIGHT - 2) as usize;


            // there now must be one or more blizzards

            // moving up
            if map[x][y] & 2 != 0 {
                if y == space_before_top_wall {
                    new_map[x][space_before_bottom_wall] += 2; // wrap
                } else {
                    new_map[x][y - 1] += 2;
                }
            }

            // moving down
            if map[x][y] & 4 != 0 {
                if y == space_before_bottom_wall {
                    new_map[x][space_before_top_wall] += 4; // wrap
                } else {
                    new_map[x][y + 1] += 4;
                }
            }

            // moving left
            if map[x][y] & 8 != 0 {
                if x == space_before_left_wall {
                    new_map[space_before_right_wall][y] += 8; // wrap
                } else {
                    new_map[x - 1][y] += 8;
                }
            }

            // moving right
            if map[x][y] & 16 != 0 {
                if x == space_before_right_wall {
                    new_map[space_before_left_wall][y] += 16; // wrap
                } else {
                    new_map[x + 1][y] += 16;
                }
            }
        }
    }

    new_map
}

fn neighbors(map: &Map, nodes: &HashSet<Pos>) -> HashSet<Pos>{

    let mut new_nodes = HashSet::new();

    for node in nodes {
        let neighbors = [
            *node, // self
            Pos{y: node.y - 1, ..*node }, // up
            Pos{y: node.y + 1, ..*node }, // down
            Pos{x: node.x - 1, ..*node }, // left
            Pos{x: node.x + 1, ..*node }, // right
        ];

        for n in neighbors {
            if let Some(col) = map.get(n.x as usize) {
                if let Some(tile) = col.get(n.y as usize) {
                    if *tile == 0 {
                        new_nodes.insert(n);
                    }
                }
            }
        }
    }

    // Remove starting place since it always generates itself as a neighbor if it was a node
    // new_nodes.remove(&Pos { x: 1, y: 0 });

    new_nodes
}

pub fn twenty_four() {
    let mut map = load_map(INPUT_FILE);
    // let initial_map = map.clone();
    print(&map);

    let mut have_snacks = false;

    let starting_node = Pos { x: 1, y: 0 };
    let goal_node = Pos { x: WIDTH - 2, y: HEIGHT - 1 };

    let mut nodes = HashSet::new();
    nodes.insert(starting_node);

    for i in 1..=6000 {
        println!("\nMinute {i}!");
        map = next_map(&map);
        // print(&map);

        nodes = neighbors(&map, &nodes);
        // print_nodes(&nodes);

        if nodes.iter().filter(|&n| *n == goal_node).count() > 0 {
            println!("FOUND PATH BABY! First Run");

            // go back to start
            nodes.clear();
            nodes.insert(goal_node);

            for ii in i+1..=6000 {
                println!("\nMinute {ii}!");
                map = next_map(&map);
                nodes = neighbors(&map, &nodes);

                if nodes.iter().filter(|&n| *n == starting_node).count() > 0 {
                    println!("FOUND PATH BABY! Back at start");

                    // go back to end
                    nodes.clear();
                    nodes.insert(starting_node);

                    for iii in ii+1..=6000 {
                        println!("\nMinute {iii}!");
                        map = next_map(&map);
                        nodes = neighbors(&map, &nodes);

                        if nodes.iter().filter(|&n| *n == goal_node).count() > 0 {
                            println!("FOUND PATH BABY! DONE!");
                            return;
                        }
                    }

                }
            }
        }

        // let mut found_match = true;

        // check if matches initial state to find out what period is
        // period is least common multiple of dimensions inside walls!! 600 for regular input, 12 for example
        // for y in 0..HEIGHT as usize {
        //     for x in 0..WIDTH as usize {
        //         if map[x][y] != initial_map[x][y] {
        //             found_match = false;
        //             break;
        //         }
        //     }
        //
        //     if !found_match {
        //         break;
        //     }
        // }
        //
        // if found_match {
        //     println!("Found Match!!!");
        //     break;
        // }

    }
}


// type Map = HashMap<Pos, Elf>;
//
// #[derive(Copy, Clone, Eq, PartialEq)]
// enum Elf {
//     MoveNorth,
//     MoveSouth,
//     MoveWest,
//     MoveEast,
//     MoveNone,
// }
//
// #[derive(Copy, Clone, Eq, Hash, PartialEq)]
// struct Pos(i16, i16);
//
// trait Solve {
//     fn new() -> Elf;
//     fn next_direction(&self) -> Elf;
//     fn positions_in_direction(&self, _: &Pos) -> (Pos, Pos, Pos);
// }
//
//
//
// // top left is 0,0
// impl Solve for Elf {
//     fn new() -> Elf {
//         Elf::MoveNone
//     }
//
//     fn next_direction(&self) -> Elf {
//         match self {
//             Elf::MoveNorth => Elf::MoveSouth,
//             Elf::MoveSouth => Elf::MoveWest,
//             Elf::MoveWest => Elf::MoveEast,
//             Elf::MoveEast => Elf::MoveNorth,
//             Elf::MoveNone => Elf::MoveNone
//         }
//     }
//
//     fn positions_in_direction(&self, &Pos(x, y): &Pos) -> (Pos, Pos, Pos) {
//         match self {
//             Elf::MoveNorth => (
//                 Pos(x, y - 1),
//                 Pos(x - 1, y - 1),
//                 Pos(x + 1, y - 1)
//             ),
//             Elf::MoveSouth => (
//                 Pos(x, y + 1),
//                 Pos(x - 1, y + 1),
//                 Pos(x + 1, y + 1)
//             ),
//             Elf::MoveWest => (
//                 Pos(x - 1, y),
//                 Pos(x - 1, y - 1),
//                 Pos(x - 1, y + 1)
//             ),
//             Elf::MoveEast => (
//                 Pos(x + 1, y),
//                 Pos(x + 1, y - 1),
//                 Pos(x + 1, y + 1)
//             ),
//             Elf::MoveNone => (
//                 Pos(x, y),
//                 Pos(x, y),
//                 Pos(x, y)
//             )
//         }
//     }
// }
//
// fn load_map(filename: &str) -> Map {
//     let file = fs::read_to_string(filename).expect("Can't find file");
//
//     let mut elves = Map::with_capacity(1500);
//
//     let mut x: i16 = 0;
//     let mut y: i16 = 0;
//     for char in file.chars() {
//         let elf = match char {
//             '#' => {
//                 x += 1;
//                 Elf::new()
//             }
//             '.' => {
//                 x += 1;
//                 continue;
//             }
//             '\n' => {
//                 x = 0;
//                 y += 1;
//                 continue;
//             }
//             c => {
//                 panic!("Found unknown character '{c}'");
//             }
//         };
//
//         elves.insert(Pos(x, y), elf);
//     }
//
//     elves
// }
//
// fn bounding_rect(map: &Map) -> (Pos, Pos) {
//     let mut min_x= i16::MAX;
//     let mut max_x= i16::MIN;
//     let mut min_y= i16::MAX;
//     let mut max_y= i16::MIN;
//
//     map.iter().for_each(|(p, _)| {
//         if p.0 < min_x {
//             min_x = p.0;
//         }
//
//         if p.0 > max_x {
//             max_x = p.0;
//         }
//
//         if p.1 < min_y {
//             min_y = p.1;
//         }
//
//         if p.1 > max_y {
//             max_y = p.1;
//         }
//     });
//
//     // convert from indices to lengths
//     max_x += 1;
//     max_y += 1;
//
//     let width = max_x - min_x;
//     let height = max_y - min_y;
//     let empty = width * height - map.len() as i16;
//     println!("Bounding rect: w: {width} h: {height}   Empty: {empty}");
//     (Pos(min_x, min_y), Pos(max_x, max_y))
// }
//
// fn print(map: &Map) {
//     let rect = bounding_rect(map);
//
//     let offset_x = if rect.0.0 > 0 { -rect.0.0 } else { rect.0.0 };
//     let offset_y = if rect.0.1 > 0 { -rect.0.1 } else { rect.0.1 };
//
//     let width = rect.1.0 - rect.0.0;
//     let height = rect.1.1 - rect.0.1;
//
//     for y in 0..height {
//         for x in 0..width {
//             if map.get(&Pos(x - offset_x, y - offset_y)) == None {
//                 print!(".");
//             } else {
//                 print!("#");
//             }
//         }
//         println!();
//     }
// }
//
// fn vec2map(v: Vec<(Pos, Elf)>) -> Map {
//     let mut map = Map::with_capacity(1500);
//     v.into_iter()
//         .for_each(|(pos, elf)| { map.insert(pos, elf); });
//     map
// }
//
// pub fn twenty_three() {
//     let mut elves = load_map("2022/input/23");
//
//     let mut global_direction = Elf::MoveNorth;
//
//
//     // println!("Initial State!---------------------------------");
//     // print(&elves);
//
//     for round in 1..=1000 {
//         // part1: try moving starting with current global direction, set elf to direction it can move
//
//         let proposed_elves: Vec<(Pos, Elf)> = elves.par_iter().map(|(pos, _)| {
//             let mut can_move_all = true;
//             let mut dir = global_direction;
//             let mut elf = Elf::MoveNone;
//
//             for _i in 0..4 {
//                 let (p, p_left, p_right) = dir.positions_in_direction(&pos);
//
//                 // can't move here if any of the positions in that direction are occupied
//                 if elves.contains_key(&p) || elves.contains_key(&p_left) || elves.contains_key(&p_right) {
//                     can_move_all = false;
//                 } else if elf == Elf::MoveNone { // if this is the first direction we can move in, use it
//                     elf = dir;
//                 }
//
//                 dir = dir.next_direction();
//             }
//
//             // if all directions are open, the elf should stay put
//             if can_move_all {
//                 (*pos, Elf::MoveNone)
//             } else {
//                 (*pos, elf)
//             }
//         }).collect();
//
//         // If all elves wanted to stay put, we're done
//         if proposed_elves.iter().filter(|(_, elf)| *elf != Elf::MoveNone).count() == 0 {
//             println!("Reached round {round} and finished before rounds are over!");
//             break;
//         }
//
//         // part2: move elves if will not collide
//
//         // move all elves without checking, keeping only elves that collide
//         // let fold_map: HashMap<Pos, u16> = ;
//         let prohibited_elves: HashMap<Pos, u16> = proposed_elves
//             .iter()
//             .map(|(pos, elf)| { elf.positions_in_direction(pos).0 })
//             .fold(HashMap::new(), |mut acc, pos| {
//                 if acc.get(&pos) == None {
//                     acc.insert(pos, 1);
//                 } else {
//                     acc.insert(pos, 2);
//                 }
//                 acc
//             }).into_iter()
//             .filter(|(_, num)| *num > 1)
//             .collect();
//
//         // switch to map and clobber any elves that moved to same space, leaving only one.
//         // let unchecked_moved_elves = vec2map(unchecked_moved_elves);
//
//         // Move elves to proposed position if they will not collide with elf in unchecked_moved_elves
//         let moved_elves: Vec<(Pos, Elf)> = proposed_elves
//             .par_iter()
//             .map(|(pos, elf)| {
//                 let proposed_pos = elf.positions_in_direction(pos).0;
//                 if prohibited_elves.contains_key(&proposed_pos) {
//                     (*pos, *elf)
//                 } else {
//                     (proposed_pos, *elf)
//                 }
//             })
//             .collect();
//
//
//
//         elves = vec2map(moved_elves);
//
//         // update global direction to next
//         global_direction = global_direction.next_direction();
//
//         //println!("End of Round {round}!---------------------------------");
//         //print(&elves);
//     }
//
// }


// fn two(first_half: bool) {
//     let file = fs::read_to_string("2022/input/2").expect("Can't find file");
//
//     let mut score = 0;
//     for s in file.split("\n") {
//         let mut round = vec![];
//         for a in s.split(" ") {
//             round.push(a);
//         }
//         let you = round.pop().expect("you");
//         let them = round.pop().expect("them");
//
//
//
//         if first_half {
//             score += if you == "X" {1} else if you == "Y" {2} else {3};
//             // tie
//             if (you == "X" && them == "A") || (you == "Y" && them == "B") || (you == "Z" && them == "C") {
//                 score += 3;
//             }
//             else if (you == "X" && them == "B") || (you == "Y" && them == "C") || (you == "Z" && them == "A") {
//                 score += 0
//             } else {
//                 score += 6;
//             }
//         } else {
//             if you == "X" {
//                 score += 0;
//                 if them == "A" {
//                     score += 3;
//                 } else if them == "B" {
//                     score += 1;
//                 } else {
//                     score += 2;
//                 }
//             } else if you == "Y" {
//                 score += 3;
//                 if them == "A" {
//                     score += 1;
//                 } else if them == "B" {
//                     score += 2;
//                 } else {
//                     score += 3;
//                 }
//             } else {
//                 score += 6;
//                 if them == "A" {
//                     score += 2;
//                 } else if them == "B" {
//                     score += 3;
//                 } else {
//                     score += 1;
//                 }
//
//             }
//
//
//         }
//
//     }
//     println!("{}", score);
// }
//
//
// pub fn one() -> Vec<i32> {
//     let file = fs::read_to_string("2022/input/1").expect("Can't find file");
//
//     let mut elves = vec![];
//
//     elves = file.split("\n\n").map(|s| {
//         s.lines()
//             .map(|calories| calories.parse::<i32>())
//             .filter_map(Result::ok)
//             .sum::<i32>()
//     }).collect();
//
//     elves.sort_by(|a, b| b.cmp(a));
//
//     elves
// }