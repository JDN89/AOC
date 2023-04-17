
pub fn receive_input(input: &str)  {
    input.lines().for_each(|line| {
        println!("{:?}", line);
    });
}