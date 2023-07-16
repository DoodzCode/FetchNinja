use reqwest::{ Client, Response, RequestBuilder };
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

/// traits to be implemented on to the API structs
pub trait APIModule {
    fn new(name: String, path: String) -> Self;
    fn fetch(&self);
}

struct DadJokeAPI {
    name: String,
    path: String,
}

trait AnyModuleList {}

struct ModuleList<T: APIModule> {
    modules: Vec<T>,
}

impl<T: APIModule> AnyModuleList for ModuleList<T> {}

/// Struct responsible for data and methods pertaining to making http requests to the api server.
///
/// Contains four fields:
#[derive(Debug)]
pub struct NinjaAPI<T: APIModule> {
    /// holds the root of the url, pulled from the .env file
    base_url: String,
    /// holds the api key from apininja, pulled from the .env file
    api_key: String,
    /// vector that holds the various modules for specific api datasets
    pub modules: Vec<T>,
    /// the reqwest client for http requests/responses
    req_client: Client, // Is a reqwest client coupled to the requests it makes?
}

impl <T> NinjaAPI<T>
    where T: APIModule {
        fn new() -> NinjaAPI<T> {
            dotenv().ok();
            NinjaAPI {
                base_url: std::env
                    ::var("NINJA_ROOT_URL")
                    .expect("Environment variable NINJA_ROOT_URL not set."),
                api_key: std::env
                    ::var("NINJA_API_KEY")
                    .expect("Environment variable NINJA_API_KEY not set."),
                req_client: Client::new(),
                modules: Vec::<T>::new(),
            }
        }
}
