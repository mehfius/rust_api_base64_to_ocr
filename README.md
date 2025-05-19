# Base64 to OCR Service

A high-performance OCR (Optical Character Recognition) service built with Rust that converts base64-encoded images to text. This service is particularly optimized for Portuguese language text recognition.

## Features

- Fast and efficient OCR processing using Tesseract
- Base64 image input support
- Optimized for Portuguese language text recognition
- Clean text output with control character removal
- Processing time measurement
- RESTful API endpoint

## Prerequisites

- Rust (latest stable version)
- Tesseract OCR engine
- Portuguese language data for Tesseract

## Installation

1. Install Tesseract OCR:
```bash
# Ubuntu/Debian
sudo apt-get install tesseract-ocr
sudo apt-get install tesseract-ocr-por

# macOS
brew install tesseract
brew install tesseract-lang
```

2. Clone the repository:
```bash
git clone https://github.com/yourusername/base64_to_ocr.git
cd base64_to_ocr
```

3. Build the project:
```bash
cargo build --release
```

## Usage

1. Start the server:
```bash
cargo run --release
```

The server will start at `http://127.0.0.1:8080`

2. Send a POST request to the `/ocr` endpoint with a JSON payload:
```json
{
    "base64": "your_base64_encoded_image_string"
}
```

Example using curl:
```bash
curl -X POST http://127.0.0.1:8080/ocr \
     -H "Content-Type: application/json" \
     -d '{"base64": "your_base64_encoded_image_string"}'
```

## Response Format

Successful response:
```json
{
    "text": "extracted text from image",
    "processing_time_ms": 123
}
```

Error response:
```json
{
    "error": "error message",
    "expected_format": {
        "message": "O servidor espera um JSON com um campo 'base64' contendo uma string Base64 válida representando uma imagem (ex.: PNG ou JPEG).",
        "example": {
            "base64": "iVBORw0KGgoAAAANSUhEUgAA..."
        }
    }
}
```

## Dependencies

- actix-web: Web framework
- base64: Base64 encoding/decoding
- image: Image processing
- rusty-tesseract: Tesseract OCR bindings
- serde: Serialization/deserialization

## License

[Add your chosen license here]

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. 