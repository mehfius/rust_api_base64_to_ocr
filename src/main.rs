use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use base64::{engine::general_purpose, Engine as _};
use image::load_from_memory;
use rusty_tesseract::{Image, Args, image_to_string};
use serde::Deserialize;
use std::io;
use std::collections::HashMap;
use std::time::Instant;

/// Estrutura para desserializar o JSON do POST
#[derive(Deserialize)]
struct ImageInput {
    base64: String,
}

/// Limpa o texto removendo caracteres de controle como \n, \f, etc.
fn clean_ocr_text(text: &str) -> String {
    text.chars()
        .filter(|c| !c.is_control()) // Remove caracteres de controle (inclui \n, \f)
        .collect::<String>()
        .trim() // Remove espaços em branco no início e fim
        .to_string()
}

/// Recebe uma string Base64 e retorna o texto extraído por OCR junto com o tempo gasto.
fn extract_text_from_base64(base64_data: &str) -> Result<(String, u128), Box<dyn std::error::Error>> {
    let start_time = Instant::now();
    
    // Valida se a string Base64 é aproximadamente válida (tamanho e caracteres)
    if base64_data.is_empty() || base64_data.len() < 4 || !base64_data.chars().all(|c| c.is_ascii()) {
        return Err("String Base64 inválida".into());
    }

    // Remove possíveis quebras de linha ou espaços da string Base64
    let cleaned_base64 = base64_data.replace("\n", "").replace("\r", "").replace(" ", "");
    
    // Decodifica Base64 usando o Engine recomendado
    let image_bytes = general_purpose::STANDARD.decode(&cleaned_base64)?;
    
    // Carrega a imagem diretamente da memória
    let dyn_img = load_from_memory(&image_bytes)?;
    let img = Image::from_dynamic_image(&dyn_img)?;
    
    // Configurações de OCR
    let mut config_variables = HashMap::new();
    config_variables.insert("tessedit_create_txt".to_string(), "1".to_string()); // Garante saída de texto puro
    
    let args = Args {
        lang: "por".to_string(),
        dpi: Some(300),
        psm: Some(6),
        oem: Some(3),
        config_variables,
    };
    
    // Executa o Tesseract e retorna o texto
    let texto = image_to_string(&img, &args)?;
    
    // Limpa o texto de caracteres indesejados
    let texto_limpo = clean_ocr_text(&texto);
    
    // Calcula o tempo gasto em milissegundos
    let duration_ms = start_time.elapsed().as_millis();
    
    Ok((texto_limpo, duration_ms))
}

/// Endpoint POST para processar a imagem Base64 e retornar o texto OCR
#[post("/ocr")]
async fn ocr_handler(input: Option<web::Json<ImageInput>>) -> impl Responder {
    match input {
        Some(json_input) => match extract_text_from_base64(&json_input.base64) {
            Ok((text, duration_ms)) => HttpResponse::Ok().json(serde_json::json!({
                "text": text,
                "processing_time_ms": duration_ms
            })),
            Err(e) => HttpResponse::BadRequest().json(serde_json::json!({
                "error": e.to_string(),
                "expected_format": {
                    "message": "O servidor espera um JSON com um campo 'base64' contendo uma string Base64 válida representando uma imagem (ex.: PNG ou JPEG).",
                    "example": {
                        "base64": "iVBORw0KGgoAAAANSUhEUgAA..."
                    }
                }
            })),
        },
        None => HttpResponse::BadRequest().json(serde_json::json!({
            "error": "JSON inválido ou ausente no corpo da requisição",
            "expected_format": {
                "message": "O servidor espera um JSON com um campo 'base64' contendo uma string Base64 válida representando uma imagem (ex.: PNG ou JPEG).",
                "example": {
                    "base64": "iVBORw0KGgoAAAANSUhEUgAA..."
                }
            }
        })),
    }
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    println!("Servidor rodando em http://0.0.0.0:5000");
    HttpServer::new(|| {
        App::new()
            .service(ocr_handler)
    })
    .bind("0.0.0.0:5000")?
    .run()
    .await
}