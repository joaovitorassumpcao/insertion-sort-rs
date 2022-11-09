use std::env::args;
use insertion_sort_rs::*;

fn main(){
	let args: Vec<String> = std::env::args().collect();
	let mut a: Vec<i32> = Vec::new();

	if args.len() > 2 {
		for i in 0..args.len() {
			if let Ok(parsed) = args[i].parse::<i32>(){
				a.push(parsed);
			} else {
				panic!("Invalid argument! Integers are required.");
			}
		}
	} else {
		panic!("Not enough arguments! Must be bigger than 2.")
	}
}