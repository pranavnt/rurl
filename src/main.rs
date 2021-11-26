use clap;
use ureq;

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
    let req = ureq::get(url);

    if headers != "" {
        let header_split: Vec<&str> = headers.split(":").collect();
        req.set(header_split[0], header_split[1]);
    }

    let res = req.call();
    let body = res.unwrap();
}

fn post(url: &str, headers: &str, data: &str) -> String {
    ureq::post(url).set(headers).send_string(data).into_string().unwrap()
}