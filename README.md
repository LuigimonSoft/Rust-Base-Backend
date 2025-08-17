# Rust-Base-Backend

## About
Rust-Base-Backend is a foundational backend project written in Rust, designed with a clean architecture using the services-repositories pattern. This project serves as a starting point for developing robust and efficient backend systems using the Rust programming language.

## Features
- Modular and scalable architecture
- RESTful API setup
- Static file serving from the `public` directory
- Docker support for containerization
- Middleware for parameter validation
- Error handling conforming to RFC 7807
- Asynchronous programming with Tokio
- Swagger integration for API documentation using OpenAPI

## Architecture
The project is structured to follow the principles of clean architecture, ensuring separation of concerns and maintainability. The main components include:

### Services
Services contain the business logic of the application. They interact with repositories to perform operations and handle the core functionality.

### Repositories
Repositories are responsible for data access and storage. They provide an abstraction layer over the data sources, making it easier to switch between different storage solutions. For demonstration purposes, the project includes a basic example where the repository simulates a database using an in-memory array.

### Controllers
Controllers handle incoming HTTP requests, interact with services, and return appropriate responses. They act as a bridge between the API endpoints and the business logic.

#### Creating a Controller and Endpoint
Here is an example demonstrating how to create a controller and define a simple endpoint:

```rust
use warp::Filter;
use serde_json::json;

// Controller function
pub async fn get_health() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::json(&json!({
        "status": "ok"
    })))
}

// Route definition
pub fn health_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("health")
        .and(warp::get())
        .and_then(get_health)
}

// Adding the endpoint to the router in main function
#[tokio::main]
async fn main() {
    let routes = health_route();

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
```
### Models
Models define the structure of the data used throughout the application and are divided into two main categories:

1. **Database Models:** These models represent the data as it is stored in the database. They define the schema and are used by repositories to interact with the database layer.

2. **Data Transfer Objects (DTOs):** DTOs are used for input and output in the API. They define the structure of data that is sent to and received from the API endpoints, ensuring that only the necessary information is exposed.

### Middleware
The project includes a middleware for validating input parameters to ensure they meet the required criteria before being processed by the application. This is implemented using:
- [base_validator.rs](https://github.com/LuigimonSoft/Rust-Base-Backend/blob/master/src/validators/base_validator.rs): Defines the base validation logic.
- [validator.rs](https://github.com/LuigimonSoft/Rust-Base-Backend/blob/master/src/middleware/validator.rs): Contains specific validation rules and types.

#### Middleware for Input Parameter Validation
The middleware for input parameter validation in this project ensures that incoming requests meet the required criteria before being processed by the services. It leverages various validation rules to check the integrity and format of the input data.

Example of using the validator middleware:

```rust
pub fn validate_create_message(path:Option<String>) -> impl Filter<Extract = (CreateMessageModelDto,), Error = Rejection> + Clone {
    let path = warp::any().map(move || path.clone());
    warp::body::json()
        .and(path)
        .and_then(|body: CreateMessageModelDto, path: Option<String>| async move {
          let content_validation = match Rule::new(body.content.as_ref(),Some("content".to_string()), path)
                .not_null()
                .with_error_code(ErrorCodes::NotNull)
                .not_empty()
                .with_error_code(ErrorCodes::NotEmpty)
                .max_length(32)
                .with_error_code(ErrorCodes::MaxSize)
                .validate() {
                    Ok(_) => Ok(body),
                    Err(err) => Err(err)
                };

                content_validation
                    .map(|body| body)
                    
        })
}
```
### List of Validation Types
The `ValidationRule` enum in the `validator.rs` file defines various types of validation rules:

|  Validation       | Description                                                  |
|------------------|---------------------------------------------------------------|
| **NotNull**      | Ensures that the input value is not null.                     |
| **NotEmpty**     | Ensures that the input string is not empty.                   |
| **MaxLength**    | Checks that the input string does not exceed a specified length. |
| **WithinRange**  | Ensures that the input value is within a specified range.     |
| **IsInteger**    | Validates that the input is an integer.                       |
| **IsDecimal**    | Validates that the input is a decimal number.                 |
| **IsNumber**     | Ensures that the input is a valid number (integer or decimal).|
| **HasDecimals**  | Checks if the input number has a decimal part.                |


### Error Handling
The project includes an error handler that returns errors in accordance with [RFC 7807](https://datatracker.ietf.org/doc/html/rfc7807), the standard for problem details in HTTP APIs. This ensures that error responses are consistent and informative, providing clear details about the issues encountered.

#### Example of an Error Response
Here is an example of an error response conforming to RFC 7807:

```json
{
  "type": "https://example.com/probs/invalid-input",
  "title": "Invalid input",
  "status": 400,
  "instance": "/request/12345",
  "details": [
    {
      "field": "email",
      "message": "Email format is invalid",
      "error_code": 4010
    }
  ]
}
```

### Basic Example with In-Memory Array
The repository in the project includes a basic example where it simulates a database using an in-memory array. This example demonstrates how to store, retrieve, and manipulate data without the need for an actual database. This approach is useful for testing and development purposes.

### Asynchronous Programming with Tokio
The project utilizes [Tokio](https://tokio.rs/), an asynchronous runtime for the Rust programming language, to handle asynchronous operations efficiently. Tokio allows the application to handle many tasks concurrently without blocking the execution thread, which is especially useful for I/O-bound operations like handling multiple HTTP requests.

#### Async/Await Example
Here is a simple example demonstrating the use of async/await in the project:

```rust
use tokio::time::{sleep, Duration};

async fn process_request() {
    // Simulate a delay
    sleep(Duration::from_secs(2)).await;
    println!("Request processed");
}

#[tokio::main]
async fn main() {
    // Call the asynchronous function
    process_request().await;

    // Additional async operations
    let routes = health_route();

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
```
### Swagger Integration
The project integrates Swagger for API documentation using OpenAPI. This allows for automatically generated and interactive API documentation, making it easier for developers to understand and interact with the API endpoints.

#### Enabling Swagger
To enable Swagger documentation in the project, ensure that the necessary dependencies are included and configured to generate the OpenAPI specification, which can then be served and viewed using tools like Swagger UI.

## Getting Started

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- [Docker](https://www.docker.com/get-started) (for containerization)

### Installation
1. Clone the repository:
```bash
   git clone https://github.com/LuigimonSoft/Rust-Base-Backend.git
   cd Rust-Base-Backend
```

2. Build the project:
```bash
  cargo build 
```
3. Run the project
```bash
  cargo run
```
## Contributing
Contributions are welcome! Please fork this repository and submit pull requests.

## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contact
For any questions or issues, please open an issue on this repository.
