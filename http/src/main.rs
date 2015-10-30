//! HTTP get request example
//! http://rosettacode.org/wiki/HTTP#Rust

extern crate hyper;

use std::io::Read;
use hyper::client::Client;

fn main() {
    let client = Client::new();
    let mut resp = client.get("http://rosettacode.org").send().unwrap();
    let mut body = String::new();
    resp.read_to_string(&mut body).unwrap();
    println!("{}", body);
}
