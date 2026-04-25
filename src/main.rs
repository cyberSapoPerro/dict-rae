fn rae_api(word: &str) -> Result<(), ureq::Error> {
    let url = format!("https://rae-api.com/api/words/{}", word);
    let body = ureq::get(&url)
        .call()?
        .body_mut()
        .read_to_string()?;

    println!("{}", body);

    Ok(())
}


fn main() {
    let args: Vec<String> = std::env::args().collect();
    let word: &str = &args[1];

    if let Err(e) = rae_api(word) {
        eprintln!("Error: {}", e);
    }
}
