FROM gcr.io/distroless/cc

EXPOSE 8080

COPY ./target/release/identity-service .

ENTRYPOINT ["./identity-service"]