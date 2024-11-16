
use std::collections::HashMap;
use std::fs;
use regex::Regex;

fn extract_words(content: &str) -> String {
	let re = Regex::new(r"[^\w\s]").unwrap();
	re.replace_all(content, "").to_string()
}

fn words_info(content: String) -> (String, usize) {
	let mut result: HashMap<String, usize> = HashMap::new();

	let words = extract_words(content.as_str());
	let vec_words: Vec<&str> = words.split(' ').collect();
	
	for w in vec_words {
		*result.entry(w.to_lowercase()).or_insert(0) += 1;
	}
	let top_word = result.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();
	(top_word.0.clone(), top_word.1.clone())
}

fn file_content(path: &str) -> String {
	fs::read_to_string(path.to_string()).expect("Couldn't open file for reading")
}

pub fn run() {
	let content = file_content("test_file.txt");
	let most_repeated_word = words_info(content);
	println!("{:?}", most_repeated_word);
}