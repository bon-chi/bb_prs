extern crate rustbox;
extern crate hyper;
extern crate rustc_serialize;

use std::env;
use std::error::Error;
use std::default::Default;

use rustbox::{Color, RustBox};
use rustbox::Key;

use hyper::client::Client;
use std::io::Read;
use hyper::header::{Headers, Authorization, Basic};

use rustc_serialize::json;

const BITBUCKET_API_URL: &'static str = "BITBUCKET_API_URL";
const BITBUCKET_USERNAME: &'static str = "BITBUCKET_USERNAME";
const BITBUCKET_PASSWORD: &'static str = "BITBUCKET_PASSWORD";

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
    if let Ok(api_url) = env::var(BITBUCKET_API_URL) {
        if let Ok(user_name) = env::var(BITBUCKET_USERNAME) {
            if let Ok(password) = env::var(BITBUCKET_PASSWORD) {
                let client = Client::new();
                let authorization = Authorization(Basic {
                    username: user_name,
                    password: Some(password),
                });
                let mut headers = Headers::new();
                headers.set(authorization);
                let mut res = client.get(api_url.as_str())
                    .headers(headers)
                    .send()
                    .unwrap();
                let mut result = String::new();
                res.read_to_string(&mut result);
                println!("{:?}", result);
            }
        }
    }
}
