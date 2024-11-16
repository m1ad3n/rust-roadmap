#![allow(dead_code)]

mod hello_test;

use std::io::{stdin, stdout, Write};
use cached::proc_macro::cached;

#[derive(Debug)]
struct Person {
	name: String,
	lastname: String,
	age: u8
}

impl Person {
	fn create(name_a: &str, lastname_a: &str) -> Self {
		Person {
			name: name_a.to_string(),
			lastname: lastname_a.to_string(),
			age: 0
		}
	}
}

#[cached]
fn fib(n: u32) -> u32 {
	if n < 1 { return 0 }
	else if n <= 2 { return 1 }
	fib(n - 1) + fib(n - 2)
}

#[cached]
fn factorial(n: u32) -> u32 {
	if n < 2 {
		return 1
	}
	return n * factorial(n - 1)
}

fn count_sentences(text: &str) -> u32 {
	let mut count: u32 = 0;
	for c in text.chars() {
		if c == '.' || c == '?' || c == '!' {
			count += 1;
		}
	}
	count
}

fn input_names() -> Vec<Person> {
	let mut buffer = String::new();
	let mut result: Vec<Person> = vec![];

	loop {
		buffer.clear();

		print!("fullname >> ");
		stdout().flush().unwrap();
		stdin().read_line(&mut buffer).expect("stdin error");

		if buffer.len() < 2 {
			break;
		}

		let buvec: Vec<&str> = buffer.trim().split(' ').collect();
		if buvec.len() < 2 {
			println!("please enter your first and last name");
			continue;
		}

		result.push(Person::create(buvec[0], buvec[1]));
	}

	result
}

fn count_words(text: &str) -> u32 {
	// false - in blank space
	// true - in word
	let mut status: bool = false;
	let mut count: u32 = 0;

	for ch in text.chars() {
		if (ch == ' ' || ch == '\n') && status {
			status = false;
		}
		else if !status {
			count += 1;
			status = true;
		}
	}

	count
}

fn say_hello(name: &str) {
	println!("Hello, {name}");
}

fn main() {
	say_hello("Mladen");
	
	let fb = fib(5);
	println!("fib of 7 = {fb}");

	let counts = count_sentences("Hey, there! What is your name?");
	println!("sentences = {counts}");

	let countw = count_words("Hello, how are you my friend.\nDidn't see you for a while.");
	println!("words = {countw}");

	let map = input_names();
	println!("{:#?}", map);
}