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

---

## Production Deployment (manage.pnar.online)

To deploy the API service to your VPS and expose it at `manage.pnar.online`:

### 1. Prepare your VPS

- Install Podman (or Docker)
- Open firewall ports 80 (HTTP), 443 (HTTPS), 8000 (API), 9001 (pgAdmin), 5432 (Postgres) as needed

### 2. Clone your project

```bash
git clone https://github.com/armego/pnar-world-backend-rust.git
cd pnar-world-backend-rust
```

### 3. Update configuration for production

Edit `configuration.yaml`:

```yaml
application:
  host: "0.0.0.0"
  port: 8000
  base_url: "https://manage.pnar.online"
  cors:
    allowed_origins: ["https://manage.pnar.online"]
    ...existing code...

database:
  host: "postgres"
  ...existing code...

jwt:
  secret: "<your-secure-production-secret>"
  cookie_domain: "manage.pnar.online"
  cookie_secure: true
  ...existing code...
```

### 4. Update pod.yaml for production

Set environment variables for the API container:

```yaml
env:
  - name: DATABASE_HOST
    value: postgres
  - name: APPLICATION_HOST
    value: 0.0.0.0
  - name: RUST_LOG
    value: info
```

### 5. Build and start the services

```bash
podman build -t pnar-world-api:1.0.0 .
podman play kube pod.yaml
```

### 6. Set up HTTPS (recommended)

- Use a reverse proxy (Nginx, Caddy, or Traefik) to:
  - Forward requests from `manage.pnar.online` to the API container (port 8000)
  - Terminate SSL (Let's Encrypt recommended)
- Example Nginx config:

```nginx
server {
    listen 80;
    server_name manage.pnar.online;
    return 301 https://$host$request_uri;
}

server {
    listen 443 ssl;
    server_name manage.pnar.online;

    ssl_certificate /etc/letsencrypt/live/manage.pnar.online/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/manage.pnar.online/privkey.pem;

    location / {
        proxy_pass http://localhost:8000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

### 7. Access your services

- API & Swagger UI: https://manage.pnar.online/swagger-ui/index.html
- PgAdmin: http://<your-vps-ip>:9001
- PostgreSQL: as configured

### 8. Security recommendations

- Change all default passwords
- Restrict access to pgAdmin and Postgres (firewall, VPN, or local only)
- Keep your VPS updated
- Use HTTPS for all API traffic

---
