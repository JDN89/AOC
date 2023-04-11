#[derive(Debug, PartialEq)]
pub enum MyError {
    InvalidDigit(char),
}

pub fn to_num(c: char) -> Result<u32, MyError> {
    match c.to_digit(10)

    {
        Some(value) => Ok(value),
        None => Err(MyError::InvalidDigit(c)),
    }
}

pub fn create_grid(input: &str) -> Result<Vec<Vec<u32>>, MyError> {
    let mut grid = vec![];

    for line in input.lines() {
        let row = line
            .chars()
            .map(to_num)
            .collect::<Result<Vec<u32>, MyError>>()?;
        grid.push(row);
    }
    println!("{:?}", grid);
    Ok(grid)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_char_to_num() {
        let input = '4';
        let result = to_num(input);
        assert_eq!(result, Ok(4))
    }
}