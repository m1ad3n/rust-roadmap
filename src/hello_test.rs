#![cfg(test)]

use crate::fib;
use crate::factorial;

#[test]
fn fib_test() {
	assert_eq!(13, fib(7));
	assert_eq!(0, fib(0));
	assert_eq!(1, fib(1));
	assert_eq!(1, fib(2));
}

#[test]
fn factorial_test() {
	assert_eq!(factorial(5), 120);
	assert_eq!(factorial(4), 24);
	assert_eq!(factorial(2), 2);
	assert_eq!(factorial(6), 720);
}
