extern crate curl;

use std::env;
use std::str::from_utf8;
use curl::http;

fn main() {
	let token = match env::args().nth(1) {
		Some(t) => t,
		None => {
			println!("expected private token");
			return
		},
	};

	let resp = http::handle()
		.get("gitlab/api/v3/projects/1")
		.header("PRIVATE-TOKEN", &token)
		.exec();
	
	match resp {
		Ok(r) => {
			println!("code: {}", r.get_code());

			let body = from_utf8(r.get_body());
			match body {
				Ok(b) => println!("body:\n{}", b),
				Err(e) => {
					println!("error: {}", e);
					return
				},
			}
		}
		Err(e) => {
			println!("error: {}", e);
			return
		},
	}
}