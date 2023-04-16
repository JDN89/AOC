mod grid;

    fn main() {

        let input =  read_input().unwrap();
        grid::create_grid(&input);

    }

    fn read_input() -> Result<String,std::io::Error> {
        Ok(std::fs::read_to_string("src/test.txt")?)

    }
