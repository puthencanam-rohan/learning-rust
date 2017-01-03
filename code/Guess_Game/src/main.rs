
extern crate rand;

use std::io;
use std::cmp::Ordering;
use  rand::Rng;

fn main() {
	let rand_num = rand::thread_rng().gen_range(1, 101);

	println!("The secret number is : {}", rand_num);
	loop {
		println!("Guess a number");
        	let mut num = String::new();
		//let mut str = String::new();	
		io::stdin().read_line(&mut num)
			   .expect("Failed");
		//io::stdin().read_line(&mut str)
		//	   .expect("Failes");
	let num: u32 =match num.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};

		println!("You  	guessed : {}",num);
		match num.cmp(&rand_num) {
			Ordering::Less		=>	println!("Too Small"),
			Ordering::Greater	=>	println!("Too Big"),
			Ordering::Equal		=>	{	
							println!("You Win");
							break;
						}
		}
	
	/*	match str.cmp(&num){
		 	Ordering::Less          =>      println!("Too Small"),
                	Ordering::Greater       =>      println!("Too Big"),
                	Ordering::Equal         =>      println!("You Win"),

		}*/
	}
}
