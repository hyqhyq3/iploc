extern crate hyper;
use hyper::Client;
use hyper::client::IntoUrl;
use std::fs::File;
use std::io::{Read, Write};
use std::io;
use std::path::Path;

fn get_content(url: &str) -> hyper::Result<String> {
    let client = Client::new();
    let mut resp = client.get(url).send().unwrap();
    let mut content = String::new();
    try!(resp.read_to_string(&mut content));
    return Ok(content);
}

fn file_get_content(file: &str) -> Result<String, io::Error> {
    let mut content = String::new();
    try!(File::open(file).unwrap().read_to_string(&mut content));
    Ok(content)
}

fn main() {
    let filename = "data.txt";
    if !Path::new(filename).exists() {
        let url = "http://ftp.apnic.net/stats/apnic/delegated-apnic-latest";
        let body = get_content(url).unwrap();
        let mut file = File::create(filename).unwrap();
        file.write_all(body.as_bytes()).unwrap();
    }
    let content = file_get_content(filename).unwrap();
    let lines = content.lines();
    for line in lines {
        match line.starts_with('#') {
            true => (),
            false => {
                let fields: Vec<_> = line.split("|").collect();
                if fields.len() > 6 && fields[2] == "asn" {
                    println!("Hello, world! {} {} {}", fields[1], fields[3], fields[5]);
                }
            }
        }

    }
}
