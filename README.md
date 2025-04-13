# 🧮 Rust Calculator App (Backend Only)

This is a modular and extensible calculator application written in **Rust**, following **Hexagonal Architecture (Ports and Adapters)**. It provides basic arithmetic operations (`add`, `subtract`, `multiply`, `divide`) and is exposed via an **Actix-web HTTP API**.

## 📁 Project Structure

```
calculator_app_in_rust/
├── Cargo.toml                   # Workspace root
└── src/
    ├── main.rs                  # Actix-web server bootstrap
    ├── app/                     # Application core (use cases)
    ├── domain/                  # Domain models & business logic
    ├── adapters/
    │   ├── http/                # HTTP Adapters (Actix routes)
    │   └── ui/                  # CLI / UI Adapters (if needed)
    └── infrastructure/          # Frameworks, servers, DBs (e.g. actix-web)
```

## 🛠️ Technologies Used

- 🦀 [Rust](https://www.rust-lang.org/)
- 🌐 [Actix-web](https://actix.rs/) – Web server framework
- 🧱 Clean/Hexagonal Architecture

## 🚀 Running the Backend

### 1. Clone the Repository

```bash
git clone https://github.com/your-username/calculator_app_in_rust.git
cd calculator_app_in_rust
```

### 2. Build the Project

```bash
cargo build
```

### 3. Run the Server

```bash
cargo run
```

By default, the Actix web server will run at:  
**http://localhost:8080**

## 📡 Available Endpoints

All endpoints expect a `POST` request with a JSON payload:

- `POST /add`
- `POST /subtract`
- `POST /multiply`
- `POST /divide`

### ✅ Sample Request

```bash
curl -X POST http://localhost:8080/add \
  -H "Content-Type: application/json" \
  -d '{"a": 5, "b": 3}'
```

### ✅ Sample Response

```json
{
  "result": 8
}
```

## 🧱 Architecture Overview

This project follows **Hexagonal Architecture**, which separates:

- **Core business logic** (domain + app)
- **Ports** (traits/interfaces)
- **Adapters** (HTTP server, CLI, UI, etc.)
- **Infrastructure** (Actix, DBs, frameworks)

This makes the project testable, maintainable, and easily extensible.

## 📦 Future Enhancements

- Logging & error handling
- CLI adapter
- Frontend interface (currently omitted)
- Docker deployment

## 📝 License

MIT License