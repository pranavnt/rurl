use clap;

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
                .help("Type of request (i.e. GET, POST, etc.) "),
        )
        .arg(clap::Arg::with_name("data")..short("d").long("data").help("Data to send"))
        .arg(
            clap::Arg::with_name("header")
                .short("H")
                .long("header")
                .help("Header to send"),
        )
        .get_matches();
}
