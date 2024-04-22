use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use llm::{ModelArchitecture, TokenizerSource};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::PathBuf;
use std::env;
use std::fs;
#[derive(Debug, Deserialize)]
struct ChatRequest {
    prompt: String,
}

#[derive(Debug, Serialize)]
struct ChatResponse {
    response: String,
}

async fn infer(prompt: String) -> Result<String, Box<dyn Error>> {
    let tokenizer_source = TokenizerSource::Embedded;
    let model_architecture = ModelArchitecture::Llama;
    let model_path = PathBuf::from("open_llama_3b-q4_0-ggjt.bin");
    let model = llm::load_dynamic(
        Some(model_architecture),
        &model_path,
        tokenizer_source,
        Default::default(),
        llm::load_progress_callback_stdout,
    )?;

    let mut session = model.start_session(Default::default());
    let mut generated_tokens = String::new();

    let res = session.infer::<std::convert::Infallible>(
        model.as_ref(),
        &mut rand::thread_rng(),
        &llm::InferenceRequest {
            prompt: (&prompt).into(),
            parameters: &llm::InferenceParameters::default(),
            play_back_previous_tokens: false,
            maximum_token_count: Some(140),
        },
        &mut Default::default(),
        |r| match r {
            llm::InferenceResponse::PromptToken(t) | llm::InferenceResponse::InferredToken(t) => {
                generated_tokens.push_str(&t);
                Ok(llm::InferenceFeedback::Continue)
            }
            _ => Ok(llm::InferenceFeedback::Continue),
        },
    );

    match res {
        Ok(_) => Ok(generated_tokens),
        Err(err) => Err(Box::new(err)),
    }
}

async fn chat_handler(input: web::Json<ChatRequest>) -> impl Responder {
    match infer(input.prompt.clone()).await {
        Ok(inference_result) => {
            let response_message = format!("Inference result: {}", inference_result);
            HttpResponse::Ok().json(ChatResponse { response: response_message })
        }
        Err(err) => {
            eprintln!("Error in inference: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if let Ok(current_dir) = env::current_dir() {
        println!("Current directory: {}", current_dir.display());

        // Attempt to read the directory
        match fs::read_dir(current_dir) {
            Ok(entries) => {
                // Iterate over the entries in the directory
                for entry in entries {
                    if let Ok(entry) = entry {
                        // Get the file name as a string
                        if let Some(file_name) = entry.file_name().to_str() {
                            println!("{}", file_name);
                        }
                    }
                }
            }
            Err(err) => {
                eprintln!("Error reading directory: {}", err);
                // Handle the error appropriately (e.g., return or panic)
                // Uncomment the line below to panic on error
                // panic!("Failed to read directory: {}", err);
            }
        }
    } else {
        eprintln!("Failed to get current directory");
        // Handle the error appropriately (e.g., return or panic)
        // Uncomment the line below to panic on error
        // panic!("Failed to get current directory");
    }

    HttpServer::new(|| {
        App::new()
            .service(web::resource("/chat").route(web::post().to(chat_handler)))
    })
    // .bind("127.0.0.1:8080")?
    .bind("0.0.0.0:8080")?
    .run()
    .await
}