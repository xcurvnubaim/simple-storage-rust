# Simple Storage Rust

This is a Rust project that provides a simple storage solution using Actix Web and SQLite.

## Configuring Environment Variables

The project uses the following environment variables:

- `STORAGE_BASE_URL`: The base URL for accessing stored files.

Create a `.env` file in the project root directory and add the following lines:

```env
STORAGE_BASE_URL=http://localhost:3000
```

## Running the Project

To run the project, follow these steps:

1. Make sure you have Rust and Cargo installed on your machine.
2. Navigate to the project directory.
3. Run the following command to start the project:

   ```sh
   cargo run
   ```

   This will start the Actix Web server on `http://localhost:3000`.

## Dependencies

The project uses the following dependencies:

- `actix-web`: A powerful, pragmatic, and extremely fast web framework for Rust.
- `serde`: A framework for serializing and deserializing Rust data structures.
- `futures`: An abstraction for asynchronous programming in Rust.
- `tokio`: An asynchronous runtime for the Rust programming language.
- `dotenv`: A crate for loading environment variables from a `.env` file.
- `async-trait`: A crate for defining async functions in traits.
- `actix-multipart`: A crate for handling multipart/form-data requests in Actix Web.
- `uuid`: A crate for generating and working with UUIDs.
- `actix-files`: A crate for serving static files in Actix Web.
- `rusqlite`: A SQLite binding for Rust.
