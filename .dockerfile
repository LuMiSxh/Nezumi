FROM rust:latest

WORKDIR .

COPY . .

RUN cargo build --release

CMD ["./target/release/nezumi"]
