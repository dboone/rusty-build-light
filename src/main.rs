#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate iron;
extern crate serde;
extern crate serde_json;
extern crate serial;

use iron::prelude::*;
use iron::status;
use serial::SerialPort;
use std::io;
use std::io::Read;
use std::thread;
use std::time::Duration;

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

fn write_to_port<T : SerialPort>(port : &mut T, message : String) -> io::Result<()> {
	let mut buffer : Vec<u8> = message.bytes().collect();

	try!(port.write(&buffer[..]));
	println!("Wrote: {}", message);

	Ok(())
}

fn initialize_port<T : SerialPort>(port : &mut T) {
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
    fn process_build_request(request: &mut Request) -> IronResult<Response> {
		println!("Received request!");
		let mut payload = String::new();
		request.body.read_to_string(&mut payload).unwrap();
		println!("Received {}", payload);
		let build_message : BuildMessage = serde_json::from_str(&payload).unwrap();
		let message = format!("Project\n  {}\nCommit\n  {}\nBranch\n  {}\nAuthor\n  {}\nMessage\n  {}\nStatus\n  {};",
			build_message.repository.name,
			&build_message.commit.sha[..7],
			build_message.reference,
			build_message.commit.author_name,
			build_message.commit.message,
			build_message.build_status);

		let mut port = serial::open("COM3").unwrap();
		initialize_port(&mut port);
		write_to_port(&mut port, message);

		Ok(Response::with((status::Ok, payload)))
    }

    Iron::new(process_build_request).http("192.168.0.105:3000").unwrap();
}