## Auth Service

**Purpose:**
Handles user authentication (login, token management) and basic authorization checks.

![Auth Model](https://cloud.bryancellier.fr/api/v1/buckets/public/objects/download?preview=true&prefix=auth.png&version_id=null)

### Endpoints

- **GET `/auth`**
    - **Description:** Retrieves all credentials.
    - **Usage:** Intended for the backups-service only.
    - **Access:** Restricted to internal requests (using an internal JWT).
- **GET `/auth/me`**
    - **Description:** Returns basic credential information (such as roles and user ID) for the currently authenticated user.
    - **Note:** This functionality could alternatively be handled by the Users Service if more detailed profile data is required.
- **POST `/auth/register`**
    - **Description:** Registers a new user by creating credentials (email/password, roles, etc.).
    - **Note:** In some architectures, this endpoint may also need to trigger a notification to the Users Service to create a corresponding user profile.
- **POST `/auth/login`**
    - **Description:** Authenticates a user using email and password.
    - **Response:** Returns a token (JWT or similar) upon successful authentication.
- **POST `/auth/refresh`**
    - **Description:** Refreshes an existing token when it is near expiration.
    - **Note:** This is optional and depends on the token strategy used.
- **POST `/auth/logout`**
    - **Description:** Invalidates the user session (if server-side sessions are maintained).
    - **Note:** This endpoint is often omitted if stateless JWTs are in use.
- **DELETE `/auth/{user_id}`**
    - **Description:** Deletes the authentication credentials associated with the specified user.
    - **Purpose:** Ensures that when a user is deleted, their authentication data is also removed.
    - **Access:** Restricted to internal requests (using an internal JWT) to prevent unauthorized deletions.

### Swagger Documentation

For a complete overview of the Auth Service API, please refer to the Swagger documentation available at:

[**http://localhost:80/api/auth/doc/**](http://localhost/api/auth/doc/)