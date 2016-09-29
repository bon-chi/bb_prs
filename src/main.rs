extern crate rustbox;
extern crate hyper;

use std::error::Error;
use std::default::Default;

use rustbox::{Color, RustBox};
use rustbox::Key;

use hyper::client::Client;
use std::io::Read;
// use hyper::Client;
use hyper::header::{Headers, Authorization, Basic};

fn main() {
    send_request();
    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    let mut query = String::new();

    rustbox.print(0,
                  0,
                  rustbox::RB_BOLD,
                  Color::White,
                  Color::Black,
                  "bb_prs > type your ticket name");
    rustbox.print(1,
                  3,
                  rustbox::RB_BOLD,
                  Color::White,
                  Color::Black,
                  "Press 'q' to quit.");
    rustbox.present();
    loop {
        rustbox.clear();
        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Char('q') => {
                        break;
                    }
                    Key::Char(c) => {
                        query.push(c);
                        rustbox.print(0,
                                      0,
                                      rustbox::RB_NORMAL,
                                      Color::Default,
                                      Color::Default,
                                      &query)
                    }
                    _ => {}
                }
            }
            Err(e) => panic!("{}", e.description()),
            _ => {}
        }
        rustbox.present();
    }
}

fn send_request() {
    let client = Client::new();
    let authorization = Authorization(Basic {
        username: "".to_string(),
        password: Some("".to_string()),
    });
    let mut headers = Headers::new();
    headers.set(authorization);
    let mut res = client.get("")
        .headers(headers)
        .send()
        .unwrap();
    // let mut body = String::new();
    // res.read_to_string(&mut body).unwrap();

    // println!("{}", body);
    let mut result = String::new();
    res.read_to_string(&mut result);
    println!("{:?}", result);
    // println!("{:?}", res.chars());

}
