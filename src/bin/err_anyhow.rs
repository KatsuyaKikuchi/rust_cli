use anyhow::{Context, Result};

fn get_int_from_file() -> Result<i32> {
    let path = "number.txt";

    let num_str = std::fs::read_to_string(path).
        with_context(|| format!("faild to read string from {}", path))?;

    num_str.trim().parse::<i32>().map(|t| t * 2).context("faild to parse string")
}

fn main()
{
    match get_int_from_file() {
        Ok(x) => println!("{}", x),
        Err(e) => println!("{}", e)
    }
}