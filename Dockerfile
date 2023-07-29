FROM alpine:latest

EXPOSE 8080

COPY ./target/release/identity-service .

RUN ls -la .

ENTRYPOINT ["./identity-service"]
