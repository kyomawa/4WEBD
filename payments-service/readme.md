## Payment Service

**Purpose:**
Simulates or handles credit card payments, stores payment records, and facilitates regular backups for legal compliance.

![Payment Model](https://cloud.bryancellier.fr/api/v1/buckets/public/objects/download?preview=true&prefix=payme.png&version_id=null)

### Endpoints

- **GET `/payments`**
    - **Description:** Lists all payments.
    - **Access:** Restricted to users with an `admin` role.
- **GET `/payments/:id`**
    - **Description:** Retrieves details of a specific payment.
    - **Access:** Restricted to users with an `admin` role or to the user who made the payment.
- **POST `/payments`**
    - **Description:**
        - Called internally by the Tickets Service when a user attempts to purchase a ticket.
        - Initiates a payment for a given `ticket_id` (or `event_id`), user ID, and amount.
        - Returns a payment record with an initial status of `pending`.
        - Triggers the creation of a new notification.
- **PATCH `/payments/:id`** *(optional)*
    - **Description:** Updates a payment record (e.g., changing the status to `refunded` or handling partial refunds).
- **DELETE `/payments/:id`** *(optional)*
    - **Description:** Removes a payment record, which can be used for voiding transactions or cleaning up test data.

*Note: The `payments` collection is regularly backed up to comply with legal requirements.*

### Cron Job for Processing Payments

A background task runs every 10 seconds to process pending payments:

1. **Fetch Pending Payments:**
    
    The task queries the database for all payments with a status of `PENDING`.
    
2. **Update Status:**
    
    For each pending payment, the status is updated to `Success`.
    
3. **Attempt to Send Notification:**
    
    For each payment that has been marked as `Success`, the service triggers a new notification to the Notifications Service.
    
4. **Activate Ticket:**
    
    For each successful payment, an internal request is made to activate the corresponding ticket.
    

### Swagger Documentation

For a complete overview of the Payment Service API, please refer to the Swagger documentation available at:

[**http://localhost:80/api/payments/doc/**](http://localhost/api/payments/doc/)