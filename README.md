# PNAR World Backend (Rust)

This is the backend API for PNAR World, implemented in Rust.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Podman](https://podman.io/getting-started/installation) (or Docker)
- PostgreSQL (for local development)

## Development

You can run this project either locally or using containers.

### Local Development

1. Make sure PostgreSQL is running locally with:
   - Database: `pnar_world`
   - Username: `postgres`
   - Password: `root`
   - Port: `5432`

2. Run the application:
   ```bash
   cargo run
   ```

The API will be available at http://localhost:8000

### Container Development

1. Start all services (API, PostgreSQL, and pgAdmin):
   ```bash
   ./start.sh
   ```

   This script will:
   - Build the API image
   - Create a pod with all services
   - Wait for PostgreSQL to be ready
   - Provide access information

2. Access the services:
   - API & Swagger UI: http://localhost:8000/swagger-ui/index.html
   - PgAdmin: http://localhost:9001
     - Email: `admin@pnar.online`
     - Password: `root`
   - PostgreSQL:
     - Host: `localhost`
     - Port: `5432`
     - Database: `pnar_world`
     - Username: `postgres`
     - Password: `root`

### Configuration

The application uses a single `configuration.yaml` file with environment variable overrides:

```yaml
# Local development defaults (no env vars needed)
database:
  host: "127.0.0.1"
  port: 5432
  ...

# Container overrides (set via pod.yaml)
DATABASE_HOST: postgres
APPLICATION_HOST: 0.0.0.0
```

### Troubleshooting

1. **Database Connection Issues**
   - For local development, ensure PostgreSQL is running on localhost
   - For containers, check if the postgres container is running:
     ```bash
     podman ps
     ```

2. **Container Logs**
   ```bash
   # View pod logs
   podman pod logs pw-pod
   ```

3. **Clean Restart**
   ```bash
   # Remove all containers and pods
   podman pod rm -f $(podman pod ls -q)
   
   # Start fresh
   ./start.sh
   ```

4. **API Not Accessible**
   - Local: Check if something else is using port 8000
   - Container: Verify the API container is running and check its logs

## Contributing

1. Create a new branch
2. Make your changes
3. Submit a pull request

## License

[MIT](LICENSE)