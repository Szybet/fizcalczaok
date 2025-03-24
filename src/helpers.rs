

pub fn split_strings_by_list(input: &str, delimiters: Vec<String>) -> (Vec<String>, Vec<String>) {
    let mut substrings = Vec::new();
    let mut used_delimiters = Vec::new();
    let mut current = String::new();
    let mut i = 0;

    while i < input.len() {
        let mut matched = false;
        for delim in &delimiters {
            if input[i..].starts_with(delim) {
                if !current.is_empty() {
                    substrings.push(current.clone());
                    current.clear();
                }
                used_delimiters.push(delim.clone());
                i += delim.len();
                matched = true;
                break;
            }
        }
        if !matched {
            current.push(input.chars().nth(i).unwrap());
            i += 1;
        }
    }
    if !current.is_empty() {
        substrings.push(current);
    }
    
    (substrings, used_delimiters)
}