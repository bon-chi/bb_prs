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

#[derive(Debug, RustcDecodable, RustcEncodable)]
struct BBResponse {
    size: i32,
    limit: i32,
    is_last_page: bool,
    values: Vec<PullRequest>,
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
struct PullRequest {
    title: String,
    state: String,
    url: String,
}

fn main() {
    let result = send_request();
    // let decoded_response = json::decode(&(result.unwrap())).unwrap();
    let response = json::Json::from_str(&(result.unwrap())).unwrap();
    println!("{}",
             response.find("values").unwrap()[0].find("title").unwrap());
    // println!("{:?}", response.find("values").unwrap());
    // let values: json::Array = response.find("values").unwrap();
    let values = response.find("values").unwrap();
    let pull_requests = match values {
        &json::Json::Array(ref pull_request) => {
            pull_request.into_iter()
                .map(|request| {
                    PullRequest {
                        title: request.find("title").unwrap().to_string(),
                        state: request.find("state").unwrap().to_string(),
                        url: request.find("links").unwrap().find("self").unwrap()[0]
                            .find("href")
                            .unwrap()
                            .to_string(),
                    }
                })
                .collect::<Vec<PullRequest>>()
        }
        &json::Json::I64(_) => {
            let mut vec = Vec::new();
            let pr = PullRequest {
                title: "none".to_string(),
                state: "none".to_string(),
                url: "none".to_string(),
            };
            vec.push(pr);
            vec
        }
        &json::Json::U64(_) => {
            let mut vec = Vec::new();
            let pr = PullRequest {
                title: "none".to_string(),
                state: "none".to_string(),
                url: "none".to_string(),
            };
            vec.push(pr);
            vec
        }
        &json::Json::F64(_) => {
            let mut vec = Vec::new();
            let pr = PullRequest {
                title: "none".to_string(),
                state: "none".to_string(),
                url: "none".to_string(),
            };
            vec.push(pr);
            vec
        }
        &json::Json::String(_) => {
            let mut vec = Vec::new();
            let pr = PullRequest {
                title: "none".to_string(),
                state: "none".to_string(),
                url: "none".to_string(),
            };
            vec.push(pr);
            vec
        }
        &json::Json::Boolean(_) => {
            let mut vec = Vec::new();
            let pr = PullRequest {
                title: "none".to_string(),
                state: "none".to_string(),
                url: "none".to_string(),
            };
            vec.push(pr);
            vec
        }
        &json::Json::Object(_) => {
            let mut vec = Vec::new();
            let pr = PullRequest {
                title: "none".to_string(),
                state: "none".to_string(),
                url: "none".to_string(),
            };
            vec.push(pr);
            vec
        }
        Null => {
            let mut vec = Vec::new();
            let pr = PullRequest {
                title: "none".to_string(),
                state: "none".to_string(),
                url: "none".to_string(),
            };
            vec.push(pr);
            vec
        }
    };
    println!("{:?}", pull_requests);
    // let pull_requests = response.find("values").unwrap().into_iter().map(|pull_request| {
    //     PullRequest {
    //         title: pull_request.find("title").unwrap(),
    //         state: pull_request.find("state").unwrap(),
    //         url: pull_request.find("url").unwrap(),
    //     }
    // });
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

fn send_request() -> Result<String, &'static str> {
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
                // println!("{:?}", result);
                return Ok(result);
            }
        }
    }
    return Err("failed to get prs");
}
