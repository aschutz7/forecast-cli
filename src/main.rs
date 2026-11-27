mod weather;

#[tokio::main]
async fn main() {
    println!("{:#?}", weather::test_form_req_url().await.unwrap());
}
