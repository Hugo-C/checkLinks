#[macro_use] extern crate lazy_static;
#[macro_use] extern crate prettytable;

extern crate regex;
extern crate reqwest;
use std::env;

use regex::Regex;
use prettytable::{Table, Row, Cell};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("usage : cargo run <link to check>");
        return;
    }
    let link_to_check = &args[1];

    println!("checking link : {:?}!", link_to_check);
    let res = retrieve_html(link_to_check.to_string());
    let links = retrieve_links_from_html(&res, link_to_check.to_string());
    println!("link : {:?}!", links);


    let status = are_websites_up(&links);
    let sub_table = get_sub_table(links, status);
    let table = get_table(link_to_check, !res.is_empty(), sub_table);
    table.printstd();
}

fn retrieve_html(link: String) -> String {
    let mut response = reqwest::get(&link).unwrap();
    response.text().unwrap()
}

fn retrieve_links_from_html(text: &str, from_link: String) -> Vec<&str> {
    lazy_static! {  // build the regex only once
        static ref RE: Regex = Regex::new(r#"<a [^>]* href="([^"]+)"([^>]|\s)*>"#).unwrap();
    }

    let mut res = Vec::new();
    for link in RE.captures_iter(&text) {
        res.push(link.get(1).unwrap().as_str());
    }
    handle_relative_links(&res, from_link);
    res
}

fn handle_relative_links(links: &Vec<&str>, base_link: String) {
    for mut link in links.iter() {
        println!("link : {:?}", link);
        if link.starts_with("/") {  // check if the link is a relative link
            println!("BASE !");
            let mut tmp = base_link.clone();
            tmp.push_str(link);
            link = &(tmp.as_str());
            println!("link changed : {:?}", link);
        }
    }
}

fn is_website_up(link: &str) -> bool {
    let resp = reqwest::get(link);

    match resp {
        Err(e) => {
            println!("err ({:?})", e);
            false
        },
        Ok(r) => {
            if r.status().is_success() {
                println!("success!");
                true
            } else if r.status().is_server_error() {
                println!("server error!");
                false
            } else {
                println!("Something else happened. Status: {:?}", r.status());
                false
            }
        }
    }
}

fn are_websites_up(links: &Vec<&str>) -> Vec<bool> {    // TODO create Client connection for reuse
    let mut res = Vec::new();
    for link in links {
        res.push(is_website_up(link));
    }
    res
}

fn get_sub_table(links: Vec<&str>, status: Vec<bool>) -> Table {
    // Create the table
    let mut table = Table::new();
    table.add_row(row!["Links", "Status"]);

    for i in 0..links.len() {
        let string_status;
        if status[i] {
            string_status = "✔"
        } else {
            string_status = "❌"
        }
        table.add_row(row![links[i], string_status]);
    }
    table
}


fn get_table(link: &str, status: bool, sub_table: Table) -> Table {
    println!("link : {:?}!", link);
    // Create the table
    let mut table = Table::new();
    table.add_row(row!["Links", "Status", "Sublinks"]);
    let string_status;
    if status {
        string_status = "✔"
    } else {
        string_status = "❌"
    }
    table.add_row(row![link, string_status, sub_table]);
    table
}

// TODO use multithreading to retrieve link
// TODO handle local links
// TODO allow the user to specify n level of links to follow
// TODO customize table colors / formats
