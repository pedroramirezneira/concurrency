pub(crate) fn leibniz(n: u64) -> f64 {
    let mut pi = 0.0;
    let mut sign = 1.0;
    for i in 0..n+1 {
        pi = pi + sign / (2 * i + 1) as f64;
        sign = -sign
    }
    pi * 4.0
}