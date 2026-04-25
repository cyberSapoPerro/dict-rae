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

    let body = match response.body_mut().read_to_string() {
        Ok(b) => b,
        Err(e) => {
            eprintln!("Read error: {}", e);
            return;
        }
    };

    println!("{}", body);
}


fn main() {
    let args: Vec<String> = std::env::args().collect();
    let word: &str = &args[1];

    rae_api(&word);
}
