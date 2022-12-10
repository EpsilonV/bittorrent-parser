pub fn encode_url_query(params: &[(&str, &str)]) -> String {
    let param_strings = params
        .iter()
        .map(|&(k, v)| format!("{}={}", k, v))
        .collect();
    param_strings.join("&")
}
