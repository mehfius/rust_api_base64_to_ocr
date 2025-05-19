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
RUN git clone https://github.com/mehfius/rust_api_base64_to_ocr.git  .

# Exibir informações do commit mais recente
RUN git log -1 --format=%H > /GIT_COMMIT_ID.txt && \
    git log -1 --format=%cd > /GIT_COMMIT_DATE.txt && \
    git log -1 --format=%s > /GIT_COMMIT_MESSAGE.txt

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

# Copiar metadados do Git para a imagem final
COPY --from=builder /GIT_COMMIT_ID.txt /GIT_COMMIT_DATE.txt /GIT_COMMIT_MESSAGE.txt /opt/git-info/


# Expor a porta 5000
EXPOSE 5000

# Comando para executar o servidor + exibir versão Git ao iniciar
CMD echo "Git Commit ID: $(cat /opt/git-info/GIT_COMMIT_ID.txt)"; \
    echo "Commit Date: $(cat /opt/git-info/GIT_COMMIT_DATE.txt)"; \
    echo "Commit Message: $(cat /opt/git-info/GIT_COMMIT_MESSAGE.txt)"; \
    echo "Starting server..."; \
    rust_api_base64_to_ocr