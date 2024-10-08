# Variables
BINARY_NAME=ascii-converter
BUILD_DIR=./build
SRC_DIR=./backend/server
FRONTEND_DIR=./frontend
FRONTEND_BUILD_DIR=$(FRONTEND_DIR)/dist

all: run
	
build: clean deps fmt
	@echo "Building frontend..."
	@cd $(FRONTEND_DIR) && bun i && bun run build

	@echo "Creating build directory..."
	@mkdir -p $(BUILD_DIR)

	@echo "Copying frontend build files..."
	@cp -r $(FRONTEND_BUILD_DIR) $(BUILD_DIR)/frontend

	@echo "Building Go project..."
	@go build -o $(BUILD_DIR)/$(BINARY_NAME) $(SRC_DIR)/main.go

	@echo "Build complete"

run: build
	@echo "Running the project..."
	@$(BUILD_DIR)/$(BINARY_NAME)

clean:
	@echo "Cleaning build directories..."
	@rm -rf $(BUILD_DIR)
	@rm -rf $(FRONTEND_BUILD_DIR)

fmt:
	@echo "Formatting Go code..."
	@go fmt ./...

deps:
	@echo "Updating dependencies..."
	@go mod tidy

.PHONY: all build run clean fmt deps
