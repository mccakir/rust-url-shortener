## Rust URL Shortener Server

This is a simple URL shortener server implemented in Rust using the Actix-Web framework. It allows you to generate short URLs for long ones, making it convenient to share links in a concise manner.
Features
- Shorten long URLs to a customizable short format.
- Retrieve the original URL by using the short URL.

### Getting Started
Prerequisites:
- Rust: Make sure you have Rust installed on your machine.
- Cargo: The Rust package manager.

### Installation
1. Clone the repository:
```
git clone https://github.com/mccakir/rust-url-shortener.git
```

2. Navigate to the project directory:
```
rust-url-shortener
```

3.
```
cargo build --release
```

### Usage
1. Run the server:
```
cargo run --release
```

2. Shorten a URL:

Send a POST request to http://127.0.0.1:8080/shorten with a JSON body containing the original URL:
```
curl -X POST -H "Content-Type: application/json" -d '{"url":"https://www.example.com"}' http://127.0.0.1:8080/shorten
```

3. Access the original URL:

Use a web browser or another tool to access the shortened URL:
```
http://127.0.0.1:8080/{short_url}

```