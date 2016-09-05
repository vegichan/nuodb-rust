use std::io::prelude::*;
use std::net::TcpStream;
use std::io::Error;
use std::mem;
use std::mem::transmute;
use std::str;

extern crate regex;
use regex::Regex;


fn read_address(xml: &str) -> (Option<&str>, Option<u32>) {
    let mut address = None;
    let mut port = None;
    let re = Regex::new(r"^<(\w+) (.*)/>$").unwrap();
    let element_re = Regex::new(r#"^(\w+)="([^"]*)"$"#).unwrap();
    assert!(re.is_match(xml));
    for cap in re.captures_iter(xml) {
        assert!(cap.at(1).unwrap() == "Cloud");
        for attr in cap.at(2).unwrap().split(" ") {
            assert!(element_re.is_match(attr));
            for cap_attr in element_re.captures_iter(attr) {
                match cap_attr.at(1).unwrap() {
                    "Address" => {
                        address = Some(cap_attr.at(2).unwrap());
                    },
                    "Port" => {
                        port = Some(cap_attr.at(2).unwrap().parse::<u32>().unwrap());
                    },
                    _ => {
                        println!("found unknown param {}", cap_attr.at(1).unwrap());
                    }
                }
            }
        }
    }
    (address, port)
}

fn create_connection() -> Result<(), Error> {
    let mut stream = TcpStream::connect("127.0.0.1:48004").unwrap();

    let connect_string = b"<Connect Service=\"SQL2\" Database=\"test\"/>"; 
    let size_of_connection = mem::size_of_val(connect_string) as u32;
    let mut bytes: [u8; 4] = unsafe { transmute(size_of_connection) };
    bytes.reverse();

    let mut v = Vec::new();
    v.extend(bytes.iter().cloned());
    v.extend(connect_string.iter().cloned());

    try!(stream.write_all(&v));

    let mut buffer = Vec::new();
    try!(stream.read_to_end(&mut buffer));

    let answer = str::from_utf8(&buffer[4..]).unwrap();

    let (addr, port) = read_address(answer.trim());
    println!("Connect to engine {}:{}", addr.unwrap(), port.unwrap());

    Ok(())
}

fn main() {
    create_connection().unwrap();
}
