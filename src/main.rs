extern crate reqwest;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("usage : cargo run <url to check>");
        return;
    }
    let url_to_check = &args[1];

    println!("checking url : {:?}!", url_to_check);
    let res = check_url(url_to_check.to_string());
    println!("result : {:?}!", res);
}

fn check_url(url: String) -> bool {
	let response = reqwest::get(&url).unwrap();
	response.status().is_success()
}
