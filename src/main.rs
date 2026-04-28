use std::io::IsTerminal;

use serde_json::Value;

use clap::Parser;

fn rae_api(word: &str) -> Option<Value> {
    let url = format!("https://rae-api.com/api/words/{}", word);

    let mut response = match ureq::get(&url).call() {
        Ok(resp) => resp,
        Err(ureq::Error::StatusCode(code)) => {
            eprintln!("HTTP error: {}", code);
            return None;
        },
        Err(e) => {
            eprintln!("Request failed: {}", e);
            return None;
        },
    };

    let json: Value = match response.body_mut().read_json() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("JSON error: {}", e);
            return None;
        }
    };

    Some(json)
}

fn print_meanings(json: &Value, colors: &bool){
    let word = json["data"]["word"]
        .as_str()
        .unwrap_or("<unknown>");

    let styled_word = if *colors {
        format!("\x1b[34m\x1b[1m{}\x1b[0m", word)
    } else {
        word.to_string()
    };
    println!("Palabra: {}\n", styled_word);

    let meanings = match json["data"]["meanings"].as_array() {
        Some(m) => m,
        None => {
            eprint!("No se encontraron significados");
            return;
        }
    };

    if let Some(origin) = meanings[0]["origin"]["raw"].as_str() {
        let styled_origin = if *colors {
            format!("\x1b[35m\x1b[3m{}\x1b[0m", origin)
        } else {
            origin.to_string()
        };
        println!("{}\n", styled_origin)
    }

    for meaning in meanings {
        let senses = match meaning["senses"].as_array() {
            Some(s) => s,
            None => continue,
        };
        for sense in senses {
            let num = sense["meaning_number"].as_u64().unwrap_or(0);
            let desc = sense["description"].as_str().unwrap_or("");

            let styled_num = if *colors {
                format!("\x1b[34m\x1b[1m{}\x1b[0m", num)
            } else {
                num.to_string()
            };
            println!("{}. {}", styled_num, desc);

            if let Some(syns) = sense["synonyms"].as_array() {
                let syns_str: Vec<&str> = syns
                    .iter()
                    .filter_map(|s| s.as_str())
                    .collect();
                if !syns_str.is_empty() {
                    let styled_sin = if *colors {
                        format!("\x1b[33m{:>8}:\x1b[0m", "Sin")
                    } else {
                        format!("Sin")
                    };
                    println!("{} {}", styled_sin, syns_str.join(", "));
                }
            }

            if let Some(ant) = sense["antonyms"].as_array() {
                let ant_str: Vec<&str> = ant
                    .iter()
                    .filter_map(|a| a.as_str())
                    .collect();
                if !ant_str.is_empty() {
                    let styled_ant = if *colors {
                        format!("\x1b[33m{:>8}:\x1b[0m", "Ant")
                    } else {
                        format!("Ant")
                    };
                    println!("{} {}", styled_ant, ant_str.join(", "));
                }
            }
        }
    }

}

#[derive(Parser)]
#[command(name = "dict-rae")]
#[command(version = "1.0")]
#[command(
    about = "Consulta el dicionario de la RAE desde tu terminal",
    long_about = None
)]
struct Cli {
    /// Word to look up
    word: String,

    /// Control ANSI color output (default: auto)
    #[arg(short, long, value_parser = ["always", "never", "auto"])]
    colors: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    let colors: bool = match cli.colors.as_deref() {
        Some("always") => true,
        Some("never") => false,
        Some("auto") => std::io::stdout().is_terminal(),
        _ => std::io::stdout().is_terminal(),
    };
    
    if let Some(json) = rae_api(&cli.word) {
        print_meanings(&json, &colors);
    }
}
