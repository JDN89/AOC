use core::num;

// place in tuple
// zip, pairs ,sort, unzip and compare the distances
use regex::Regex;
//working regex: (mul\(\d+\,\d+\))

// from the docs
// let re = Regex::new(r"'([^']+)'\s+\((\d{4})\)").unwrap();
// let hay = "Not my favorite movie: 'Citizen Kane' (1941).";
// let caps = re.captures(hay).unwrap();
//
//let re = Regex::new(r"'(?<title>[^']+)'\s+\((?<year>[0-9]{4})\)").unwrap();
// let hay = "'Citizen Kane' (1941), 'The Wizard of Oz' (1939), 'M' (1931).";
// let mut it = re.captures_iter(hay);

// let caps = it.next().unwrap();
// assert_eq!(&caps["title"], "Citizen Kane");
// assert_eq!(&caps["year"], "1941");
//
//
pub fn part1(input: &str) -> i32 {
    let re = Regex::new(r"(?<operand>mul\((?<nums>\d+\,\d+)\))").unwrap();
    let hay = input;
    let it = re.captures_iter(hay);

    let tuple_numbers: Vec<(i32, i32)> = it
        .map(|cap| {
            let nums: Vec<i32> = cap["nums"]
                .split(',')
                .filter_map(|n| n.parse::<i32>().ok()) // NOTE: filter_map because .ok() converts a Result<T,E> into an Option!!!
                .collect();

            (nums[0], nums[1])
        })
        .collect();
    let mut total = 0;
    for paar in tuple_numbers {
        total += paar.0 * paar.1;
    }
    total
}

//how many times does num appear in right list
//Working regex: /(?<dont>don't\(\))|(?<operand>mul\((?<nums>\d+\,\d+)\))|(?<do>do\(\))/gm
pub fn part2(input: &str) -> i32 {
    let re = Regex::new(r"(?<dont>don't\(\))|(?<operand>mul\((?<nums>\d+\,\d+)\))|(?<do>do\(\))")
        .unwrap();
    let hay = input;
    let it = re.captures_iter(hay);

    enum Event {
        Do,
        Dont,
        Mul(i32, i32),
    }

    let events: Vec<Event> = it
        .filter_map(|cap| {
            if cap.name("do").is_some() {
                Some(Event::Do)
            } else if cap.name("dont").is_some() {
                Some(Event::Dont)
            } else if let Some(nums_match) = cap.name("nums") {
                let nums: Vec<i32> = nums_match
                    .as_str()
                    .split(',')
                    .filter_map(|n| n.parse::<i32>().ok())
                    .collect();
                Some(Event::Mul(nums[0], nums[1]))
            } else {
                None
            }
        })
        .collect();

    let total = events
        .into_iter()
        .fold((true, 0), |(enabled, acc), event| match event {
            Event::Do => todo!(),
            Event::Dont => todo!(),
            Event::Mul(_, _) => todo!(),
        })
        .1;

    0
}
