FROM rust:1.89.0 as builder

WORKDIR /app
COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /usr/local/bin

COPY --from=builder /app/target/release/pnar-world-backend-rust .

# ENV DATABASE_URL=postgres://postgres:root@localhost:5432/pnar_word
# ENV RUST_LOG=info

EXPOSE 8000

CMD ["./pnar-world-backend-rust"]