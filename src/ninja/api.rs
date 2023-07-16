use reqwest::{Client, Response, RequestBuilder};
use dotenv::dotenv;


#[derive(Debug)]
struct APIParam {
    name: String,
    value: String,
}
impl APIParam {
    fn stringify(&self) -> String {
        format!("{}{}", self.name, self.value)
    }
}

/// struct for holding methods and data coupled to a single api.
#[derive(Debug)]
pub struct DadJokeAPI {
    name: &'static str,
}

/// traits to be implemented on to the API structs
trait APIModule {
    fn new(name: &'static str) -> Self;
    fn name(&self) -> String {
        &self.name
    }
    fn path(&self) -> String {
        &self.path
    }
}

/// Struct responsible for data and methods pertaining to making http requests to the api server.
///
/// Contains four fields:
#[derive(Debug)]
pub struct NinjaAPI<T> {
    /// holds the root of the url, pulled from the .env file
    base_url: String,
    /// holds the api key from apininja, pulled from the .env file
    api_key: String,
    /// vector that holds the various modules for specific api datasets
    pub modules: Vec<APIModule<T>>::new(),
    /// the reqwest client for http requests/responses
    req_client: Client, // Is a reqwest client coupled to the requests it makes?
}
impl NinjaAPI {
    /// returns a new NinjaAPI
    fn new() -> NinjaAPI {
        let thing: Vec<T> = vec![32, 16, 45];
        dotenv().ok();
        NinjaAPI {
            base_url: std::env::var("NINJA_ROOT_URL").expect("Environment variable NINJA_ROOT_URL not set."),
            api_key: std::env::var("NINJA_API_KEY").expect("Environment variable NINJA_API_KEY not set."),
            modules: Vec::new(),
            req_client: Client::new(),
        }
    }
}