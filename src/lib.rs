use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub is_case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let is_case_sensitive = std::env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            is_case_sensitive,
        })
    }
}

fn search<'a>(word: &str, content: &'a str) -> Vec<&'a str> {
    let mut found = Vec::new();

    for line in content.lines() {
        if line.contains(&word) {
            found.push(line);
        }
    }

    found
}

fn search_case_insensitive<'a>(word: &str, content: &'a str) -> Vec<&'a str> {
    let lowercased = word.to_lowercase();

    let mut found = Vec::new();

    for line in content.lines() {
        if line.to_lowercase().contains(&lowercased) {
            found.push(line);
        }
    }

    found
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;

    let result = if config.is_case_sensitive {
        search(&config.query, &content)
    } else {
        search_case_insensitive(&config.query, &content)
    };

    println!("Results are:\n{:?}", result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let query = "nika";
        let content = "Soy un ombre
myonrado
kemegusta
lomexor
nika nika prika prika
martlacada nikao
nika ra ari axla es
gamagebine erti
nika's saxli ras aketebs netavi";

        let expected = vec![
            "nika nika prika prika",
            "martlacada nikao",
            "nika ra ari axla es",
            "nika's saxli ras aketebs netavi",
        ];
        let outcome = search(&query, &content);

        assert_eq!(expected, outcome);
    }

    #[test]
    fn test2() {
        let query = "Nika";
        let content = "Soy un ombre
myonrado
kemegusta
lomexor
nika nika prika prika
martlacada nikao
niKa ra ari axla es
gamagebine erti
nika's saxli ras aketebs netavi";

        let expected = vec![
            "nika nika prika prika",
            "martlacada nikao",
            "niKa ra ari axla es",
            "nika's saxli ras aketebs netavi",
        ];
        let outcome = search_case_insensitive(&query, &content);

        assert_eq!(expected, outcome);
    }
}
