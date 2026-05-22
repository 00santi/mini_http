# Mini Async HTTP Server

A small async HTTP server written in Rust with Hyper and Tokio. It includes a thin app layer that keeps most route logic separate from Hyper's request and response types, plus a translator layer that converts between the two.

I built this as a learning project for async Rust, basic HTTP routing, and containerizing a Rust service with Docker.

## Endpoints

| Layer | Method | Path | Description |
| --- | --- | --- | --- |
| Hyper | GET | `/` `/health` | Returns `ok!` |
| Hyper | GET | `/echo` | Echoes request headers |
| Hyper | POST | `/echo_body` | Echoes the request body |
| App | GET | `/time` | Returns the current system time |
| App | POST | `/sum` | Returns the sum from JSON like `{ "a": 2, "b": 3 }` |
| App | Any | invalid path | Returns `404!` |

## Run Locally

```bash
cargo run
```

The server binds to `0.0.0.0:7878`.

Example request:

```bash
curl -X POST http://127.0.0.1:7878/sum \
  -H "content-type: application/json" \
  -d '{"a":2,"b":3}'
```

## Run With Docker

```bash
docker build -t mini_http .
docker run --rm -p 7878:7878 mini_http
```

The Dockerfile uses a simple multi-stage build: the Rust image compiles the release binary, and the final Debian image runs only that binary.

## Tests

```bash
cargo test
```

The tests cover the app router, translator layer, and a couple of end-to-end route paths through the Hyper router.

## Notes

- The app layer is intentionally small and framework-independent from Hyper, which makes route logic easier to test.
- Request bodies are eagerly converted into strings for simplicity. That is fine for this project, but streaming would be better for large payloads.
- Translator errors currently collapse into `404` responses. More specific `400` or `405` responses would be a natural next step.
