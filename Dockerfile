FROM gcr.io/distroless/cc
LABEL org.opencontainers.image.description identity-service-on-rust-axum

EXPOSE 8080

COPY ./target/release/identity-service .

ENTRYPOINT ["./identity-service"]