# Builder
from rust:latest as builder

run rustup target add x86_64-unknown-linux-musl
run apt update && apt install -y musl-tools musl-dev
run update-ca-certificates

workdir /app
copy . .
run cargo build --target x86_64-unknown-linux-musl --release

# Final Image
FROM scratch
workdir /app
copy --from=builder /app/target/x86_64-unknown-linux-musl/release/backend-rs .
cmd ["/app/backend-rs"]
