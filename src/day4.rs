use std::fs;

pub fn day4(){
    let result_example: u32 = calc_part1("examples/day4.txt");
    assert_eq!(result_example, 2);
    println!("Pass Example");

    let result = calc_part1("inputs/day4.txt");
    println!("{}", result);

    let result_example2 = calc_part2("examples/day4.txt");
    assert_eq!(result_example2, 4);
    println!("Pass Example 2");

    let result2 = calc_part2("inputs/day4.txt");
    println!("{}", result2);
}


fn calc_part1(path: &str) -> u32 {
    let contents = fs::read_to_string(path).expect("Should be able to read");
    let lines = contents.split("\n");

    let mut counter = 0;
    for line in lines {

        let mut elves = line.split(',').map(|elf| {
            let mut limits = elf.split('-').map(|num: &str| num.parse::<u32>().unwrap());
            [limits.next().unwrap(), limits.next().unwrap()]
        });
        let elves = [elves.next().unwrap(), elves.next().unwrap()];

        let r0 = elves[0][0]..elves[0][1] + 1;
        let r1 = elves[1][0]..elves[1][1] + 1;

        if (r0.contains(&elves[1][0]) && r0.contains(&elves[1][1])) || // elf 1 in 0
            (r1.contains(&elves[0][0]) && r1.contains(&elves[0][1])) { // elf 0 in 1
            counter += 1
        }
    }

    return counter
}

fn calc_part2(path: &str) -> u32 {
    let contents = fs::read_to_string(path).expect("Should be able to read");
    let lines = contents.split("\n");

    let mut counter = 0;
    for line in lines {

        let mut elves = line.split(',').map(|elf| {
            let mut limits = elf.split('-').map(|num: &str| num.parse::<u32>().unwrap());
            [limits.next().unwrap(), limits.next().unwrap()]
        });
        let elves = [elves.next().unwrap(), elves.next().unwrap()];

        let r0 = elves[0][0]..elves[0][1] + 1;
        let r1 = elves[1][0]..elves[1][1] + 1;

        // does any range contain the start end of the other elf
        if r0.contains(&elves[1][0]) ||
            r0.contains(&elves[1][1]) ||
            r1.contains(&elves[0][0]) ||
            r1.contains(&elves[0][1]) {
            counter += 1
        }
    }
    return counter;

}