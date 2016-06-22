extern crate hyper;
use self::hyper::Client;
use std::io;
use std::io::{Read, Write};
use std::fs::File;
use std::path::Path;
use util::ToUl;

struct Record {
    start: u32,
    size: u32,
    country: String,
}

pub struct IpTable {
    records: Vec<Record>,
    filename: &'static str,
    url: &'static str,
}

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


impl IpTable {
    pub fn new() -> IpTable {
        IpTable {
            records: vec![],
            filename: "data.txt",
            url: "http://ftp.apnic.net/stats/apnic/delegated-apnic-latest",
        }
    }

    pub fn init(&mut self) {
        if !Path::new(self.filename).exists() {
            let body = get_content(self.url).unwrap();
            let mut file = File::create(self.filename).unwrap();
            file.write_all(body.as_bytes()).unwrap();
        }
        let content = file_get_content(self.filename).unwrap();
        let lines = content.lines();
        for line in lines {
            match line.starts_with('#') {
                true => (),
                false => {
                    let fields: Vec<_> = line.split("|").collect();
                    if fields.len() > 6 && fields[2] == "ipv4" {
                        let mut record = Record {
                            size: 0,
                            start: 0,
                            country: String::new(),
                        };
                        record.size = fields[4].parse::<u32>().unwrap();
                        record.start = fields[3].to_ul().unwrap();
                        record.country = fields[1].to_owned();
                        self.records.push(record);
                    }
                }
            }

        }
        self.records.sort_by_key(|k| k.start);
    }

    pub fn get_country(&self, ip: &str) -> String {
        let ul = ip.to_ul().unwrap();
        for record in &self.records {
            if ul < record.start + record.size {
                return match ul >= record.start {
                    true => record.country.clone(),
                    false => String::from("unknown") 
                }
            }
        }
        String::from("unknown")
    }
}