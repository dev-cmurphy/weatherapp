use warp::Filter;

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

// https://www.weatherapi.com/docs/
// weather API: http://api.weatherapi.com/v1
// dotenv().ok(); pour l'API_KEY
// let api_key = env::var("API_KEY").expect("API_KEY not set");