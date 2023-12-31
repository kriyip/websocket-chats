FROM rust:1.68

WORKDIR /src
COPY . .

RUN cargo install --path .

CMD ["chat-server"]
