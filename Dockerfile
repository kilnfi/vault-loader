FROM rust:1.72 as builder

COPY . .

RUN RUSTFLAGS='-C target-feature=+crt-static' cargo install --path . --target x86_64-unknown-linux-gnu

FROM scratch

COPY --from=builder /usr/local/cargo/bin/vault-loader /

ENTRYPOINT ["/vault-loader"]
