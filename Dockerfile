FROM debian:bullseye
RUN apt-get update && apt-get install -y \
    tesseract-ocr \
    tesseract-ocr-por \
    libtesseract-dev \
    clang \
    git \
    && rm -rf /var/lib/apt/lists/*

COPY target/release/rust_api_base64_to_ocr /usr/local/bin/

EXPOSE 5001

CMD ["rust_api_base64_to_ocr"]