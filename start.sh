#!/bin/bash

# Exit on any error
set -e

echo "��� Starting PNAR World API setup..."

# Remove any existing pod
echo "��� Cleaning up any existing pods..."
podman pod rm -f pw-pod 2>/dev/null || true

# Build the API image
echo "��� Building API image..."
podman build -t pnar-world-api:1.0.0 .

# Start the pod with all containers
echo "��� Starting pod with all containers..."
podman play kube pod.yaml

# Wait for PostgreSQL to be ready
echo "⏳ Waiting for PostgreSQL to be ready..."
until podman exec pw-pod-postgres pg_isready -h 127.0.0.1 -U postgres; do
    echo "PostgreSQL is not ready yet... waiting"
    sleep 2
done

echo "✨ Setup complete! You can now access:"
echo "��� API & Swagger UI: http://localhost:8000/swagger-ui/index.html"
echo "��� PgAdmin: http://localhost:9001"
echo "  - Email: admin@pnar.online"
echo "  - Password: root"
echo ""
echo "PostgreSQL connection details:"
echo "��� Host: localhost"
echo "��� Port: 5432"
echo "��� Database: pnar_world"
echo "��� Username: postgres"
echo "��� Password: root"
