FROM rust:slim AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

WORKDIR /usr/src/app

COPY . .

RUN cargo build --target x86_64-unknown-linux-musl --release


FROM alpine

WORKDIR /usr/local/bin

ENV PORT=44000

COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/word-of-wisdom-server ./
LABEL service="server"

COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/word-of-wisdom-client ./
LABEL service="client"

EXPOSE 44000
CMD ["/bin/sh", "-c", "if [ \"$SERVICE\" = \"server\" ]; then /usr/local/bin/word-of-wisdom-server; else /usr/local/bin/word-of-wisdom-client; fi"]