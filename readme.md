# Concert & Events Ticketing System

This project is a microservices-based SaaS system designed to manage concert and event ticketing. It caters to both small-scale events (e.g., school functions) and large international tours. The system is built using Rust (with Actix Web) and MongoDB, and it leverages Docker and Docker Compose for containerization and deployment.

---

## Table of Contents

- [Architecture Overview](#architecture-overview)
- [Services Overview](#services-overview)
- [Traefik as API Gateway & Load Balancer](#traefik-as-api-gateway--load-balancer)
- [Endpoints & Swagger Documentation](#endpoints--swagger-documentation)
- [Health Checks](#health-checks)
- [Ticket Purchase Flow](#ticket-purchase-flow)
- [Backups & Cron Jobs](#backups--cron-jobs)
- [UML Diagrams](#uml-diagrams)
- [Environment Configuration](#environment-configuration)
- [JWT Authentication](#jwt-authentication)
- [Development Environment Setup](#development-environment-setup)
- [Production Environment Setup](#production-environment-setup)
- [Scaling Services](#scaling-services)
- [External Resources](#external-resources)
- [Credits](#credits)

---

## Architecture Overview

The system is structured as a set of independent microservices, each responsible for a specific domain. Communication between services is done through HTTP calls and asynchronous messaging where necessary. At the heart of the infrastructure is **Traefik**, which acts as both an API Gateway and a load balancer, routing incoming requests to the appropriate service.

The major components are:

- **API Gateway & Load Balancer (Traefik):** Routes client requests to the relevant service endpoints, manages SSL termination (if needed), and provides a dashboard for monitoring.
- **Auth Service:** Manages user authentication and authorization using JWT tokens.
- **Users Service:** Handles user profiles and related data.
- **Events Service:** Manages events (creation, update, deletion, and queries).
- **Tickets Service:** Processes ticket purchases and ensures no overselling occurs.
- **Notifications Service:** Simulates sending notifications (email/SMS) confirming ticket purchases or other actions.
- **Payments Service:** Handles the payment process for ticket purchases and triggers subsequent actions.
- **Backups Service:** Periodically backs up data from other services (configured to run every 10 minutes in this demo to show functionality).

Each service maintains its own set of APIs and exposes a Swagger (OpenAPI) documentation interface to describe its endpoints.

---

## Services Overview

Each microservice is implemented as a separate Rust application. The key services are:

- **Auth Service:**  
  - Endpoints: `/api/auth/...` (e.g., `/api/auth/doc/` for Swagger documentation, `/api/auth/health` for health check)
  - Manages user login, registration, token refresh, and deletion of credentials.
  - **Additional Documentation:** A README in the `auth-service` folder explains its routes in detail.
  
- **Users Service:**  
  - Endpoints: `/api/users/...` (e.g., `/api/users/doc/` for Swagger documentation, `/api/users/health` for health check)
  - Handles user profile creation, updates, deletion, and queries.
  - **Additional Documentation:** A README in the `users-service` folder explains its routes in detail.
  
- **Events Service:**  
  - Endpoints: `/api/events/...` (e.g., `/api/events/doc/` for Swagger documentation, `/api/events/health` for health check)
  - Manages event lifecycle operations (CRUD operations) and seat updates.
  - **Additional Documentation:** A README in the `events-service` folder explains its routes in detail.
  
- **Tickets Service:**  
  - Endpoints: `/api/tickets/...` (e.g., `/api/tickets/doc/` for Swagger documentation, `/api/tickets/health` for health check)
  - Responsible for ticket purchase operations and ensuring ticket-to-user matching.
  - **Additional Documentation:** A README in the `tickets-service` folder explains its routes in detail.
  
- **Notifications Service:**  
  - Endpoints: `/api/notifications/...` (e.g., `/api/notifications/doc/` for Swagger documentation, `/api/notifications/health` for health check)
  - Processes and sends notifications after key actions (e.g., successful ticket purchase).
  - **Additional Documentation:** A README in the `notifications-service` folder explains its routes in detail.
  
- **Payments Service:**  
  - Endpoints: `/api/payments/...` (e.g., `/api/payments/doc/` for Swagger documentation, `/api/payments/health` for health check)
  - Simulates credit card payments. A successful payment triggers a notification and activates the corresponding ticket.
  - **Additional Documentation:** A README in the `payments-service` folder explains its routes in detail.
  
- **Backups Service:**  
  - Endpoints: `/api/backups/...` (e.g., `/api/backups/doc/` for Swagger documentation, `/api/backups/health` for health check)
  - Regularly backs up data from each service. For demonstration purposes, the backup interval is set to 10 minutes instead of one day.
  - **Additional Documentation:** A README in the `backups-service` folder explains its routes in detail.

---

## Traefik as API Gateway & Load Balancer

![Traefik Architecture](https://cloud.bryancellier.fr/api/v1/buckets/public/objects/download?preview=true&prefix=4WEBD.png)

**Traefik** is the entry point for all external requests. It performs the following functions:

- **Routing:** Directs incoming requests (based on defined rules) to the corresponding microservice.
- **Load Balancing:** Distributes requests across multiple instances of a service.
- **Security:** In production, it is configured with basic authentication for the dashboard.
- **Dashboard:** Provides a monitoring interface for observing the traffic and health of services.

### Traefik Dashboard Access

- **Development:**
  - Accessible at: [http://localhost:8080](http://localhost:8080)
  - Uses insecure mode for simplicity.
- **Production:**
  - Accessible at: [http://traefik.localhost](http://traefik.localhost)
  - Protected via basic authentication.
  
The basic authentication credentials are defined in the `.env` file:
- **DASHBOARD_USER**
- **DASHBOARD_PASSWORD**

---

## Endpoints & Swagger Documentation

Each service exposes its API documentation through Swagger. The default URL pattern is:

- **Auth Service Swagger:** `http://localhost:80/api/auth/doc/`
- **Users Service Swagger:** `http://localhost:80/api/users/doc/`
- **Events Service Swagger:** `http://localhost:80/api/events/doc/`
- **Tickets Service Swagger:** `http://localhost:80/api/tickets/doc/`
- **Notifications Service Swagger:** `http://localhost:80/api/notifications/doc/`
- **Payments Service Swagger:** `http://localhost:80/api/payments/doc/`
- **Backups Service Swagger:** `http://localhost:80/api/backups/doc/`

In all cases, Traefik routes the requests from port **80** (as defined by `HOST_PORT_TRAEFIK=80` in the `.env` file) to the respective service.

---

## Health Checks

Each service is configured with a health check endpoint (`/health`) to ensure its availability. These health checks are used by both Traefik and Docker Compose to monitor the status of services. For example:

- **Auth Service Health Check:** `http://localhost:8080/api/auth/health`
- **Users Service Health Check:** `http://localhost:8080/api/users/health`
- ... and so on for other services.

Health checks are configured with appropriate intervals and timeouts to ensure prompt detection of any issues.

---

## Ticket Purchase Flow

When a ticket is created via the **Tickets Service**:

1. **Ticket Creation:**  
   A ticket purchase request is received and processed.
2. **Payment Process:**  
   The service triggers a payment via the **Payments Service**. The payment starts in a `PENDING` state.
3. **Payment Validation:**  
   A background cron job in the Payments Service simulates the payment validation. If successful, the payment status is updated to `Success`.
4. **Notification Trigger:**  
   Upon successful payment, a notification is created via the **Notifications Service**.
5. **Ticket Activation:**  
   The successful payment also triggers an internal call to activate the ticket (changing its status to `Active`).

This flow ensures that all operations are atomic and that the user is notified at each step.

---

## Backups & Cron Jobs

To ensure data integrity and meet legal requirements, each service has a **Backups Service** that periodically backs up data:

- In this demo, the backup interval is set to **10 minutes** instead of one day. This frequency allows you to quickly verify that the backup mechanism is functioning.
- A background cron job within the Backups Service:
  - Fetches data from the respective microservice.
  - Stores the data in the backup database.
  - Updates the backup status (e.g., `Pending`, `InProgress`, `Completed`, or `Failed`).

---

## UML Diagrams

![UML Diagrams](https://cloud.bryancellier.fr/api/v1/buckets/public/objects/download?preview=true&prefix=4WEBD_2.png&version_id=null)

This section provides an overview of the UML diagrams and architectural schematics for the entire system. These diagrams illustrate the structure and interactions between the microservices.

---

## Environment Configuration

The system relies on a set of environment variables defined in a `.env` file. Key variables include:

```bash
# Ports
HOST_PORT_TRAEFIK=80
HOST_PORT_TRAEFIK_DASHBOARD=8080
HOST_PORT_MONGODB=27017

# Traefik Dashboard Credentials
DASHBOARD_USER=username
DASHBOARD_PASSWORD=password
DASHBOARD_PASSWORD_HASH=passwordhashed

# JWT Signatures
JWT_INTERNAL_SIGNATURE=your_internal_signature
JWT_EXTERNAL_SIGNATURE=your_external_signature

# Database URLs for each service
DATABASE_URL_AUTH_SERVICE=mongodb://...:27017/auth_db
DATABASE_URL_USERS_SERVICE=mongodb://...:27017/users_db
DATABASE_URL_EVENTS_SERVICE=mongodb://...:27017/events_db
DATABASE_URL_TICKETS_SERVICE=mongodb://...:27017/tickets_db
DATABASE_URL_NOTIFICATIONS_SERVICE=mongodb://...:27017/notifications_db
DATABASE_URL_PAYMENTS_SERVICE=mongodb://...:27017/payments_db
DATABASE_URL_BACKUPS_SERVICE=mongodb://...:27017/backups_db

# Email Configuration (for Notifications Service)
MAIL_HOSTNAME=smtp.example.com
MAIL_USERNAME=your_email_username
MAIL_PASSWORD=your_email_password
```
---

## JWT Authentication

All APIs utilize two distinct JWT tokens to secure routes:

- **Internal JWT:**
    
    Used for routes that are meant to be accessed only internally by microservices. This token ensures secure communication between services.
    
- **External JWT:**

    Used to protect routes that require a user to be authenticated. Public routes remain open and do not require a token.
    

This dual-token approach allows for a robust security model, ensuring both inter-service communication and user interactions are properly secured.

---

## Development Environment Setup

To run the system in a development **environment**, use the provided `docker-compose.yml` file.

### Steps:

1. **Clone the repository.**
2. **Ensure the `.env` file is correctly configured.**
3. **Run the following command:**
    
    ```bash
    docker compose up --build
    ```
    
4. **Access Services:**
    - Traefik dashboard: [http://localhost:8080](http://localhost:8080/)
    - Swagger documentation for each service is available via Traefik on port 80 (e.g., [http://localhost:80/api/auth/doc/](http://localhost/api/auth/doc/)).

---

## Production Environment Setup

For production, use the `docker-compose.prod.yml` file and the production Dockerfile.

### Steps:

1. **Clone the repository.**
2. **Ensure the `.env` file is correctly configured for production.**
3. **Run the following command:**
    
    ```bash
    docker-compose -f docker-compose.prod.yml up --build
    ```
    
4. **Access Services:**
    - Traefik dashboard: [http://traefik.localhost](http://traefik.localhost/) (secured with basic auth).
    - Swagger documentation for each service is available via Traefik on port 80 (e.g., http://traefik.localhost/api/auth/doc/).

---

## Scaling Services

Both development and production setups support scaling of services using Docker Compose's `--scale` flag. For example, to run two instances of each service in **development**:

```bash
docker compose up --build --scale auth-service=2 --scale users-service=2 --scale events-service=2 --scale tickets-service=2 --scale notifications-service=2 --scale payments-service=2 --scale backups-service=2
```

For **production**, run:

```bash
docker-compose -f docker-compose.prod.yml up --build --scale auth-service=2 --scale users-service=2 --scale events-service=2 --scale tickets-service=2 --scale notifications-service=2 --scale payments-service=2 --scale backups-service=2
```

This allows you to simulate a load-balanced, scalable microservices environment.

---

## External Resources

For further details and collaborative discussions, please refer to the external resources below:

- **Notion Documentation:**
    
    [https://stingy-lift-9ed.notion.site/4WEBD-1b659930250580b790bfcfce22460c31?pvs=4](https://www.notion.so/4WEBD-1b659930250580b790bfcfce22460c31?pvs=21)
    
- **FigJam Diagrams:**
    
    https://www.figma.com/board/9xhEBDfFMlQnbiUsrNOPNs/4WEBD?node-id=1-3003&t=ViKz7soe83BnP5Nt-1

---

## Credits

Project realised by :

- [kyomawa](https://github.com/kyomawa)
- [lyeschougar](https://github.com/lyeschougar)

---