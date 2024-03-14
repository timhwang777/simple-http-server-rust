# Simple Rust HTTP Server

![Static Badge](https://img.shields.io/badge/Rust-Solutions-blue?logo=rust
)

## Table of Contents

1. [About the Project](#about-the-project)
2. [Getting Started](#getting-started)
   - [Prerequisites](#prerequisites)
3. [Author](#author)

## About the Project

This project is a minimalistic web server written in Rust. It's designed to handle basic HTTP GET and POST requests. The server can serve files from a specified directory, echo request paths, and display request user-agent information. It's an educational project to demonstrate the basics of TCP networking and HTTP protocol handling in Rust.

The server listens on `127.0.0.1:4221` and supports serving static files, echoing back part of the request URL, and responding with the `User-Agent` header of the request. Additionally, it allows for the creation of new files in the specified directory via POST requests.

Features:
- Serve static files from a specified directory or the current directory if none is specified.
- Echo back data after `/echo/` in the URL path.
- Respond with the `User-Agent` of the HTTP request.
- Handle file creation in the specified directory through POST requests.

## Getting Started

To get a local copy up and running, follow these simple steps:

### Prerequisites

To run this web server, you will need:

- Rust programming environment setup on your machine. You can follow the official guide to install Rust: [Rust Installation](https://www.rust-lang.org/tools/install).
- Basic understanding of Rust and TCP/IP networking.

Once Rust is installed, you can clone this repository or copy the source code into your own Rust project. Make sure to include all the provided code in your `main.rs` and any module files as required.

To run the server, navigate to the project directory in your terminal and execute:
```rust
cargo run main.rs
```

Alternatively, you can execute the `your_server.sh` shell script.
## Author

Timothy Hwang