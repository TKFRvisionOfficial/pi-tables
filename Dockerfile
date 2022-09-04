FROM rust AS builder

RUN set -ex; \
    apt-get update; \
    apt-get install -y --no-install-recommends python3 python3-pip libleptonica-dev libtesseract-dev clang; \
    rm -rf /var/lib/apt/lists/*; \
    pip install maturin

WORKDIR /src
COPY . .

RUN maturin build -r
