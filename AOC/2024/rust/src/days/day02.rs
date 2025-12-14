// safe if levels increase or decrease by 1 or 2
pub fn part1(input: &str) -> i32 {
    let mut safe_reports = 0;

    for line in input.lines() {
        let report: Vec<i32> = line
            .split_whitespace()
            .filter_map(|n| n.parse::<i32>().ok())
            .collect();

        let mut is_safe = true;

        // Niet mutable want ofwel moeten alle el groter zijn dan het voorgaande element in het
        // rapport, of ze moeten kleiner zijn dan het voorgaande element
        let is_increasing = report[1] > report[0];

        for pair in report.windows(2) {
            // if diff is < 1 || >2 false
            let diff = (pair[1] - pair[0]).abs();
            if !(1..=3).contains(&diff) {
                is_safe = false;
                break;
            }

            if is_increasing && pair[1] < pair[0] {
                is_safe = false;
                break;
            }
            if !is_increasing && pair[1] > pair[0] {
                is_safe = false;
                break;
            }

            is_safe = true
        }
        if is_safe {
            safe_reports += 1;
        }
    }

    safe_reports
}

fn check_is_safe_pair(pair: &[i32], is_increasing: bool) -> bool {
    dbg!(pair);
    // if diff is < 1 || >2 false
    let diff = (pair[1] - pair[0]).abs();
    if !(1..=3).contains(&diff) {
        return false;
    }

    if is_increasing && pair[1] < pair[0] {
        return false;
    }
    if !is_increasing && pair[1] > pair[0] {
        return false;
    }

    true
}

// If removing a single level makes a report safe it's still considered safe
pub fn part2(input: &str) -> i32 {
    let mut safe_reports = 0;

    for line in input.lines() {
        let report: Vec<i32> = line
            .split_whitespace()
            .filter_map(|n| n.parse::<i32>().ok())
            .collect();

        // in my current logic i have to recalculate the is_increasing part because if
        let is_increasing = report[1] > report[0];
        let mut is_safe = true;
        let mut count_is_unsafe = 0;

        dbg!(&report);
        for i in 0..(report.len() - 1) {
            // if i == report.len() - 1 {
            //     break;
            // }

            match check_is_safe_pair(&[report[i], report[i + 1]], is_increasing) {
                true => continue,
                false => {
                    count_is_unsafe += 1;
                    if count_is_unsafe > 1 {
                        is_safe = false;
                        break;
                    }

                    if i + 2 > report.len() - 1 {
                        break;
                    }
                    let first: bool =
                        check_is_safe_pair(&[report[i], report[i + 2]], is_increasing);

                    let third: bool =
                        check_is_safe_pair(&[report[i + 1], report[i + 2]], is_increasing);
                    let second = if i == 0 {
                        true
                    } else {
                        check_is_safe_pair(&[report[i - 1], report[i + 1]], is_increasing)
                    };

                    dbg!(first);
                    // let second: bool =
                    //     check_is_safe_pair(&[report[i + 1], report[i + 2]], is_increasing);

                    // dbg!(second);
                    if first || second || third {
                        continue;
                    } else {
                        is_safe = false;
                        break;
                    }
                }
            }
        }
        if is_safe {
            safe_reports += 1;
        }
    }

    safe_reports
}
