
use std::fs;

#[allow(dead_code)]
pub fn day1() {
    let contents = fs::read_to_string("inputs/day1.txt").expect("Should be able to read");
    let elves_vec = contents.split("\n\n").collect::<Vec<&str>>();

    let mut calories_total : Vec<i32> = Vec::new();

    for elve in elves_vec {
        let calories = elve.split("\n");
        let calories_int :Vec<i32> = calories.map(|x| i32::from_str_radix(x, 10).expect("a")).collect();
        calories_total.push(calories_int.iter().sum());
    }

    calories_total.sort();
    calories_total.reverse();
    println!("Part 1: {}", calories_total[0]);
    println!("Part 2: {}", calories_total[0] + calories_total[1] +calories_total[2]);

}
