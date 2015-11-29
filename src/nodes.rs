use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::fs::OpenOptions;
use std::path::Path;

pub struct NodeAddress{
	ip: String,
	port: i32,
}

pub fn register_node() -> NodeAddress{
	
	let path = Path::new("nodes.txt");

    let mut file_in = match OpenOptions::new().
    	read(true).open(&path) {
        Err(_) => return NodeAddress{ ip: String::from(""), port: -1 },
        Ok(file) => file,
    };

	let file_reader = BufReader::new(file_in);

	let mut file_out = match OpenOptions::new().
    	write(true).append(true).open(&path) {
        Err(_) => return NodeAddress{ ip: String::from(""), port: -1 },
        Ok(file) => file,
    };

	let mut readLine = String::new();
	let mut index = 0;

	for line in file_reader.lines(){
		readLine = line.unwrap();
		index += 1;
	}

	if index == 0{
		let node = NodeAddress{ ip: String::from("127.0.0.1"), port: 6000 };

		let text = String::new() + &node.ip + " " + &node.port.to_string() + "\n";

		match file_out.write_all(text.as_bytes()) {
       	 	Err(_) => return NodeAddress{ ip: String::from(""), port: -1 },
        	Ok(_) => println!("successfully wrote to"),
    	}

		return node
	}else{

		let vec = readLine.split(" ").collect::<Vec<&str>>();

		let port = match vec[1].parse::<i32>() {
			Ok(n) => n + 1,
			Err(_) => -1,
		};

		let node = NodeAddress{ ip: String::from(vec[0]), port: port };

		let text = String::new() + &node.ip + " " + &node.port.to_string() + "\n";

		match file_out.write_all(text.as_bytes()) {
       	 	Err(_) => return NodeAddress{ ip: String::from(""), port: -1 },
        	Ok(_) => println!("successfully wrote to"),
    	}

		return node
	}
	
}