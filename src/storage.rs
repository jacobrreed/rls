use std::collections::HashMap;
use main::generate_url;

pub struct Storage {
    urls: HashMap<String, String>,
    shortener: Shortener
}