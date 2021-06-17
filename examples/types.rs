#![allow(unused)] // silence unused warnings while exploring (to comment out)

fn main() {
	// 1) Integer (scalar type)
	let x: i32 = 123;
	println!("value of x: {}", x);

	// 2) Floating (scalar type)
	let y: f32 = 321.123;
	println!("value of y: {}", y);

	// Numeric operations
	let z = x as f64 / 2.0;
	let x_modulo = x % 2;
	println!("value of z: {}, x modulo 2: {}", z, x_modulo);

	// 3) Boolean (scalar type)
	let happy = true;
	let coding: bool = true;
	println!("happy coding? {}", happy && coding);

	// 4) Char (scalar type)
	let c: char = '😻';
	println!("value of c: {}", c);

	// 5) Tuple (compound type)
	let point3d_with_gravity: (i64, i64, i64, f32) = (1, 10, 0, 9.3);
	println!("value of point3d_with_gravity: {:?}", point3d_with_gravity);
	println!("value of gravity: {}", point3d_with_gravity.3);

	// 6) Array (compound type)
	let a: [f64; 6] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.2];
	println!("value of a: {:?}", a);

	let ski_months = ["Dec", "Jan", "Feb", "Mar", "Apr"];
	println!("value of ski months {:?}", ski_months);
	println!("first ski month: {}", ski_months[0]);

	let b = [12; 6];
	println!("value of b {:?}", b);
}
