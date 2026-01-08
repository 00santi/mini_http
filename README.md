# Mini Async HTTP Server

## Overview
- Minimal HTTP server built with **Hyper** and **Tokio**.
- Includes a small app wrapper that abstracts HTTP requests into framework-agnostic types (Strings, Vecs, enums).
- Built as a learning project to practice async, HTTP networking, and using third-party libraries.

---

## Dependencies
* **Tokio** for async
* **Hyper** for HTTP
* **Serde** for JSON deserialization

---

## Endpoints

| Layer         | Method | Path          | Description                      |
| ------------- | ------ | ------------- | -------------------------------- |
| Hyper-encoded | GET    | `/` `/health` | Returns `"ok!"`                  |
| Hyper-encoded | GET    | `/echo`       | Echoes request headers           |
| Hyper-encoded | POST   | `/echo_body`  | Echoes request body              |
| App-encoded   | GET    | `/time`       | Returns current system time      |
| App-encoded   | POST   | `/sum`        | Returns sum from JSON `{ a, b }` |
| App-encoded   | Any    | invalid path  | Returns `"404!"`                 |

---

## Learning Achievements

* **Decoupled app logic from HTTP:** Created a Hyper-independent app layer for easy endpoint addition and domain modeling, at the cost of performance.
* **Async reasoning:** Distinguished between synchronous (`GET`) and asynchronous (`POST`) handlers.
* **Error modeling:** Designed translator errors (`InvalidMethod`, `InvalidHeaders`, `InvalidBody`), while separating domain logic from HTTP-layer concerns.
* **Evaluated architectured tradeoffs:** Balanced simplicity, performance, and API usability (e.g., optional eager `String` conversion vs async streaming bodies, collapsing errors to a single `404` response for simplicity).
* **Third-party library integration:** Worked with Hyper, Tokio, and Serde. Explored app-layer patterns similar to those used in higher-level libraries like `Actix` and `Axum`.

---

## Tradeoffs / Notes

* **Decoupling vs performance:** App layer simplifies domain logic but introduces extra overhead for body conversion.
* **Eager string conversion:** Simplifies code but reduces throughput compared to async streaming.
* **Error handling:** Translator errors are collapsed into 404 responses. But internal error types allow future expansion (`400` / `405`).
* **Framework-agnostic design:** app module only depends on Rust core types.
---

## How to Run

```bash
cargo run
```
Server listens on `127.0.0.1:7878`.

---

## Future Work

* **Unit tests** for translator, hyper endpoints, and app endpoints
* Mapping specific translator errors to proper HTTP codes
