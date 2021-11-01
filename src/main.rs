use warp::Filter;

#[tokio::main]
async fn main() {
    let hello = warp::path!("Hello" / String)
        .map(|name| format!("Bruh, {}", name));

    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
