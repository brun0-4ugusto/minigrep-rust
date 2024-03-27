use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String
}

impl Config {
   pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Sem argumentos suficientes")
        }
        return Ok(Config {
            query: args[1].clone(),
            file_path: args[2].clone()
        });
    }
}

pub fn run(args: Config) -> Result<(),Box<dyn Error>> {
    let contents = fs::read_to_string(args.file_path)?;

    let resultados = search(&args.query, &contents);

    for resultado in resultados {
        println!("{}", resultado)
    }
    Ok(())
}

pub fn search<'a>(query: &'a str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in content.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            results.push(line);
        }
    }
    results
}