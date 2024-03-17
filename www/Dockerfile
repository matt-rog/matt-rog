FROM rust:alpine3.18

EXPOSE 8080

ENV MRC_PORT="8080"

COPY ./ ./

RUN cargo build --release

CMD ["./target/release/www"]
