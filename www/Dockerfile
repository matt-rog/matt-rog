FROM rust:alpine3.18

EXPOSE 80

ENV MRC_PORT="80"

COPY ./ ./

RUN cargo build --release

CMD ["./target/release/www"]
