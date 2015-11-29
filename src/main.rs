use std::env;

mod nodes;

fn main() {
	let exit_code = start();
	std::process::exit(exit_code);
}

fn start() -> i32{
	
	let args: Vec<_> = env::args().collect();

	if args.len() != 2{
		return usage_error();
	}

	match args[1].parse::<i32>() {
		Ok(n) => println!("OK {}", n),
		Err(_) => return usage_error(),
		
	}

	let node = nodes::register_node();

	return 0;
}

fn usage_error() -> i32{
	println!("Usage: int - process number");
	return 1;
}
