#![allow(unused)] // silence unused warnings while exploring (to comment out)

// 2) Constants
const DEFAULT_SPEED: i32 = 500;

fn main() {
	// 1) Variables
	let mut x = 123;
	x = 125;
	println!("value of x: {}", x);

	// 2) Constants
	const Y: i32 = 222;
	println!("value of Y: {}", Y);
	println!("value of DEFAULT_SPEED * x: {}", DEFAULT_SPEED * x);

	// 3) Shadowing
	let z = "123";
	let z: i32 = z.parse().unwrap();
	let a = z * 2;
	println!("Value of a: {}", a);
}
