use std::fs;

#[allow(dead_code)]
pub fn day2(){
    let result_example: u32 = calc_part1("examples/day2.txt");
    assert_eq!(result_example, 15);
    println!("Pass Example");

    let result = calc_part1("inputs/day2.txt");
    println!("{}", result);

    let result_example2 = calc_part2("examples/day2.txt");
    assert_eq!(result_example2, 12);
    println!("Pass Example2");

    let result2 = calc_part2("inputs/day2.txt");
    println!("{}", result2);
}


fn calc_part1(path: &str) -> u32{
    let contents = fs::read_to_string(path).expect("Should be able to read");

    let rounds = contents.split("\n");
    let mut score = 0;
    for round in rounds{
        let choices: Vec<&str> = round.split(" ").collect();
        let oponent = choices[0].as_bytes()[0]- 'A' as u8;
        let my_choice = choices[1].as_bytes()[0] - 'X' as u8;

        let round_score = match (oponent + 3 - my_choice) % 3 {
            0 => my_choice+1 + 3, // draw
            1 => my_choice+1, //loose
            2 => my_choice+1 + 6,
            _ => 255
        };
        score += round_score as u32;
    }

    return score
}

fn calc_part2(path: &str) -> u32 {
    let contents = fs::read_to_string(path).expect("Should be able to read");

    let rounds = contents.split("\n");
    let mut score = 0;
    for round in rounds {
        let choices: Vec<&str> = round.split(" ").collect();
        let oponent = choices[0].as_bytes()[0] - 'A' as u8;
        let my_target = choices[1].as_bytes()[0] - 'X' as u8;

        let round_score = match my_target {
            0 => (oponent + 2) % 3,
            1 => 3 + oponent,
            2 => 6 + (oponent + 1) % 3,
            _ => 255
        } +1;
        score += round_score as u32;


    }
    return score;
}