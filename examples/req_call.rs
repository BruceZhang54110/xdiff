use reqwest;
use tokio::time::Duration;

const UA: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Safari/537.36";

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::builder()
        // 替换成你浏览器的 User-Agent
        .user_agent(UA)
        .build()?;

    let response = client
        .get("https://jsonplaceholder.typicode.com/todos/1?a=100")
        .timeout(Duration::from_secs(10))
        .send()
        .await?;

    println!("Status: {}", response.status());
    let body = response.text().await?;
    println!("Body:\n{}", body);

    Ok(())
}