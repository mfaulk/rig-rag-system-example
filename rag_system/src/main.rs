use std::collections::HashMap;
use anyhow::{Context, Result};
use rig::{
    embeddings::EmbeddingsBuilder,
    loaders::PdfFileLoader,
    providers::openai::{self, TEXT_EMBEDDING_ADA_002},
    vector_store::in_memory_store::InMemoryVectorStore,
    Embed,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use rig::client::{CompletionClient, EmbeddingsClient, ProviderClient};
use rig::providers::openai::TEXT_EMBEDDING_3_SMALL;


#[derive(Embed, Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
struct Document {
    id: String,
    #[embed]
    content: String,
}

/// Read and chunk the text of a PDF.
///
/// # Arguments
/// * 'path' - Path to a PDF file
fn load_pdf(path: PathBuf) -> Result<Vec<String>> {
    let mut chunks = Vec::new();
    let mut current_chunk = String::new();
    let chunk_size = 2000; // Approximately 2000 characters per chunk
    
    for entry in PdfFileLoader::with_glob(path.to_str().unwrap())?.read() {
        let mut content = entry?;
        content.truncate(300000);
        // Split content into words
        let words: Vec<&str> = content.split_whitespace().collect();
        
        for word in words {
            if current_chunk.len() + word.len() + 1 > chunk_size {
                // If adding the next word would exceed chunk size,
                // save current chunk and start a new one
                if !current_chunk.is_empty() {
                    chunks.push(current_chunk.trim().to_string());
                    current_chunk.clear();
                }
            }
            current_chunk.push_str(word);
            current_chunk.push(' ');
        }
    }
    
    // last chunk
    if !current_chunk.is_empty() {
        chunks.push(current_chunk.trim().to_string());
    }

    if chunks.is_empty() {
        anyhow::bail!("No content found in PDF file: {:?}", path);
    }
    
    Ok(chunks)
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize OpenAI client
    let openai_client = openai::Client::from_env();
    
    // Load PDFs using Rig's built-in PDF loader
    let documents_dir = std::env::current_dir()?.join("documents");

    let docs  = [
        // "CFR-2024-title21-vol1.pdf", // 1-99
        // "CFR-2024-title21-vol2.pdf", // 100-169
        // "CFR-2024-title21-vol3.pdf", // 170-199
        "CFR-2024-title21-vol4.pdf", // 200-299
        // "CFR-2024-title21-vol5.pdf", // 300-499
        // "CFR-2024-title21-vol6.pdf", // 500-599
        // "CFR-2024-title21-vol7.pdf", // 
    ];

    let mut doc_to_chunks = HashMap::new();

    for doc in docs.iter() {
        let chunks = load_pdf(documents_dir.join(doc))?;
        doc_to_chunks.insert(doc.to_string(), chunks);
    }
    
    // let moores_law_chunks = load_pdf(documents_dir.join("Moores_Law_for_Everything.pdf"))
    //     .context("Failed to load Moores_Law_for_Everything.pdf")?;
    // let last_question_chunks = load_pdf(documents_dir.join("The_Last_Question.pdf"))
    //     .context("Failed to load The_Last_Question.pdf")?;

    println!("Successfully loaded and chunked {} PDF documents", doc_to_chunks.len());

    // Create embedding model
    let model = openai_client.embedding_model(TEXT_EMBEDDING_3_SMALL);

    // Create embeddings builder
    let mut builder = EmbeddingsBuilder::new(model.clone());

    for (doc, chunks) in doc_to_chunks.iter() {
        for (i, chunk) in chunks.iter().enumerate() {
            let id = format!("{}_{}", doc, i);
            let d = Document{
                id: id.clone(),
                content: chunk.clone(),
            };
            builder = builder.document(d)?;
        }
    }

    // // Add chunks from Moore's Law
    // for (i, chunk) in moores_law_chunks.into_iter().enumerate() {
    //     builder = builder.document(Document {
    //         id: format!("moores_law_{}", i),
    //         content: chunk,
    //     })?;
    // }
    //
    // // Add chunks from The Last Question
    // for (i, chunk) in last_question_chunks.into_iter().enumerate() {
    //     builder = builder.document(Document {
    //         id: format!("last_question_{}", i),
    //         content: chunk,
    //     })?;
    // }

    println!("Added documents to builder");

    // Build embeddings
    let embeddings = builder.build().await?;
    // TODO: Save embeddings


    println!("Successfully generated embeddings");

    // Create vector store and index
    let vector_store = InMemoryVectorStore::from_documents(embeddings);

    let index = vector_store.index(model);


    println!("Successfully created vector store and index");

    // Create RAG agent
    let rag_agent = openai_client
        .agent("gpt-4")
        .preamble("You are a helpful assistant that answers questions based on the provided document context. When answering questions, try to synthesize information from multiple chunks if they're related.")
        .dynamic_context(4, index) // Increased to 4 since we have chunks now
        .build();

    println!("Starting CLI chatbot...");

    // Start interactive CLI
    rig::cli_chatbot::cli_chatbot(rag_agent).await?;

    Ok(())
}