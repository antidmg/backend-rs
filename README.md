# backend-rs
Templates and examples for common backend work in Rust.

## API
Build your API with [axum](https://github.com/tokio-rs/axum) - it provides routing, request parsing (extractors), built on `tower` and `tower-http`.

## Caching
Cache API requests with `moka`. (TODO)

## Storage
Persistent storage with Postgres. (TODO)

## Deployments
Containerize the app with Docker scratch image.

### Build Image
In the project root:
``` sh
docker build -t backend-rs:scratch -f Dockerfile .
```

### Run Container
``` sh
docker run -p {LOCAL_MACHINE_PORT}:{APPLICATION_PORT} --rm backend-rs:scratch
```



