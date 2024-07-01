# Raw HTTP Get method

This project is to showcase of creating manual `get` method in raw HTTP with Rust

The project spans the following listings. It is a quite large project. Each file provides a different role:
- `main.rs` — Handles command-line parsing and weaves together the functionality provided by its peer files.
- `ethernet.rs` — Generates a MAC address and converts between MAC address types (defined by the smoltcp crate).
- `http.rs` — Carries out the work of interacting with the server to make the HTTP request.
- `dns.rs` — Performs DNS resolution, which converts a domain name to an IP address.

## Run the project
Run `cargo run` in the project's root.

!Make sure rustc and cargo are installed in the machine.