use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::generation::parameters::FormatType;
use ollama_rs::Ollama;
use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use substring::Substring;

#[derive(Serialize, Debug)]
struct DocumentUpdateRequest {
    tags: Vec<u32>,
    title: String,
}

#[derive(Deserialize, Debug)]
struct Document {
    id: u32,
    tags: Vec<u32>,
    content: String,
    ollama_summary: Option<String>,
}

#[derive(Deserialize, Debug)]
struct DocumentsResponse {
    results: Vec<Document>,
}

#[tokio::main]
async fn main() {
    let mut run = true;

    while run {
        let mut documents = load_documents_to_process(
            std::env::var("PAPERLESSNGX_URL").unwrap(),
            std::env::var("PAPERLESSNGX_TOKEN").unwrap(),
            std::env::var("PAPERLESSNGX_TAGS").unwrap(),
        )
        .await
        .unwrap();
        run = documents.len() > 0;

        documents = generate_documents_summary_via_ollama(
            documents,
            std::env::var("OLLAMA_HOST").unwrap(),
            std::env::var("OLLAMA_PORT")
                .unwrap()
                .parse::<u16>()
                .unwrap(),
            std::env::var("OLLAMA_MODEL").unwrap(),
            std::env::var("OLLAMA_LANGUAGE").unwrap(),
        )
        .await;
        for document in documents {
            update_document(
                document,
                std::env::var("PAPERLESSNGX_URL").unwrap(),
                std::env::var("PAPERLESSNGX_TOKEN").unwrap(),
                std::env::var("PAPERLESSNGX_TAGS").unwrap(),
            )
            .await;
        }
    }
}

async fn update_document(
    document: Document,
    paperless_url: String,
    paperless_token: String,
    paperless_tag: String,
) {
    let summary = document.ollama_summary.unwrap();
    let limit = if summary.len() < 125 {
        summary.len()
    } else {
        125
    };

    let document_updates = DocumentUpdateRequest {
        title: summary.to_string().substring(0, 125).parse().unwrap(),
        tags: document
            .tags
            .iter()
            .copied()
            .filter(|&x| x.to_string() != paperless_tag)
            .collect(),
    };

    let request_url = format!(
        "{paperless_url}/api/documents/{document_id}/",
        document_id = document.id
    );
    let response = Client::new()
        .patch(&request_url)
        .header("Authorization", format!("Token {paperless_token}",))
        .json(&document_updates)
        .send()
        .await
        .unwrap();

    if response.status().is_success() {
        println!("Document {} updated", document.id)
    } else {
        panic!(
            "Couldn't update document {} {:?} {}",
            document.id,
            response.status(),
            response.text().await.unwrap()
        )
    }
}

async fn generate_documents_summary_via_ollama(
    documents: Vec<Document>,
    ollama_host: String,
    ollama_port: u16,
    ollama_model: String,
    ollama_response_language: String,
) -> Vec<Document> {
    let mut updated_documents = documents;

    for document in updated_documents.iter_mut() {
        document.ollama_summary = Some(
            generate_document_summary_via_ollama(
                document,
                ollama_host.clone(),
                ollama_port,
                ollama_model.clone(),
                ollama_response_language.clone(),
            )
            .await,
        );

        println!("{}: {:?}", document.id, document.ollama_summary)
    }

    updated_documents
}

async fn generate_document_summary_via_ollama(
    document: &Document,
    ollama_host: String,
    ollama_port: u16,
    ollama_model: String,
    ollama_response_language: String,
) -> String {
    let ollama = Ollama::new(ollama_host, ollama_port);
    let prompt = format!(
        "summarize the following text within 120 characters.\
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
        the summary must be in {ollama_response_language} language \n\n"
    );

    let limit = if document.content.len() < 4096 {
        document.content.len()
    } else {
        4096
    };

    let res = ollama
        .generate(
            GenerationRequest::new(
                ollama_model,
                format!(
                    "{prompt} {content}",
                    content = document.content.to_string().substring(0, limit)
                ),
            )
            .format(FormatType::Json),
        )
        .await;

    if let Ok(res) = res {
        json::parse(&*res.response).unwrap()["summary"].to_string()
    } else {
        "".to_string()
    }
}

async fn load_documents_to_process(
    paperless_url: String,
    paperless_token: String,
    paperless_tags: String,
) -> Result<Vec<Document>, Error> {
    let request_url = format!("{paperless_url}/api/documents/?tags__id__all={paperless_tags}",);
    let response = Client::new()
        .get(&request_url)
        .header("Authorization", format!("Token {paperless_token}",))
        .send()
        .await?;

    if response.status().is_success() {
        Ok(response.json::<DocumentsResponse>().await?.results)
    } else {
        println!("{:?}", response);
        panic!("Couldn't load documents")
    }
}
