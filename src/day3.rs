use std::fs;
use std::collections::HashSet;

#[allow(dead_code)]
pub fn day3(){
    let result_example: u32 = calc_part1("examples/day3.txt");
    assert_eq!(result_example, 157);
    println!("Pass Example");

    let result = calc_part1("inputs/day3.txt");
    println!("{}", result);

    let result_example2 = calc_part2("examples/day3.txt");
    assert_eq!(result_example2, 70);
    println!("Pass Example 2");

    let result2 = calc_part2("inputs/day3.txt");
    println!("{}", result2);
}


fn calc_part1(path: &str) -> u32 {
    let contents = fs::read_to_string(path).expect("Should be able to read");
    let lines = contents.split("\n");

    let mut score = 0;

    for line in lines{
        let len = line.len();
        let first = &line[0..len/2];
        let second = &line[len/2..len];
        let first_set: HashSet<char> = HashSet::from_iter(first.chars());
        let second_set: HashSet<char> = HashSet::from_iter(second.chars());

        let intersec: &char = first_set.intersection(&second_set).collect::<Vec<&char>>()[0];
        let line_score = match *intersec {
            'a'..='z' => *intersec as u32 - 96,
            _ => *intersec as u32 - 64 + 26
        };
        score += line_score;
    }
    return score;
}

fn calc_part2(path: &str) -> u32 {
    let contents = fs::read_to_string(path).expect("Should be able to read");
    let lines: Vec<&str> = contents.split("\n").collect();

    let mut score = 0;
    for i in (0..lines.len()).step_by(3) {
        let l1: HashSet<char> = HashSet::from_iter(lines[i].chars());
        let l2: HashSet<char> = HashSet::from_iter(lines[i+1].chars());
        let l3: HashSet<char> = HashSet::from_iter(lines[i+2].chars());
        let intersect1 :HashSet<char> = l1.intersection(&l2).copied().collect();
        let intersect2 = l3.intersection(&intersect1).collect::<Vec<&char>>()[0];
        let line_score = match *intersect2 {
            'a'..='z' => *intersect2 as u32 - 96,
            _ => *intersect2 as u32 - 64 + 26
        };
        score += line_score;

    }

    return score
}
