use clap;
use ureq;
use std::cell::RefCell;

fn main() {
    let matches = clap::App::new("rurl")
        .version("0.0.1")
        .author("Pranav Teegavarapu <pranavnt@outlook.com>")
        .about("Curl but in Rust")
        .arg(
            clap::Arg::with_name("url")
                .help("Sets the url to use")
                .required(true)
                .index(1),
        )
        .arg(
            clap::Arg::with_name("request")
                .short("X")
                .long("request")
                .takes_value(true)
                .help("Type of request (i.e. GET, POST, etc.) "),
        )
        .arg(
            clap::Arg::with_name("data")
                .short("d")
                .long("data")
                .takes_value(true)
                .help("Data to send"),
        )
        .arg(
            clap::Arg::with_name("header")
                .short("H")
                .long("header")
                .takes_value(true)
                .help("Header to send"),
        )
        .get_matches();

    let url = matches.value_of("url").unwrap();
    let request = matches.value_of("request").unwrap_or("GET");
    let data = matches.value_of("data").unwrap_or("");
    let header = matches.value_of("header").unwrap_or("");





    println!("{:?}", matches)
}

fn get(url: &str, headers: &str) -> (String) {
    let req = RefCell::new(ureq::get(url));

    if headers != "" {
        let headers: Vec<&str> = headers.split(",").collect();
        for header in headers {
            let header: Vec<&str> = header.split(":").collect();
            req.borrow_mut().clone().set(header[0], header[1]);
        }
    }

    let res = req.borrow_mut().clone().call();
    let body = res.unwrap().into_string().unwrap();

    body
}

fn post(url: &str, headers: &str, data: &str) -> String {
    // ureq::post(url).set(headers).send_string(data).into_string().unwrap()
    "hi".to_string()
}