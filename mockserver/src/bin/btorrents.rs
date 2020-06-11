use rweb::*;
use serde::Serialize;
use mockserver::yts::mocks::mockYtsMovies;
use mockserver::yts::data::YtsResponse;

#[get("/")]
fn index() -> String {
    String::from("content type will be 'text/plain' as you return String")
}

#[get("/ws/yts/torrents/{page}")]
fn movies(page: String) -> Json<YtsResponse> {
    Json::from(mockYtsMovies())
}

#[tokio::main]
async fn main() {
    let (_spec, filter) = openapi::spec().build(|| index().or(movies()));

    //serve(filter);
    // Use the code below to run server.
    //
    serve(filter).run(([127, 0, 0, 1], 3030)).await;
}
