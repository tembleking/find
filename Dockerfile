FROM rust as builder
WORKDIR /app
COPY . /app
RUN cargo build --release

FROM debian
COPY --from=builder /app/target/release/find /usr/bin/find
CMD ["sleep", "100000000"]