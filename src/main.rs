extern crate curl;
extern crate rustc_serialize;

use curl::{http, ErrCode};
use curl::http::Response;
use rustc_serialize::{json, Encodable, Decodable};
use std::str::from_utf8;

static SERVER : &'static str = "gitlab/";
static API_PATH : &'static str = "api/v3";
static TOKEN : &'static str = "<private-token>";

fn get(path : &str) -> Result<Response, ErrCode> {
	let command = SERVER.to_string() + API_PATH + path;

	http::handle()
		.get(command)
		.header("PRIVATE-TOKEN", &TOKEN)
		.exec()
}

// TODO - this should use /projects and iterate
// through all of the projects searching for one
// that matches the given name

// TODO - this should have proper error handling
fn get_project_id(project : &str) -> u32 {
	#[derive(RustcEncodable, RustcDecodable)]
	struct ProjectIdResponse { id : u32, }

	let result = get("/projects/1");
	
	match result {
		Ok(response) => {
			let body = from_utf8(response.get_body());
			match body {
				Ok(b) => {
					let id_response : ProjectIdResponse = json::decode(&b).unwrap();
					return id_response.id
				},
				Err(e) => {
					println!("error: {}", e);
					return 2^31
				},
			}
		}
		Err(e) => {
			println!("error: {}", e);
			return 2^31
		},
	}
}

fn main() {
	let id0 = get_project_id("my-awesome-project");
	println!("{}", id0);
}