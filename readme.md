# Concert & Events Ticketing System

This project is a work in progress. It is being developed as a microservices-based SaaS system for managing concerts and event ticketing.

Dev env :
``` bash
docker compose up --build
```

Prod env : 
``` bash
docker-compose -f docker-compose.prod.yml up --build
```

If you want multiple instances of services you can do :

Dev env :
``` bash
docker compose up --build --scale auth-service=2 --scale users-service=2 --scale events-service=2 --scale tickets-service=2 --scale notifications-service=2 --scale payments-service=2 --scale backups-service=2
```

Prod env : 
``` bash
docker-compose -f docker-compose.prod.yml up --build --scale auth-service=2 --scale users-service=2 --scale events-service=2 --scale tickets-service=2 --scale notifications-service=2 --scale payments-service=2 --scale backups-service=2
```

## Project Overview

The system is designed to handle ticketing for events ranging from small local shows to large international tours. It is built using a microservices architecture where each service is responsible for a specific domain:

- **API Gateway**: Routes incoming requests to the appropriate microservice.
- **Auth Service**: Manages user authentication and authorization (JWT token management).
- **Users Service**: Handles user profile information and management.
- **Events Service**: Manages event creation, updating, deletion, and querying of event details.
- **Tickets Service**: Manages ticket purchases, ensuring no overselling and linking tickets to users.
- **Notification Service**: Simulates sending notifications (email/SMS) to confirm ticket purchases.

## Tech Stack

- **Programming Language**: Rust (using Actix Web)
- **Database**: MongoDB (each microservice has its own database/collection)
- **Containerization**: Docker & Docker Compose

## Architecture

The system follows a microservices architecture where each service runs as an independent container. The API Gateway is responsible for routing the requests from the client to the corresponding services. Communication between services is done through HTTP calls or asynchronous messaging.

## Current Status

- The project is currently under active development.
- Core services and basic endpoints have been set up.
- Testing is performed using API testing tools like Postman and cURL since there is no UI at the moment.
- Future work includes enhancing service inter-communication, adding detailed logging, and thorough testing.

## How to Run

1. Clone the repository.
2. Navigate to the project root directory.
3. Build and run all services using Docker Compose:
   ```bash
   docker-compose up --build
   ```
4. Use API testing tools (Postman, Insomnia, or cURL) to interact with the endpoints.

## Contributing

Since this is a work in progress, any contributions, feedback, or suggestions are welcome. Please open an issue or submit a pull request.

*Note: This README is a temporary placeholder. Further documentation and detailed instructions will be added as the project evolves.*
