FROM rust:1.46-alpine3.12

RUN apk add --no-cache musl-dev 'postgresql-dev<12.0' openssl-dev openssl-libs-static

ENV PQ_LIB_STATIC_X86_64_UNKNOWN_LINUX_MUSL=true \
    OPENSSL_STATIC=true

WORKDIR /work
COPY . .
RUN cargo build
