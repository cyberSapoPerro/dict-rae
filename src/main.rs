use serde_json::Value;

fn rae_api(word: &str) {
    let url = format!("https://rae-api.com/api/words/{}", word);

    let mut response = match ureq::get(&url).call() {
        Ok(resp) => resp,
        Err(ureq::Error::StatusCode(code)) => {
            eprintln!("HTTP error: {}", code);
            return;
        },
        Err(e) => {
            eprintln!("Request failed: {}", e);
            return;
        },
    };

    let json: Value = match response.body_mut().read_json() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("JSON error: {}", e);
            return;
        }
    };

    print_meanings(&json);
}

fn print_meanings(json: &Value){
    println!("\n");
    let word = json["data"]["word"]
        .as_str()
        .unwrap_or("<unknown>");

    println!("Palabra: \x1b[34m\x1b[1m{}\x1b[0m\n", word);

    let meanings = match json["data"]["meanings"].as_array() {
        Some(m) => m,
        None => {
            eprint!("No se encontraron significados");
            return;
        }
    };

    for meaning in meanings {
        let senses = match meaning["senses"].as_array() {
            Some(s) => s,
            None => continue,
        };
        for sense in senses {
            let num = sense["meaning_number"].as_u64().unwrap_or(0);
            let desc = sense["description"].as_str().unwrap_or("");

            println!("\x1b[34m\x1b[1m{}.\x1b[0m {}", num, desc);

            if let Some(syns) = sense["synonyms"].as_array() {
                let syns_str: Vec<&str> = syns
                    .iter()
                    .filter_map(|s| s.as_str())
                    .collect();
                if !syns_str.is_empty() {
                    println!("\x1b[33m{:>8}:\x1b[0m {}", "Sin", syns_str.join(", "));
                }
            }

            if let Some(ant) = sense["antonyms"].as_array() {
                let ant_str: Vec<&str> = ant
                    .iter()
                    .filter_map(|a| a.as_str())
                    .collect();
                if !ant_str.is_empty() {
                    println!("\x1b[33m{:>8}:\x1b[0m {}", "Ant", ant_str.join(", "));
                }
            }
        }
    }

}

fn main() {
    let word = std::env::args().nth(1).unwrap_or_else(|| {
            eprint!("Uso: dict-rae <palabra>");
            std::process::exit(1);
        });

    rae_api(&word);
}
