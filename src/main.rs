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

fn get_project_id(project : &str) -> Option<u32> {
	#[derive(RustcEncodable, RustcDecodable)]
	struct ProjectIdResponse {
		id : u32,
		name : String,
	}

	let result = get("/projects");

	match result {
		Ok(response) => {
			let body = from_utf8(response.get_body());
			match body {
				Ok(b) => {
					let responses : Vec<ProjectIdResponse> = json::decode(&b).unwrap();

					for response in responses.iter() {
						if response.name == project {
							return Some(response.id)
						}
					}
					None
				},
				Err(e) => { println!("error: {}", e); None },
			}
		}
		Err(e) => {	println!("error: {}", e); None },
	}
}

fn main() {
	let id0 = get_project_id("my-awesome-project");
	println!("{}", id0.unwrap());

	let id1 = get_project_id("my-lame-project");
	println!("{}", id1.unwrap());
}