# A Multithreaded Web Server in Rust

This tutorial will guide you through the process of running a multi-threaded Rust web API project.

## Prerequisites

- Rust and Cargo installed on your system
- Git installed on your system

## Steps

1. **Clone the repository**

   Open a terminal and run:
   ```
   git clone git@github.com:Shahrullo/building-computer-systems-with-rust.git
   cd building-computer-systems-with-rust/multithreaded-web-server
   ```

2. **Check the Rust version**

   Ensure you have the correct Rust version:
   ```
   rustc --version
   ```
   If needed, update Rust:
   ```
   rustup update
   ```

3. **Build the project**

   Compile the project:
   ```
   cargo build
   ```

4. **Run the project**

   Start the web API:
   ```
   cargo run
   ```

5. **Test the API**

   Open a web browser or use a tool like curl to test the API endpoints. The exact URLs will be on localhost:8080.

## Troubleshooting

- If you encounter any dependency issues, try running `cargo update` to update the project's dependencies.
- Check the project's README or documentation for any specific setup instructions or environment variables that need to be set.

## Conclusion

You should now have the Rust web API running on your local machine. Refer to the project's documentation for information on available endpoints and how to use the API.