use std::io::IsTerminal;

use serde_json::Value;

use clap::Parser;

fn rae_api(word: &str) -> Option<Value> {
    let url = format!("https://rae-api.com/api/words/{}", word);

    let response = match reqwest::blocking::get(&url) {
        Ok(resp) => {
            let status = resp.status();
            if !status.is_success() {
                eprint!("HTTP error: {}", status);
                return None;
            }
            resp
        }
        Err(e) => {
            eprintln!("Request failed: {}", e);
            return None;
        }
    };

    let json: Value = match response.json() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("JSON error: {}", e);
            return None;
        }
    };

    Some(json)
}

fn print_meanings(json: &Value, colors: &bool){
    let meanings = match json["data"]["meanings"].as_array() {
        Some(m) => m,
        None => {
            eprint!("No se encontraron significados");
            return;
        }
    };

    let mut i: i32 = 1;
    for meaning in meanings {
        let meaning_count = if *colors{
            format!("\x1b[34mSignificado {}\x1b[0m", i)
        } else {
            format!("Significado {}", i)
        };
        println!("{}", meaning_count);

        if let Some(origin) = meanings[0]["origin"]["raw"].as_str() {
            let styled_origin = if *colors {
                format!("\x1b[3;35m{}\x1b[0m", origin)
            } else {
                origin.to_string()
            };
            println!("{:4}{}", "", styled_origin)
        }

        let senses = match meaning["senses"].as_array() {
            Some(s) => s,
            None => continue,
        };
        for sense in senses {
            let num = sense["meaning_number"].as_u64().unwrap_or(0);
            let desc = sense["description"].as_str().unwrap_or("");

            let styled_num = if *colors {
                format!("\x1b[1;34m{}\x1b[0m", num)
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
                        format!("{:4}\x1b[33m{}:\x1b[0m", "", "Sin")
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
                        format!("{:4}\x1b[33m{}:\x1b[0m", "", "Ant")
                    } else {
                        format!("Ant")
                    };
                    println!("{} {}", styled_ant, ant_str.join(", "));
                }
            }
        }
        println!("");
        i = i + 1;
    }
}

fn print_minimal(json: &Value){
    match json["data"]["meanings"][0]["senses"][0]["raw"].as_str() {
        Some(s) => {
            println!("{}", s)
        }
        None => {
            eprint!("No se encontraron significados");
            return
        }
    };
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

    /// Print minimal output
    #[arg(short, long)]
    minimal: bool,
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
        if cli.minimal {
            print_minimal(&json);
        }
        else {
            print_meanings(&json, &colors);
        }
    }
}
