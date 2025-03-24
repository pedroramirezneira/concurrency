pub(crate) fn bruteforce(text: &str, pattern: &str) -> bool {
    let n = text.len();
    let m = pattern.len();
    for i in 0..n - m + 1 {
        let mut j = 0;
        while j < m && text.chars().nth(i + j).unwrap() == pattern.chars().nth(j).unwrap() {
            j += 1;
        }
        if j == m {
            return true;
        }
    }
    false
}