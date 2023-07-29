FROM debian:bookworm-slim

EXPOSE 8080

COPY ./target/release/identity-service .

ENTRYPOINT ["./identity-service"]
