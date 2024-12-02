use std::{fs::{self, File}, io::Write};
use reqwest::header::COOKIE;

pub fn download_input(day: u8, year: &str) {
    let session_token = fs::read_to_string("session_token").expect("Failed to read session token from the \"session_token\" file");
    let client = reqwest::blocking::Client::new();
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let resp = client.get(&url)
        .header(COOKIE, format!("session={session_token}",))
        .send()
        .expect("Request Failed");

    if resp.status().is_success() {
        let body = resp.text().expect("Body is invalid");
        let mut out = File::create(format!("inputs/day{}.txt", day)).expect("Failed to make file");
        out.write_all(body.as_bytes()).expect("Failed to write to file");
    } else {
        eprintln!("Failed to download input: {}", resp.status());
    }

    let url = format!("https://adventofcode.com/{}/day/{}", year, day);
    let resp = client.get(&url)
        .header(COOKIE, format!("session={session_token}",))
        .send()
        .expect("Request Failed");

    if resp.status().is_success() {
        let body = resp.text().expect("Body is invalid");
        // Filter the body to hopefully get the test input data
        let body = body.split("<pre><code>").nth(1).unwrap_or_else(|| {println!("Failed to get test input"); ""}).split("</code></pre>").nth(0).unwrap_or_else(|| {println!("Failed to get test input"); ""}).to_string();
        let mut out = File::create_new(format!("inputs_tests/day{}.txt", day)).expect("Failed to make the test input file");
        out.write_all(body.as_bytes()).expect("Failed to write to file");
    } else {
        eprintln!("Failed to download test input: {}", resp.status());
    }
}