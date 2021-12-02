use reqwest;
use reqwest::header;
use std::fs;

fn build_client(session: &str) -> reqwest::blocking::Client {
    let cookie = format!("session={}", session);

    let mut headers = header::HeaderMap::new();
    headers.insert(
        "cookie", 
        header::HeaderValue::from_str(&cookie)
            .expect(&format!("invalid header value {}", &cookie)));
        
    reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()
        .expect("Could not create HTTP client")
}

pub fn download_input(session: &str, day: u8) -> String {
    fs::create_dir_all("input")
        .expect("Could not create input directory");

    let filename = format!("input/day-{:02}.txt", day);
    let input = fs::read_to_string(&filename);
    let input = match input {
        Ok(input) => input,
        Err(_) => {
            println!("http request");
            let url = format!("https://adventofcode.com/2021/day/{}/input", day);
            let url = reqwest::Url::parse(&url).expect("valid url");
        
            build_client(session)
                .get(url)
                .send()
                .expect("Could not send GET request")
                .text()
                .expect("Could not parse response")
        }  
    };

    fs::write(&filename, &input)
        .expect("Unable to write file");
    
    input
}