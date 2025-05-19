# Usar a imagem oficial do Rust como base
FROM rust:1.82 AS builder

# Instalar dependências do sistema para o Tesseract e Git
RUN apt-get update && apt-get install -y \
    tesseract-ocr \
    tesseract-ocr-por \
    libtesseract-dev \
    clang \
    git \
    && rm -rf /var/lib/apt/lists/*

# Criar diretório de trabalho
WORKDIR /usr/src/rust_api_base64_to_ocr

# Clonar o repositório do GitHub
RUN git clone https://github.com/mehfius/rust_api_base64_to_ocr.git .

# Compilar o projeto em modo release
RUN cargo build --release

# Imagem final para execução
FROM debian:bookworm-slim

# Instalar dependências de runtime
RUN apt-get update && apt-get install -y \
    tesseract-ocr \
    tesseract-ocr-por \
    && rm -rf /var/lib/apt/lists/*

# Copiar o binário compilado da etapa de build
COPY --from=builder /usr/src/rust_api_base64_to_ocr/target/release/rust_api_base64_to_ocr /usr/local/bin/

# Expor a porta 8080
EXPOSE 8080

# Executar o servidor
CMD ["rust_api_base64_to_ocr"]