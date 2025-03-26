## Common

**Purpose:**
This directory contains shared code and resources that are used across all microservices in the project.

### Contents

- **JWT Utilities:**
    
    Provides functions and modules for handling both internal and external JWT (JSON Web Tokens) operations. These utilities help in token creation, verification, and management across different services.
    
- **Response Types:**
    
    Defines a standard response object structure used throughout the application. Typically, responses follow the format:
    
    ```json
    {
      "success": true,
      "message": "A descriptive message",
      "data": { ... }
    }
    ```
    
    or, in case of errors:
    
    ```json
    {
      "success": false,
      "message": "Error description",
      "error": "Error message"
    }
    ```
    
- **Common Models:**
    
    Contains shared data models that are used by multiple microservices, ensuring consistency in data representation and reducing duplication.
    

### Usage

Each microservice can import these common utilities and models to handle authentication, standardize API responses, and share common business logic. This approach improves maintainability and promotes consistency across the entire project.