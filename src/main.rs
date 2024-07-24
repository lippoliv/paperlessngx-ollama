use reqwest::{Client, Error};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Document {
    id: u32,
    content: String,
}

#[derive(Deserialize, Debug)]
struct DocumentsResponse {
    results: Vec<Document>,
}

#[tokio::main]
async fn main() {
    let documents = load_documents_to_process(
        env!("PAPERLESSNGX_URL").to_string(),
        env!("PAPERLESSNGX_TOKEN").to_string(),
        env!("PAPERLESSNGX_TAGS").to_string(),
    )
        .await
        .unwrap();

    println!("{:?}", documents)
}

async fn load_documents_to_process(paperless_url: String, paperless_token: String, paperless_tags: String) -> Result<Vec<Document>, Error> {
    let request_url = format!(
        "{paperless_url}/api/documents/?tags__id__all={paperless_tags}",
    );
    let response = Client::new()
        .get(&request_url)
        .header(
            "Authorization",
            format!(
                "Token {paperless_token}",
            ),
        )
        .send()
        .await?;

    if response.status().is_success() {
        Ok(response.json::<DocumentsResponse>().await?.results)
    } else {
        println!("{:?}", response);
        panic!("Couldn't load documents")
    }
}
