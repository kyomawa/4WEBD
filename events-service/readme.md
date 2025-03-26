## Events Service

**Purpose:**
Manages event data such as title, description, date, capacity, and other related information.

![Event Model](https://cloud.bryancellier.fr/api/v1/buckets/public/objects/download?preview=true&prefix=events.png&version_id=null)

### Endpoints

- **GET `/events`**
    - **Description:** Lists all available events.
- **GET `/events/:id`**
    - **Description:** Retrieves detailed information for a specific event identified by its ID.
- **POST `/events`**
    - **Description:** Creates a new event.
    - **Access:** Restricted to users with `admin` or `eventCreator` roles.
- **PUT `/events/:id`**
    - **Description:** Updates an existing event.
    - **Access:** Restricted to users with `admin` or `eventCreator` roles.
- **PATCH `/events/:id/update-seats`**
    - **Description:** Updates the `remaining_seats` for an event.
    - **Usage:** Intended for internal calls only.
    - **Payload:** Accepts a JSON object with a delta value (e.g., `{ "delta": 1 }` to increment or `{ "delta": -1 }` to decrement the remaining seats).
- **DELETE `/events/:id`**
    - **Description:** Deletes a specific event.
    - **Access:** Restricted to users with `admin` privileges, or in some cases, the `eventCreator` who created the event.

### Swagger Documentation

For a complete overview of the Events Service API, please refer to the Swagger documentation available at:

[**http://localhost:80/api/events/doc/**](http://localhost/api/events/doc/)