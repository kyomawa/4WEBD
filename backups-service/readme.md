## Backup Service

**Purpose:**
Centralizes the management and scheduling of backups for one or more microservices.

![Backup Model](https://cloud.bryancellier.fr/api/v1/buckets/public/objects/download?preview=true&prefix=backups.png&version_id=null)

### Endpoints

- **GET `/backups/:service_name/last`**
    - **Description:** Retrieves the most recent backup performed for a specified service.
    - **Access:** Restricted to users with an `admin` role.
- **GET `/backups/:id`**
    - **Description:** Retrieves detailed information about a specific backup.
    - **Access:** Restricted to users with an `admin` role.
- **POST `/backups`**
    - **Description:** Initiates a new backup for the specified collection.
    - **Operation:**
        - The Backup Service contacts the corresponding microservice to retrieve the records to be backed up.
        - The retrieved data is stored in the Backup Service's database.
    - **Access:** Restricted to users with an `admin` role.
- **DELETE `/backups/:id`**
    - **Description:** Deletes a specific backup.
    - **Access:** Restricted to users with an `admin` role.

### Cron Job for Processing Backups

A background cron job runs every 20 seconds to process pending backups:

1. **Fetch Pending Backups:**
    
    The job queries the Backup Service's database for all backup records with a status of `Pending`.
    
2. **Update Status to InProgress:**
    
    For each pending backup, the status is updated to `InProgress` to indicate active processing.
    
3. **Retrieve and Store Data:**
    
    The job contacts the relevant microservice internally to fetch the latest records, storing the fetched data in the backup record’s `data` field.
    
4. **Final Status Update:**
    - If the data retrieval and storage are successful, the backup status is updated to `Completed`.
    - If an error occurs during processing, the status is updated to `Failed`.

This asynchronous process ensures that backups are managed efficiently without blocking user requests, and provides clear tracking of backup status and history.

### Swagger Documentation

For a complete overview of the Backup Service API, please refer to the Swagger documentation available at:

[**http://localhost:80/api/backups/doc/**](http://localhost/api/backups/doc/)