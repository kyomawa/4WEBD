## Notification Service

**Purpose:**
Sends confirmations (via email or SMS) regarding ticket purchases and other user-related notifications.

![Notification Model](https://cloud.bryancellier.fr/api/v1/buckets/public/objects/download?preview=true&prefix=notif.png&version_id=null)

### Endpoints

- **GET `/notifications`**
    - **Description:** Lists all notifications.
    - **Access:** Restricted to users with an `admin` role.
- **GET `/notifications/:id`**
    - **Description:** Retrieves detailed information for a specific notification.
    - **Access:** Restricted to users with an `admin` role.
- **POST `/notifications`**
    - **Description:** Creates a new notification request and sets the notification status to `PENDING`.
    - **Usage:** This endpoint is typically called by the Tickets Service after a successful purchase (internal use only).
- **PATCH `/notifications/:id`**
    - **Description:** Updates a notification, such as changing its status from `pending` to `sent` or `failed`.
    - **Access:** Restricted to users with an `admin` role.
- **DELETE `/notifications/:id`**
    - **Description:** Deletes a notification record if necessary.
    - **Access:** Restricted to users with an `admin` role.

### Cron Job for Processing Notifications

A background task runs every 30 seconds to process notifications with a `PENDING` status:

1. **Fetch Pending Notifications:**
    
    The task queries the database for all notifications where `status` is set to `PENDING`.
    
2. **Attempt to Send:**
    
    For each pending notification, the service attempts to send the corresponding email.
    
3. **Update Status:**
    - If the notification is successfully sent, its status is updated to `SENT`.
    - If the sending fails, its status is updated to `FAILED`.

This automated process ensures that notifications are processed asynchronously, keeping the main endpoints (such as ticket creation) responsive without waiting for the notification process to complete.

### Swagger Documentation

For a complete overview of the Notification Service API, please refer to the Swagger documentation available at:

[**http://localhost:80/api/notifications/doc/**](http://localhost/api/notifications/doc/)