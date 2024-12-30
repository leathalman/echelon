use axum::{
    routing::get,
    Router
};

#[tokio::main]
async fn main() {
    // single route for application
    // let app = Router::new().route("/", get(|| async {"Hello World"}));
    let app = Router::new()
        .route("/", get(root))
        .route("/foo", get(get_foo).post(post_foo))
        .route("/foo/bar", get(foo_bar));


    // run app with hyper, listen on port 3001
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() {
    println!("get root")
}
async fn get_foo() {
    println!("get foo")
}
async fn post_foo() {
    println!("post foo")
}
async fn foo_bar() {
    println!("get foo bar")
}
