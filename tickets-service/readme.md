## Tickets Service

**Purpose:**
Handles ticket purchases by linking users to events while ensuring no overselling occurs.

![Ticket Model](https://cloud.bryancellier.fr/api/v1/buckets/public/objects/download?preview=true&prefix=tickets.png&version_id=null)

### Endpoints

- **GET `/tickets`**
    - **Description:**
        - Lists all tickets.
        - For a normal user, only their own tickets are returned.
        - For users with `admin` or `operator` roles, all tickets are returned.
- **GET `/tickets/:id`**
    - **Description:** Retrieves detailed information for a specific ticket.
    - **Access:** Depends on the user's role or ticket ownership.
- **POST `/tickets`**
    - **Description:**
        - Checks if the user is authenticated.
        - Creates (purchases) a new ticket for a given `event_id`.
        - Verifies `remaining_seats` in the Events Service to avoid overselling.
        - Triggers a payment process (simulated or real) and, upon success, creates a ticket record.
- **PATCH `/tickets/:id/active`**
    - **Description:**
        - Intended for internal use only.
        - Activates a ticket by updating its status to `Active`.
        - Creates a new notification upon successful activation.
- **PATCH `/tickets/:id/cancel`**
    - **Description:**
        - Cancels a ticket by updating its status to `Cancelled`.
        - May trigger an increment in `remaining_seats` in the Events Service.
        - Accessible by the ticket owner (under specific conditions) or an administrator.
        - Creates a new notification upon cancellation.
- **PATCH `/tickets/:id/refund`**
    - **Description:**
        - Refunds a ticket by updating its status to `Refunded`.
        - Initiates the refund process (simulated or real) and, upon success, increments `remaining_seats` in the Events Service.
        - Accessible by the ticket owner (under specific conditions) or an administrator.
        - Creates a new notification upon refund.
- **DELETE `/tickets/:id`**
    - **Description:**
        - Permanently deletes (hard delete) a ticket record from the database.
        - **Note:** This action is typically reserved for administrators, as it permanently removes the ticket's history.

### Swagger Documentation

For a complete overview of the Tickets Service API, please refer to the Swagger documentation available at:

[**http://localhost:80/api/tickets/doc/**](http://localhost/api/tickets/doc/)