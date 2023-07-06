FROM rust AS builder

COPY ./ ./

RUN cargo build --release

FROM rust AS runner

COPY --from=builder ./target/release/s3-download-files-capnp-generator /usr/local/bin
