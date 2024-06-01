pub mod utils {
    use regex::Regex;

    const URL_REGEX: &str = r"http(s*)://(.+)/(.*)$";

    pub async fn get_file_url(url: &str) -> Result<&str, Box<dyn std::error::Error + Send + Sync>> {
        let re = Regex::new(URL_REGEX).unwrap();
        let captures = re.captures(url).unwrap();
        let filename = captures.get(3).unwrap().as_str();
        Ok(filename)
    }
}
