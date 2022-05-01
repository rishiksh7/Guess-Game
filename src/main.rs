use rand ::{thread_rng,Rng};
use std::io;
use std::cmp::Ordering;
fn main(){
	println!("WELCOME TO GUESS GAME");
	let secret = rand::thread_rng().gen_range(1..101);
	println!("{}",secret);
	let mut guess= String::new();
	println!("Guess the Number:");
	io::stdin().read_line(&mut guess)
		.expect("failed to read the line.");
	println!("Your Guessed: {}",guess);
	let guess:u32=guess.trim().parse().expect("Enter an integer");
	match guess.cmp(&secret){
		Ordering::Less =>println!("Too Short"),
		Ordering::Greater =>println!("Too BIG"),
		Ordering::Equal =>{
			println!("You Win");
		},
	}
}

