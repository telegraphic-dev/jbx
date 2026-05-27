FROM rust:1.86-bookworm AS builder
WORKDIR /usr/src/juv
COPY . .
RUN cargo build --release --locked

FROM eclipse-temurin:25-jdk-jammy
COPY --from=builder /usr/src/juv/target/release/juv /usr/local/bin/juv
ENTRYPOINT ["juv"]
CMD ["--help"]