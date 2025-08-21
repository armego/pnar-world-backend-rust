# PNAR World Backend (Rust)

This is the backend API for PNAR World, implemented in Rust.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Podman](https://podman.io/getting-started/installation) or Docker
- PostgreSQL client (optional)

## Quick Start

1. Clone the repository:
```bash
git clone https://github.com/armego/pnar-world-backend-rust.git
cd pnar-world-backend-rust
```

2. Run the automated setup script:

   On Linux/macOS:
   ```bash
   chmod +x start-pod.sh
   ./start-pod.sh
   ```

   On Windows (Git Bash or similar):
   ```bash
   bash start-pod.sh
   ```

   On Windows (Command Prompt):
   ```cmd
   bash.exe start-pod.sh
   ```

   The script will:
   - Check for required dependencies
   - Clean up any existing resources
   - Build the API image
   - Create necessary volumes
   - Deploy all services
   - Set up the database
   - Provide connection information

   If anything fails, the script will provide helpful error messages and instructions.

3. Alternatively, you can manually build and run the containers:
   ```bash
   # Build the API image
   podman build -t localhost/pnar-world-api:1.0.0 .

   # Start the pod with all services
   podman play kube pod.yaml
   ```

## Accessing Services

After starting the pod, you can access the following services:

- **API**: [http://localhost:8000](http://localhost:8000)
  - Swagger UI: [http://localhost:8000/swagger-ui/](http://localhost:8000/swagger-ui/)
  - OpenAPI Spec: [http://localhost:8000/api-docs/openapi.json](http://localhost:8000/api-docs/openapi.json)
  - Health Check: [http://localhost:8000/api/v1/actuator/health](http://localhost:8000/api/v1/actuator/health)

- **PgAdmin**: [http://localhost:9001](http://localhost:9001)
  - Email: `admin@pnar.online`
  - Password: `root`

- **PostgreSQL**:
  - Host: `localhost`
  - Port: `5432`
  - Database: `pnar_world`
  - Username: `postgres`
  - Password: `root`

## Development

To connect to PostgreSQL using psql:
```bash
psql -h localhost -U postgres -d pnar_world
```

To check container status:
```bash
podman ps
```

To view container logs:
```bash
# API logs
podman logs -f pw-pod-pnar-world-api-podified

# Database logs
podman logs -f pw-pod-postgres-podified

# PgAdmin logs
podman logs -f pw-pod-pgadmin-podified
```

To stop all services:
```bash
podman pod rm -f pw-pod
```

## Troubleshooting

1. **Script fails to start**
   - Ensure Podman is installed and in your PATH
   - On Windows, make sure you have Git Bash or similar bash shell installed
   - Check if ports 8000, 9001, and 5432 are available

2. **Database initialization fails**
   - Check if the migrations directory exists and contains .sql files
   - Verify the SQL files are valid PostgreSQL syntax
   - Check container logs: `podman logs pw-pod-postgres-podified`

3. **Can't connect to services**
   - API: Check `podman logs pw-pod-api-podified`
   - Database: Ensure PostgreSQL is ready with `podman logs pw-pod-postgres-podified`
   - PgAdmin: Verify logs with `podman logs pw-pod-pgadmin-podified`

4. **Need to start fresh**
   ```bash
   # Remove all related resources
   podman pod rm -f pw-pod
   podman volume rm -f pg-data migrations
   podman rmi localhost/pnar-world-api:1.0.0
   ```

5. **Windows-specific issues**
   - If using WSL, ensure Podman is properly configured
   - Path issues: Use forward slashes (/) or properly escaped backslashes (\\)
   - Line endings: Ensure scripts use LF (Unix) line endings

For more detailed logs and debugging:
```bash
# Run script with debug output
bash -x start-pod.sh

# Check pod status
podman pod ps

# View all container logs
podman pod logs pw-pod
```