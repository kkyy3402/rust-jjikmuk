#[macro_use]
extern crate serde;
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use reqwest::Client;

#[derive(Deserialize)]
struct Item {
    userId: i32,
    id: i32,
    title: String,
    body: String,
}

async fn api_call() -> Item {
    //API 요청
    //Result를 리턴하는 구문은 try/catch 블록이 필요없다고 함. 좋네.
    let req_url = "https://jsonplaceholder.typicode.com/posts/1";
    let result = reqwest::get(req_url).await;

    let resultJsonStr = result.unwrap().text().await.unwrap();
    //와 빌드가 참 징글징글하게 안되네..
    // println!("{:?}", result.unwrap().text().await);


    //To Model
    let user: Item = serde_json::from_str(&resultJsonStr).unwrap();
    println!("id : {}", user.id);
    println!("body : {}", user.body);
    return user;
    // println!(user);
    // Ok(user)
    // return user;
}

#[tokio::main]
async fn main() {
    println!("api 요청 시작");
    let _api_result = api_call().await;
    // println!(_api_result.body);
    println!("api 요청 끝");
}