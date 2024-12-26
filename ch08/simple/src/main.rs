#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let url = "http://www.rustinaction.com/";
    let response = reqwest::get(url).await?;
    let content = response.text().await?;

    println!("{}", content);

    Ok(())
}
