use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args);

    let query = config.as_ref().unwrap().query.clone();
    let file_path = config.as_ref().unwrap().file_path.clone();

    println!("Searching for {}", query);
    println!("In file {}", file_path);

    let contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

struct Config 
{
    query: String,
    file_path: String,
}

impl Config 
{
    fn build(args: &[String]) -> Result<Config, &'static str>
    {
        if args.len() < 3 
        {
            return Err("Not enough arguments!");
        };

        Ok(Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
        })
    }
}
