# Stage 1: Frontend Build
FROM oven/bun:latest AS frontend-build
WORKDIR /app/frontend
COPY frontend/package*.json ./
RUN bun install
COPY frontend/ ./
RUN bun run build

# Stage 2: Backend Build
FROM golang:1.23 AS backend-build
WORKDIR /app
COPY go.mod go.sum ./
RUN go mod download
COPY ./backend ./backend
RUN CGO_ENABLED=0 GOOS=linux go build -o /ascii-converter ./backend/server/main.go

# Stage 3: Final runtime image
FROM debian:bullseye-slim
WORKDIR /app
COPY --from=backend-build /ascii-converter .
COPY --from=frontend-build /app/frontend/dist ./build/frontend
EXPOSE 8080
CMD ["./ascii-converter"]
