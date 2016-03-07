extern crate chrono;
extern crate curl;
extern crate rustc_serialize;

use chrono::{DateTime, UTC};
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

fn get_recent_branches(project_id : u32, days : i64) -> Option<Vec<String>> {
	#[derive(RustcEncodable, RustcDecodable)]
	struct BranchResponse {
		name : String,
		commit : CommitResponse,
	}

	#[derive(RustcEncodable, RustcDecodable)]
	struct CommitResponse {
		committed_date : String,
	}

	let path = "/projects/".to_string() + &project_id.to_string() + "/repository/branches";
	let result = get(&path);

	match result {
		Ok(response) => {
			let body = from_utf8(response.get_body());
			match body {
				Ok(b) => {
					let responses : Vec<BranchResponse> = json::decode(&b).unwrap();
					let mut results : Vec<String>  = Vec::new();
					for response in responses.iter() {
						let committed_date = DateTime::parse_from_str(&response.commit.committed_date, "%Y-%m-%dT%H:%M:%S%.3f%z").unwrap();
						let difference = UTC::now() - committed_date;
						
						if difference.num_days() < days {
							results.push(response.name.clone());
						}
					}
					Some(results)
				},
				Err(e) => { println!("error: {}", e); None },
			}
		}
		Err(e) => {	println!("error: {}", e); None },
	}
}

fn main() {
	let project = "my-awesome-project";

	println!("details for {}", project);

	let id = get_project_id(project).unwrap();
	println!("project id: {}", id);

	print!("recent branches: ");
	let branches = get_recent_branches(id, 10).unwrap();
	for branch in branches {
		print!("{}, ", branch);
	}
}