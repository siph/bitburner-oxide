FROM clux/muslrust AS builder
WORKDIR /volume
COPY . .
RUN cargo build --release

FROM alpine
COPY --from=builder /volume/target/x86_64-unknown-linux-musl/release/bitburner-oxide .
ENTRYPOINT [ "/bitburner-oxide" ]
