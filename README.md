# Fetch Ninja!
<hr>

![project overview](/project_overview.png)

## Dependencies
- reqwest - http requests
- serde - serialize and deserialize data
- tokio - async runtime



## Data
The core data needed to connect to authenticate should reside in the NinjaAPI struct.



```rust

struct NinjaAPI
{
    base_url: String,
    api_key: String,
    modules: <APIModule>,
    req_client: reqwest::Client,
}

struct APIModule
{
    name: String,
    path: String,
    params: <APIParam>
}

struct APIParam
{
    name: String,
    value: String,
}


let param: APIParam
{
    name: "text=".to_string(),
    values: "Estonia".to_string(),
}

let api_module: APIModule
{
    name: "Historical Event",
    path: "/historicalevents", // from config
    params: vec![param],
}

let ninja_api: NinjaAPI
{
    base_url: "https://ninjaapi.com" // will be pulled from a config file
    api_key: "ffsdefffdcf34245tgdbdb434rf=4fvvxcx" // will be pulled from .env file
    modules: vec![api_module]
}





```