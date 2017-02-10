//! Search and Destroy
//!
//! A command line tool for finding and replacing in text.
#![recursion_limit = "1024"]

extern crate regex;
extern crate clap;

#[macro_use]
extern crate error_chain;

use clap as cl;

mod types;
mod cmd;



fn main() {
	let app = cmd::get_app();
	let matches = app.get_matches();
	let cmd = cmd::get_cmd(&matches).unwrap();
	println!("{:?}", cmd);
}
