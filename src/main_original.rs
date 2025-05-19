use std::collections::HashMap;
// 1) Use `image::ImageReader` em vez de `image::io::Reader`.
use image::ImageReader;
use rusty_tesseract::{Image, Args, image_to_string};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 2) Carrega e decodifica a imagem corretamente
    let dyn_img = ImageReader::open("./img/imagem.png")?
        .decode()?;
    let img = Image::from_dynamic_image(&dyn_img)?;

    // 3) Constrói `Args`, convertendo "por" em String
    let args = Args {
        lang: "por".to_string(),             // <-- aqui
        dpi: Some(300),
        psm: Some(6),
        oem: Some(3),
        config_variables: HashMap::new(),
    };

    // 4) Executa o OCR
    let texto = image_to_string(&img, &args)?;
    println!("Texto extraído: {}", texto);

    Ok(())
}
