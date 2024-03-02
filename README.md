# Rust Api Challenge

[![Crates.io](https://img.shields.io/crates/v/crate-name.svg)](https://crates.io/crates/crate-name)
[![Documentation](https://docs.rs/crate-name/badge.svg)](https://docs.rs/crate-name)
[![License](https://img.shields.io/crates/l/crate-name.svg)](https://github.com/your-username/crate-name/blob/main/LICENSE)

## Description

A simple API built while learning the Rust Programming Language

## Running

```
cargo run
```

## Features

- List items
```
curl http://localhost:8080/items
```
- Add items
```
curl -X POST http://127.0.0.1:8080/items \
-H "Content-Type: application/json" \
-d '{"id": 1, "name": "Sample Item"}'
```
- Error Handling
```
# Adding duplicates
curl -X POST http://127.0.0.1:8080/items \
-H "Content-Type: application/json" \
-d '{"id": 1, "name": "Sample Item"}'

curl -X POST http://127.0.0.1:8080/items \
-H "Content-Type: application/json" \
-d '{"id": 1, "name": "Sample Item"}'

# Malformed JSON
curl -X POST http://127.0.0.1:8080/items \
-H "Content-Type: application/json" \
-d '{"id": 1, "name": "Sample Item}'
```
- Unit Tests

## Installation

To use this crate, add the following dependency to your `Cargo.toml` file:

```toml
rust_api_challenge="0.1"
```


# Challenge Description
## Challenge: Rust API with POST Request
### Objective

Create a RESTful API in Rust that accepts a POST request to add a new item to a list of items. The item should have at least two fields (e.g., id and name). The API should store these items in memory (no need for a database).

### Requirements

1. Setup: Use the Rust web framework Actix-Web.

2. Data Structure: Define a struct Item with at least two fields:
    1. id: a unique identifier (e.g., integer or UUID).
    2. name: a string representing the name of the item.

3. API Endpoint: Implement a POST endpoint at /items that accepts a JSON body with id and name fields to create a new Item.

4. In-Memory Storage: Store the items in a Vec<Item> or a similar structure.

5. Error Handling: Implement basic error handling for cases such as:
    1. Receiving malformed JSON.
    2. Attempting to add an item with an ID that already exists.

6. Response: The endpoint should return a success status code and the added item in JSON format if the request is successful.

### Bonus Tasks

1. Implement a GET endpoint to retrieve all items.
2. Add unit tests for your endpoints.
3. Implement logging for incoming requests and responses.

### Submission Guidelines

1. Provide the source code with clear instructions on how to run the API.
2. Include comments explaining key parts of your implementation.
3. Ensure the code is well-structured and follows Rust's coding conventions.