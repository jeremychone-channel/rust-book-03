#![allow(unused)] // silence unused warnings while exploring (to comment out)

fn main() {
	let number = 1;

	// 1) if/else
	let message = if number < 5 {
		"pretty small"
	} else if number < 100 {
		"pretty normal"
	} else {
		"pretty big"
	};
	println!("{}", message);

	// 2) loop
	let mut counter = 0;
	loop {
		println!("Again loop {}", counter);
		counter += 1;
		if counter > 2 {
			break;
		}
	}

	// 3) while
	let a = [0, 10, 20, 30, 40];
	counter = 0;
	while counter < 3 {
		println!("Again while {}", a[counter]);
		counter += 1;
	}

	// 4) for (in iter())
	for element in a.iter() {
		println!("Again for in {}", element);
		if element == &20 {
			break;
		}
	}

	// 5) for (in range)
	for number in (0..3) {
		println!("Again for range {}", a[number]);
	}
}
