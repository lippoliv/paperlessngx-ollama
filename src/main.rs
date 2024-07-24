use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::generation::parameters::FormatType;
use ollama_rs::Ollama;
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

    generate_documents_summary_via_ollama(documents).await;
}

async fn generate_documents_summary_via_ollama(documents: Vec<Document>) {
    for document in documents.iter() {
        generate_document_summary_via_ollama(document).await;
    }
}

async fn generate_document_summary_via_ollama(document: &Document) {
    let ollama = Ollama::new(env!("OLLAMA_HOST"), env!("OLLAMA_PORT").parse::<u16>().unwrap());
    let model = "llama3:latest";
    let prompt = format!(
        "summarize the following text within 150 characters.\
        this summary will be used in a document management system to later find the latter.\
        focus on the start of the text.\
        ignore address information.\
        \
        the summary must be one sentence.\
        just write the summary.\
        \
        if you find an invoice number, mention it.\
        if it's a reminder for an invoice, mention it.\
        do not write the sender of the letter.\
        \
        format your answer as json where the summary is in the field 'summary'\
        the summary must be in {lang} language \n\n",
        lang = env!("OLLAMA_LANGUAGE")
    );

    let limit = if document.content.len() < 4096 {
        document.content.len()
    } else {
        4096
    };

    let res = ollama.generate(
        GenerationRequest::new(
            model.to_string(),
            format!("{prompt} {content}", content = document.content[..limit].to_string()),
        ).format(FormatType::Json)
    ).await;

    if let Ok(res) = res {
        println!("{}", res.response);
    }
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
