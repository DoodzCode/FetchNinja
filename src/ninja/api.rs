use reqwest::{ Client, Response, RequestBuilder };
use dotenv::dotenv;

/// Holds a url parameter
#[derive(Debug)]
struct ApiParam {
    name: String,
    value:String,
}

impl ApiParam {
    /// Returns a new ApiParam from a &str
    fn new(name: &String) -> ApiParam {
        ApiParam {
            name: name.to_owned(),
            value: "".to_string(),
        }
    }

    /// Takes a new &str and updates the value of itself
    fn update_value(&mut self, new_value: &str) {
        self.value = new_value.to_string();
    }

    /// Returns a formatted String for use within a request url
    fn to_string(&self) -> String {
        format!("{}={}", self.name, self.value)
    }
}

#[derive(Debug)]
pub struct ApiConfig {
    pub name: String,
    pub path: String,
    pub parameter_names: Vec<String>,
}
impl ApiConfig {
    fn new(name: String, path: String, parameter_names: Vec<String>) -> ApiConfig {
        ApiConfig {
            name: name,
            path: path,
            parameter_names: parameter_names.to_vec(),
        }
    }
}

#[derive(Debug)]
struct Api {
    config_obj: ApiConfig,
    params: Vec<ApiParam>,
}

impl Api {
    pub fn new(config: ApiConfig) -> Api {
        let mut params: Vec<ApiParam> = Vec::new();
        for p in &config.parameter_names {
            params.push(ApiParam::new(&p));
        }
        Api {
            config_obj: config,
            params: params,
        }
    }

    pub fn name(&self) -> &String {
        &self.config_obj.name
    }

    pub fn path(&self) -> &String {
        &self.config_obj.path
    }

    pub fn list_params(&self) -> &Vec<String> {
        &self.config_obj.parameter_names
    }

    pub fn get_api_params(&self) -> &Vec<ApiParam> {
        &self.params
    }

}

/// Struct responsible for data and methods pertaining to making http requests to the api server.
///
/// Contains four fields:
#[derive(Debug)]
pub struct NinjaAPI {
    /// holds the root of the url, pulled from the .env file
    base_url: String,
    /// holds the api key from apininja, pulled from the .env file
    api_key: String,
    /// the reqwest client for http requests/responses
    req_client: Client, // Is a reqwest client coupled to the requests it makes?
    modules: Vec<Api>,
}

impl NinjaAPI {
    fn new(configs: Vec<ApiConfig>) -> NinjaAPI {
        dotenv().ok();
        NinjaAPI {
            base_url: std::env
                ::var("NINJA_ROOT_URL")
                .expect("Environment variable NINJA_ROOT_URL not set."),
            api_key: std::env
                ::var("NINJA_API_KEY")
                .expect("Environment variable NINJA_API_KEY not set."),
            req_client: Client::new(),
            modules: NinjaAPI::build_modules(configs),
        }
    }

    fn build_modules(configs: Vec<ApiConfig>) -> Vec<Api> {
        let mut modules: Vec<Api> = Vec::new();
        for config in configs {
            modules.push(Api::new(config));
        }
        modules
    }



    /// finds an Api matching a given name and returns a reference to that Api
    fn find_api(&self, name: String) -> &Api {
        self.modules.iter().find(|&x| x.name() == &name).unwrap()
    }

    // fn build_params_for(&self, api_name: String) -> Vec<ApiParam> {
    //     let api = self.api(api_name);
    //     let param_names = api.list_params();
    //     let mut api_params: Vec<ApiParam> = Vec::new();
    //     for param in param_names.into_iter().map(|p|(p)) {
    //         api_params.push(ApiParam::new(param));
    //     };
    //     api_params
    // }

    fn fetch(what: Vec<ApiParam>, from: (String, String)) {
        // get root_url, api_ley, path, params as String
    }
}

pub fn generate_api_configurations() -> Vec<ApiConfig> {
    vec![
        ApiConfig::new(
            "Dad Jokes".to_string(),
            "dadjokes".to_string(),
            vec!["limit".to_string()]
        ),
        ApiConfig::new(
            "Historical Events".to_string(),
            "historicalevens".to_string(),
            vec!["text".to_string(), "year".to_string(), "month".to_string(), "day".to_string()]
        ),
        ApiConfig::new(
            "Chuck Norris".to_string(),
            "chucknorris".to_string(),
            Vec::new() // no parameters needed
        ),
        ApiConfig::new(
            "Password Generator".to_string(),
            "password".to_string(),
            vec!["length".to_string()]
        )
    ]
}

pub fn test_room() {
    let configs: Vec<ApiConfig> = generate_api_configurations();
    let mut neeenja: NinjaAPI = NinjaAPI::new(configs);

    println!("{:?}", neeenja.find_api("Dad Jokes".to_string()).name());
}
