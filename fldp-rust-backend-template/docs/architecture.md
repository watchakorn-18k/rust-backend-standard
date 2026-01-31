# Architecture

```mermaid
graph TD
    Client -->|HTTP| Handler
    Handler -->|Call| Service
    Service -->|Call| Repository
    Repository -->|Query| MongoDB
    Service -->|Use| Providers
    Providers -->|External| AWS_S3
    Providers -->|External| Redis
    Providers -->|External| Email_Service
```
