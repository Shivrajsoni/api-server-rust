# Rust API Server

A simple REST API server built with Rust and Actix-web.

## API Endpoints

- `GET /` - Hello world message
- `GET /hey` - Manual hello endpoint
- `GET /items` - Get all items
- `POST /api/items` - Create a new item
- `GET /item/{id}` - Get item by ID
- `PUT /item/{id}` - Update item by ID
- `DELETE /item/{id}` - Delete item by ID

## Local Development

```bash
cargo run
```

The server will start on `http://localhost:8080`

## Deployment to Render

### Option 1: Using render.yaml (Recommended)

1. Push your code to a Git repository (GitHub, GitLab, etc.)
2. Go to [Render Dashboard](https://dashboard.render.com/)
3. Click "New +" and select "Blueprint"
4. Connect your repository
5. Render will automatically detect the `render.yaml` file and deploy your service

### Option 2: Manual Deployment

1. Push your code to a Git repository
2. Go to [Render Dashboard](https://dashboard.render.com/)
3. Click "New +" and select "Web Service"
4. Connect your repository
5. Configure the service:
   - **Name**: `rust-api-server` (or any name you prefer)
   - **Environment**: `Docker`
   - **Region**: Choose closest to your users
   - **Branch**: `main` (or your default branch)
   - **Root Directory**: Leave empty (if your code is in the root)
   - **Dockerfile Path**: `./Dockerfile`
   - **Health Check Path**: `/`

## Environment Variables

- `PORT` - Port number (automatically set by Render)
- `RUST_LOG` - Log level (set to `info` by default)

## Docker Build

The project includes a multi-stage Dockerfile optimized for production deployment:

```bash
docker build -t rust-api-server .
docker run -p 8080:8080 rust-api-server
```
