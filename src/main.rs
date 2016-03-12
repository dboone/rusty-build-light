extern crate chrono;
extern crate curl;
extern crate rustc_serialize;
extern crate serial;

use chrono::{DateTime, UTC};
use curl::{http, ErrCode};
use curl::http::Response;
use rustc_serialize::{json, Encodable, Decodable};
use serial::SerialPort;
use std::io;
use std::str::from_utf8;
use std::thread;
use std::time::Duration;

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

fn get_build_status_recent_branches(project_name : String, days : i64) -> Vec<String> {
	let project_id = get_project_id(&project_name).unwrap();
	let branches = get_recent_branches(project_id, days).unwrap();
	let mut results : Vec<String>  = Vec::new();
	for branch in branches {
		let commit_status = get_commit_build_status(project_id, branch.commit.id).unwrap().status;
		let commit_short : u8 = match commit_status.as_ref() {
			"success" => { 1 }
			"failed" => { 0 }
			_ => { 0 }
		};
		results.push(format!("{}{};", branch.name, commit_short));
	}
	results
}

fn interact<T : SerialPort>(port : &mut T, project : String, messages : Vec<String>) -> io::Result<()> {
	let mut project_buff : Vec<u8> = project.bytes().collect();
	project_buff.push(b';');
	try!(port.write(&project_buff[..]));
	
	for message in messages {
		let mut buf : Vec<u8> = message.bytes().collect();
		try!(port.write(&buf[..]));
		println!("Sending: {}", message);
		//thread::sleep(Duration::new(1, 0));
	}

    Ok(())
}

fn waitForContact<T : SerialPort>(port : &mut T) {
		let mut buffer = String::new();
		port.read_to_string(&mut buffer);
		println!("Received: {}", buffer);
}

fn initializePort<T : SerialPort>(port : &mut T) {
	port.reconfigure(&|settings| {
		try!(settings.set_baud_rate(serial::Baud9600));
		settings.set_char_size(serial::Bits8);
		settings.set_parity(serial::ParityNone);
		settings.set_stop_bits(serial::Stop1);
		settings.set_flow_control(serial::FlowNone);
		Ok(())
    });

    port.set_timeout(Duration::from_millis(1000));
}

fn main() {
	//let project = "my-awesome-project".to_string();
	let mut port = serial::open("COM3").unwrap();
	println!("Opened");
	initializePort(&mut port);
	println!("Initialized");
	println!("Waiting for contact...");
	waitForContact(&mut port);
	println!("Contact established!");

	while true {
		let project = "my-awesome-project".to_string();
		let messages = get_build_status_recent_branches(project.clone(), 10);

		interact(&mut port, project.clone(), messages).unwrap();
		thread::sleep(Duration::new(10, 0));
	}
}