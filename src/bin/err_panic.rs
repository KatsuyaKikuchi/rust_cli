fn get_int_from_file() -> Result<i32, String> {
    let path = "number.txt";

    let num_str = std::fs::read_to_string(path).map_err(|e| e.to_string())?;

    num_str.trim().parse::<i32>().map(|n| 2 * n).map_err(|e| e.to_string())
}

fn main()
{
    match get_int_from_file() {
        Ok(num) => println!("num is {}", num),
        Err(e) => println!("{}", e)
    };
}