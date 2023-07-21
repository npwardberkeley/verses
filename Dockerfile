FROM messense/rust-musl-cross:x86_64-musl as builder
ENV SQLX_OFFLINE=true
WORKDIR /verses

COPY . .

RUN rustup target add x86_64-unknown-linux-musl

RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch
COPY --from=builder /verses/target/x86_64-unknown-linux-musl/release/verses /verses
ENTRYPOINT ["/verses"]
EXPOSE 3000
