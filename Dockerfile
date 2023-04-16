FROM rust:1.67 as builder
WORKDIR /rls 
COPY ./Cargo* .
COPY ./src ./src
RUN cargo install --force --path .

FROM debian:bullseye-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/rls /usr/local/bin/rls
CMD ["rls"]
