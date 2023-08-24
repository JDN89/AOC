// find complete overlap
// range rust
// https://doc.rust-lang.org/std/ops/struct.Range.html
// make 2 ranges -> rang1 and range 2
// range1.contains rang2.start && range2.end
//eazy peasy?
//
//steps: split at ,
//convert 2-3 to a tuple ;
//implement tryFrom for Range (value:(i32,i32)) -> Result<Self,Self::Error>

//nom use:
//map_res = convert to i32
// seperated pair -> to convert to range Ok((remiaing_input,start..end))

pub fn process_part1(input: &str) -> u32 {}

#[cfg(test)]
mod tests {
    use super::*;

    // Define a constant input for ease of use
    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_day1_part1() {
        assert_eq!(process_part1(INPUT), 2);
    }

    // #[test]
    // fn test_day1_part2() {
    //     assert_eq!(process_part2(INPUT), 45000);
    // }
}
