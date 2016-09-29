extern crate rustbox;

use std::error::Error;
use std::default::Default;

use rustbox::{Color, RustBox};
use rustbox::Key;

fn main() {
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
