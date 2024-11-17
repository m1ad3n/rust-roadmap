#![cfg(test)]

use ::rust_roadmap::*;

#[test]
fn fib_test() {
	assert_eq!(fib(7), 13);
	assert_eq!(fib(0), 0);
	assert_eq!(fib(1), 1);
	assert_eq!(fib(2), 1);
}

#[test]
fn factorial_test() {
	assert_eq!(factorial(5), 120);
	assert_eq!(factorial(4), 24);
	assert_eq!(factorial(2), 2);
	assert_eq!(factorial(6), 720);
}

#[test]
fn count_sentences_test() {
	assert_eq!(count_sentences("Hello, user. How are you?"), 2);
	assert_eq!(count_sentences(""), 0);
}

#[test]
fn count_words_test() {
	assert_eq!(count_words("Lorem ipsum dolor sit amet, consectetur adipiscing elit."), 8);
	assert_eq!(count_words(""), 0);
}
