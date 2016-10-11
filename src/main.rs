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
    description: String,
    state: String,
    url: String,
    display_id: String,
}

fn main() {
    let pull_requests = get_pull_requests();
    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    rustbox.print(0,
                  0,
                  rustbox::RB_BOLD,
                  Color::White,
                  Color::Black,
                  "bb_prs > type your ticket name");

    let mut query = String::new();
    // let mut query = String::from("bb_prs > ");
    let mut pull_requests_num = 1;
    for pull_request in &pull_requests {
        rustbox.print(0,
                      pull_requests_num,
                      rustbox::RB_BOLD,
                      Color::White,
                      Color::Black,
                      pull_request.title.as_str());
        pull_requests_num += 1;
    }
    let mut matched_pull_requests = get_matched_pull_requests(pull_requests, query.to_string());

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
                                      "bb_prs > ");
                        rustbox.print(9,
                                      0,
                                      rustbox::RB_NORMAL,
                                      Color::Default,
                                      Color::Default,
                                      &query);
                        pull_requests_num = 1;
                        matched_pull_requests = get_matched_pull_requests(matched_pull_requests,
                                                                          query.to_string());
                        for pull_request in &matched_pull_requests {
                            rustbox.print(0,
                                          pull_requests_num,
                                          rustbox::RB_BOLD,
                                          Color::White,
                                          Color::Black,
                                          pull_request.title.as_str());
                            pull_requests_num += 1;
                        }
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

fn get_matched_pull_requests(pull_requests: Vec<PullRequest>, query: String) -> Vec<PullRequest> {
    pull_requests.into_iter()
        .filter(|pull_request| pull_request.title.contains(query.as_str()))
        .collect::<Vec<PullRequest>>()
}

fn get_pull_requests() -> Vec<PullRequest> {
    let result = send_request();
    let response = json::Json::from_str(&(result.unwrap())).unwrap();
    let values = response.find("values").unwrap();
    match values {
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
                        // description: request.find("description").unwrap().to_string(),
                        description: match request.find("description") {
                            None => "none".to_string(),
                            Some(des) => des.to_string(),
                        },
                        // description: "null".to_string(),
                        display_id: request.find("fromRef")
                            .unwrap()
                            .find("displayId")
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
                description: "none".to_string(),
                display_id: "none".to_string(),
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
                description: "none".to_string(),
                display_id: "none".to_string(),
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
                description: "none".to_string(),
                display_id: "none".to_string(),
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
                description: "none".to_string(),
                display_id: "none".to_string(),
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
                description: "none".to_string(),
                display_id: "none".to_string(),
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
                description: "none".to_string(),
                display_id: "none".to_string(),
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
                description: "none".to_string(),
                display_id: "none".to_string(),
            };
            vec.push(pr);
            vec
        }
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
