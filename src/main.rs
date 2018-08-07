#![warn(rust_2018_idioms)]
#![feature(proc_macro_non_items)]

mod html;
use crate::html::append_dir;

use std::env;
use std::fs::{read_dir, File};
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::str;
use std::thread;

const FAV: &[u8; 318] = include_bytes!("favicon.ico");

fn main() {
    let listner = TcpListener::bind("0.0.0.0:8000").unwrap();

    let args = env::args();
    let args: Vec<String> = args.skip(1).collect();

    for stream in listner.incoming() {
        let stream = stream.unwrap();
        let args = args.clone();

        thread::spawn(move || {
            handle_connection(stream, &args);
        });
    }
}

fn read_to_str(path: &Path) -> Vec<String> {
    let paths = read_dir(path).unwrap();
    let paths: Vec<String> = paths
        .map(|x| x.unwrap().path().to_str().unwrap().to_string())
        .collect();
    paths
}
fn handle_connection(mut stream: TcpStream, args: &[String]) {
    let mut buffer = [0; 512];

    let _ = stream.read(&mut buffer).unwrap();

    let mut contents = Vec::new();
    let status;

    match check_request(&buffer).as_str() {
        "/" => {
            if args.is_empty() {
                let root_dir = Path::new(".");
                let paths = read_to_str(root_dir);
                append_dir(&paths, true);
            } else {
                append_dir(&args, true);
            }

            let mut file = File::open("dir_toy_server.html").unwrap();
            file.read_to_end(&mut contents).unwrap();
            status = String::from("HTTP/1.1 200 OK\r\n\r\n");
        }
        "/favicon.ico" => {
            contents = FAV.to_vec();
            status = String::from("HTTP/1.1 200 OK\r\n\r\n");
        }
        x => {
            let path = Path::new(&x[1..]);
            if path.is_dir() {
                let paths = read_to_str(path);
                append_dir(&paths, false);
                let mut file = File::open("dir_toy_server.html").unwrap();
                file.read_to_end(&mut contents).unwrap();
                status = String::from("HTTP/1.1 200 OK\r\n\r\n");
            } else {
                let mut file = File::open(path).unwrap();
                file.read_to_end(&mut contents).unwrap();
                status = String::from("HTTP/1.1 200 OK\r\n\r\n");
            }
        }
    }

    let mut response = status.as_bytes().to_vec();
    response.extend(contents);
    stream.write_all(&response).unwrap();
    stream.flush().unwrap();
}

fn check_request(b: &[u8; 512]) -> String {
    let mut s = str::from_utf8(b).unwrap().to_string();
    if s.find("GET").is_none() {
        panic!("Not a GET request");
    }
    let slash_offset = s.find('/').unwrap();
    let http_offset = s.find("HTTP").unwrap() - 5;
    s.replace_range(..slash_offset, "");
    s.replace_range(http_offset.., "");
    s
}
