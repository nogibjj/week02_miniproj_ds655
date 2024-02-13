FROM rust:1.56

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/myapp
COPY . .

RUN cargo build --release --target x86_64-unknown-linux-musl

CMD ["./target/x86_64-unknown-linux-musl/release/my_lambda_function"]