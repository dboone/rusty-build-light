#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate iron;
extern crate serde;
extern crate serde_json;

use iron::prelude::*;
use iron::status;
use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
struct RepositoryMessage {
	name : String,
}

#[derive(Serialize, Deserialize, Debug)]
struct CommitMessage {
	sha : String,
	message : String,
	author_name : String,
}

#[derive(Serialize, Deserialize, Debug)]
struct BuildMessage {
	#[serde(rename="ref")]
	reference : String,
	build_id : u32,
	build_status : String,
	repository : RepositoryMessage,
	commit : CommitMessage,
}

fn main() {
    fn process_request(request: &mut Request) -> IronResult<Response> {
		println!("Received request!");
		let mut payload = String::new();
		request.body.read_to_string(&mut payload).unwrap();
		let build_message : BuildMessage = serde_json::from_str(&payload).unwrap();
		println!("Project {} has new commit {} on branch {} with status {}",
			build_message.repository.name,
			build_message.commit.sha,
			build_message.reference,
			build_message.build_status);
		Ok(Response::with((status::Ok, payload)))
    }

    Iron::new(process_request).http("192.168.0.105:3000").unwrap();
}