## Users Service

**Purpose:**
Manages user profile data (such as name, email, phone, etc.) independently from authentication credentials.

![User Model](https://cloud.bryancellier.fr/api/v1/buckets/public/objects/download?preview=true&prefix=users.png&version_id=null)

### Endpoints

- **GET `/users`**
    - **Description:** Lists all users.
    - **Access:** Restricted to users with `admin` or `operator` roles.
- **GET `/users/me`**
    - **Description:** Returns the profile of the currently authenticated user.
- **GET `/users/:id`**
    - **Description:** Retrieves the profile of a specific user.
    - **Access:** Restricted to users with `admin`, `operator` roles, or to the user themselves.
- **POST `/users`**
    - **Description:** Creates a new user profile.
    - **Note:** This can be used if registration is split into two steps (i.e., first creating the user profile here, then creating credentials in the Auth Service). Alternatively, the Auth Service's `register` endpoint might handle both actions.
- **PUT `/users/me`**
    - **Description:** Updates the profile of the currently authenticated user.
- **PUT `/users/:id`**
    - **Description:** Updates the profile of a specific user.
    - **Access:** Restricted to users with `admin` or `operator` roles.
- **DELETE `/users/:id`**
    - **Description:** Deletes a user profile.
    - **Access:** Restricted to users with an `admin` role.

### Swagger Documentation

For a complete overview of the Users Service API, please refer to the Swagger documentation available at:

[**http://localhost:80/api/users/doc/**](http://localhost/api/users/doc/)