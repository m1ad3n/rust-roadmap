use ::rust_roadmap::*;

#[test]
fn top_word_test() {
	let content = file_content("test_file.txt");
	let most_repeated_word = top_word(content);
	assert_eq!(most_repeated_word, (String::from("sed"), 14usize));
}

#[test]
fn longest_sentence_test() {
	let content = file_content("test_file.txt");
	let most_repeated_word = longest_sentence(content);
	assert_eq!(most_repeated_word, String::from("Curabitur malesuada, tortor a pellentesque ultricies, urna odio sodales felis, et elementum sapien mi at sapien"));
}