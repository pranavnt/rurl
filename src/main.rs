use clap;
use cli_table::{format::Justify, print_stdout, Cell, Style, Table};
use regex::Regex;
use serde_json::{Result, Value};
use std::cell::RefCell;
use ureq;
use ansi_term::Colour::{Green, Red, Yellow};

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
        .arg(
            clap::Arg::with_name("form")
                .short("F")
                .long("form")
                .takes_value(true)
                .help("Form data to send")
                .multiple(true),
        )
        .get_matches();

    let url = matches.value_of("url").unwrap();
    let request = matches.value_of("request").unwrap_or("GET");
    let data = matches.value_of("data").unwrap_or("");
    let header = matches.value_of("header").unwrap_or("");

    let form: clap::Values;
    let is_form = matches.is_present("form");

    if is_form {
        form = matches.values_of("form").unwrap();
    } else {
        form = clap::Values::default();
    }

    if request == "GET" {
        get(url, header).print();
    } else if is_form {
        // let res = post(url, form, header);
    } else if request == "POST" {
        let res = post(url, header, data);

        if res.is_ok() {
            let res = res.unwrap();

            println!("{}", res.to_string());
        } else {
            println!("{}", res.unwrap_err());
        }
    } else if request == "PUT" {
        // println!("{}", put(url, data, header))
    } else if request == "DELETE" {
        // println!("{}", delete(url, header))
    } else {
        println!("ERROR");
    }
}

fn get(url: &str, headers: &str) -> Response {
    let req = RefCell::new(ureq::get(url));

    if headers != "" {
        let headers: Vec<&str> = headers.split(",").collect();
        for header in headers {
            let header: Vec<&str> = header.split(":").collect();
            req.borrow_mut().clone().set(header[0], header[1]);
        }
    }

    let res = req.borrow_mut().clone().call().unwrap();

    let resp_header_names = res.headers_names();

    let mut resp_headers: Vec<(String, String)> = Vec::<(String, String)>::new();

    for header in resp_header_names {
        let header_value = res.header(&header).unwrap();
        resp_headers.push((header.to_string(), header_value.to_string()));
    }

    let response: Response = Response {
        status_code: res.status().to_string(),
        status_text: res.status_text().to_string(),
        body: res.into_string().unwrap(),
        headers: resp_headers,
    };

    // body
    response
}

struct Response {
    status_code: String,
    status_text: String,
    headers: Vec<(String, String)>,
    body: String,
}

impl Response {
    fn print(&self) {
        let color = match self.status_code.as_str() {
            "200" => Green,
            "201" => Green,
            "202" => Green,
            "204" => Green,
            "301" => Red,
            "302" => Red,
            "304" => Red,
            "400" => Red,
            "401" => Red,
            "403" => Red,
            "404" => Red,
            "405" => Red,
            "406" => Red,
            "408" => Red,
            "409" => Red,
            "410" => Red,
            "411" => Red,
            "412" => Red,
            "413" => Red,
            "414" => Red,
            "415" => Red,
            "416" => Red,
            "417" => Red,
            "500" => Red,
            "501" => Red,
            "502" => Red,
            "503" => Red,
            "504" => Red,
            "505" => Red,
            _ => Yellow,
        };

        // use the cli table library to display the status code and status text
        println!("{}", color.bold().paint(format!("Status Code: {}", self.status_code.clone())));
        println!("{}", color.bold().paint(format!("Status Text: {}", self.status_text.clone())));

        let mut table_data = vec![];

        for (header, value) in &self.headers {
            table_data.push(vec![header.clone().cell(), value.clone().cell()]);
        }

        let table = table_data.table().bold(true);

        print_stdout(table).unwrap();

        // print the body
        println!("{}", Green.underline().bold().paint("Body"));
        println!("{}", self.body);
    }
}

fn post(url: &str, headers: &str, data: &str) -> Result<String> {
    let req = RefCell::new(ureq::post(url));

    if headers != "" {
        let headers: Value = serde_json::from_str(headers).unwrap();
        let headers = headers.as_object().unwrap();

        for (key, value) in headers {
            req.borrow_mut().clone().set(key, &value.to_string());
        }
    }

    let json: Value = serde_json::from_str(data)?;

    match req.borrow_mut().clone().send_json(json) {
        Err(e) => {
            return Err(e).unwrap();
        }
        Ok(res) => {
            return Ok(res.into_string().unwrap());
        }
    };
}
