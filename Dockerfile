FROM rustlang/rust:nightly-slim as builder
RUN mkdir -p /app
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:stretch-slim  
COPY --from=builder /app /app
WORKDIR /app/
CMD ["./target/release/commonplacing"]  
