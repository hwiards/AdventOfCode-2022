use std::fs;



pub fn day4(){
    let result_example: u32 = calc_part1("examples/day4.txt");
    assert_eq!(result_example, 2);
    println!("Pass Example");

    let result = calc_part1("inputs/day4.txt");
    println!("{}", result);

    let result_example2 = calc_part2("examples/day4.txt");
    assert_eq!(result_example2, 4);
    println!("Pass Example2");

    let result2 = calc_part2("inputs/day4.txt");
    println!("{}", result2);
}


fn calc_part1(path: &str) -> u32 {
    let contents = fs::read_to_string(path).expect("Should be able to read");
    let lines = contents.split("\n");

    let mut counter = 0;
    for line in lines {
        let (p1, p2) = line.split_once(',').unwrap();
        let (p1s, p1e) = p1.split_once("-").unwrap();
        let (p2s, p2e) = p2.split_once("-").unwrap();

        let p1si = p1s.parse::<u32>().unwrap();
        let p1ei = p1e.parse::<u32>().unwrap();
        let p2si = p2s.parse::<u32>().unwrap();
        let p2ei = p2e.parse::<u32>().unwrap();

        let r1 = p1si..p1ei+1;
        let r2 = p2si..p2ei+1;

        if (r1.contains(&p2si) && r1.contains(&p2ei)) || (r2.contains(&p1si) && r2.contains(&p1ei)) {
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
        let (p1, p2) = line.split_once(',').unwrap();
        let (p1s, p1e) = p1.split_once("-").unwrap();
        let (p2s, p2e) = p2.split_once("-").unwrap();

        let p1si = p1s.parse::<u32>().unwrap();
        let p1ei = p1e.parse::<u32>().unwrap();
        let p2si = p2s.parse::<u32>().unwrap();
        let p2ei = p2e.parse::<u32>().unwrap();

        let r1 = p1si..p1ei + 1;
        let r2 = p2si..p2ei + 1;

        if (r1.contains(&p2si) || r1.contains(&p2ei)) || (r2.contains(&p1si) || r2.contains(&p1ei)) {
            counter += 1
        }
    }
    return counter;

}