#![allow(unused)] // silence unused warnings while exploring (to comment out)

fn main() {
	println!("Hello from function.rs!");

	another_function();
	my_fun_with_args(123, 9.81);

	let y = plus_one(123);
	println!("value of y: {}", y);
}

// 1) Simple function
fn another_function() {
	println!("Hello from another_function");
}

// 2) Function with arguments
fn my_fun_with_args(x: i32, gravity: f64) {
	println!("fun with x: {}, gravity: {}", x, gravity);
}

// 3) Function with return
fn plus_one(x: i32) -> i32 {
	let value = x + 1;
	value
}
