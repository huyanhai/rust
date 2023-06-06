use std::{env, error::Error, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Self, &str> {
        // 获取环境变量，有就返回true,没有就返回false
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<String, Box<dyn Error>> {
    let ctx: String = fs::read_to_string(config.file_path)?;
    Ok(ctx)
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
