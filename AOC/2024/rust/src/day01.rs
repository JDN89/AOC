pub fn parse_input_day01(input: &str) {
    let lines: Vec<Vec<char>> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            line.chars()
                .filter(|c| !c.is_whitespace())
                .collect::<Vec<char>>()
        })
        .collect();

    let mut count = 0;
    for line in lines {
        println!("line number {}", count);

        for num in line {
            println!("num: {}", num);
        }
        count += 1;
    }
}
