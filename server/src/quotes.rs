use once_cell::sync::Lazy;
use rand::{thread_rng, Rng};
use serde_json::{Error, Value};

static QUOTES: Lazy<Value> = Lazy::new(|| load_quotes().expect("Illegal JSON format"));

pub fn get_random_quote() -> Value {
    let quotes_count: usize = QUOTES.as_array().unwrap().len();
    let selected_index: usize = thread_rng().gen_range(0..quotes_count);
    QUOTES[selected_index].clone()
}

fn load_quotes() -> Result<Value, Error> {
    let text: &str = include_str!("../resources/quotes.json");
    serde_json::from_str::<Value>(text)
}
