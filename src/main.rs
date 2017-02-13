//! Search and Destroy
//!
//! A command line tool for finding and replacing in text.
#![recursion_limit = "1024"]

extern crate regex;
extern crate walkdir;
extern crate clap;

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate maplit;

mod cmd;
mod load;
mod replace;
mod types;

use types::*;


fn do_it(cmd: &Cmd) -> Result<()> {
	let files = load::load_paths(&cmd.paths)?;
	let replaced = files.iter().map(|f| replace::replace_file(cmd, f));

	for r in replaced {
	    println!("dumping at: {}", r.path.display());
        r.dump()?;
	}
	Ok(())
}

fn main() {
	let app = cmd::get_app();
	let matches = app.get_matches();
	let cmd = cmd::get_cmd(&matches).unwrap();
	println!("{:?}", cmd);
	do_it(&cmd).expect("can't do it");
}
