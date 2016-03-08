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

#[derive(RustcEncodable, RustcDecodable)]
struct ProjectIdResponse {
	id : u32,
	name : String,
}

#[derive(RustcEncodable, RustcDecodable, Clone)]
struct BranchResponse {
	name : String,
	commit : CommitResponse,
}

#[derive(RustcEncodable, RustcDecodable, Clone)]
struct CommitResponse {
	author_email : String,
	author_name : String,
	authored_date : String,
	committed_date : String,
	committer_email : String,
	committer_name : String,
	id : String,
	message : String,
}

#[derive(RustcEncodable, RustcDecodable)]
struct BuildStatus {
	id : String,
	status : String,
}

fn get(path : &str) -> Result<Response, ErrCode> {
	let command = SERVER.to_string() + API_PATH + path;

	http::handle()
		.get(command)
		.header("PRIVATE-TOKEN", &TOKEN)
		.exec()
}

fn get_project_id(project : &str) -> Option<u32> {
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
	
fn get_recent_branches(project_id : u32, days : i64) -> Option<Vec<BranchResponse>> {
	let path = "/projects/".to_string() + &project_id.to_string() + "/repository/branches";
	let result = get(&path);

	match result {
		Ok(response) => {
			let body = from_utf8(response.get_body());
			match body {
				Ok(b) => {
					let responses : Vec<BranchResponse> = json::decode(&b).unwrap();
					let mut results : Vec<BranchResponse>  = Vec::new();
					for response in responses.iter() {
						let committed_date = DateTime::parse_from_str(
							&response.commit.committed_date, "%Y-%m-%dT%H:%M:%S%.3f%z").unwrap();

						let difference = UTC::now() - committed_date;
						if difference.num_days() < days {
							results.push(response.clone());
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

fn get_commit_build_status(project_id : u32, commit_id : String) -> Option<BuildStatus> {
	let path = "/projects/".to_string() + &project_id.to_string() + "/repository/commits/" + &commit_id;
	let result = get(&path);

	match result {
		Ok(response) => {
			let body = from_utf8(response.get_body());
			match body {
				Ok(b) => {
					let response : BuildStatus = json::decode(&b).unwrap();
					Some(response)
				},
				Err(e) => { println!("error: {}", e); None },
			}
		}
		Err(e) => {	println!("error: {}", e); None },
	}
}

fn get_build_status_recent_branches(project_name : String, days : i64) {
	let project_id = get_project_id(&project_name).unwrap();
	let branches = get_recent_branches(project_id, days).unwrap();
	for branch in branches {
		let commit_status = get_commit_build_status(project_id, branch.commit.id).unwrap().status;
		println!("Branch {}: {}", branch.name, commit_status);
	}
}

fn main() {
	let project = "my-awesome-project".to_string();
	get_build_status_recent_branches(project, 10);	
}