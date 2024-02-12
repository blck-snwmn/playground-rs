use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

type MyResult<T> = Result<T, Box<dyn Error>>;

fn main() -> MyResult<()> {
    {
        // infinite loop if use filter_mapthe and the path value is an existing directory
        let path = ".";
        let mut lines = BufReader::new(File::open(path)?)
            .lines()
            .filter_map(Result::ok);
        let _ = lines.next();
    }
    {
        // infinite loop if use filter_mapthe and the path value is an existing directory
        let path = ".";
        let mut lines = BufReader::new(File::open(path)?)
            .lines()
            .map_while(Result::ok);
        let _ = lines.next();
    }
    Ok(())
}
