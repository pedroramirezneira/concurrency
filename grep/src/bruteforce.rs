pub fn bruteforce(text: &str, pattern: &str) -> bool {
    let text_chars: Vec<char> = text.chars().collect();
    let pattern_chars: Vec<char> = pattern.chars().collect();
    let n = text_chars.len();
    let m = pattern_chars.len();

    if m > n {
        return false;
    }

    for i in 0..=n - m {
        let mut j = 0;
        while j < m && text_chars[i + j] == pattern_chars[j] {
            j += 1;
        }
        if j == m {
            return true;
        }
    }
    false
}
