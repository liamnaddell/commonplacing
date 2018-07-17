MAINTAINER Liam Naddell (liamnprg@gmail.com)
CMD ["cargo", "run", "--release"]


FROM rustlang/rust:nightly-slim
RUN mkdir -p /app
WORKDIR /app
COPY . .
RUN cargo build --release

FROM alpine:latest  
COPY --from=builder /app .
WORKDIR /app/
CMD ["/bin/sh"]  

