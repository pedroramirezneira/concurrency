pub fn generate_route_combinations(route: &str) -> Vec<String> {
    let segments: Vec<&str> = route.trim_start_matches('/').split('/').collect();
    let n = segments.len();
    let mut results = Vec::new();

    // Generate all possible combinations using bit masking
    for mask in 0..(1 << n) {
        let mut new_route = Vec::new();
        for (i, segment) in segments.iter().enumerate() {
            if (mask & (1 << i)) != 0 {
                new_route.push(":a");
            } else {
                new_route.push(segment);
            }
        }
        results.push(format!("/{}", new_route.join("/")));
    }

    results
}
