FROM rustlang/rust:nightly-slim
MAINTAINER Liam Naddell (liamnprg@gmail.com)
RUN mkdir -p /app
WORKDIR /app
COPY . .
RUN cargo build --release
CMD ["cargo", "run", "--release"]
