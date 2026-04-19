# LogStream: High-Performance Log Aggregator Makefile

.PHONY: help build infra-up infra-down ingest api frontend verify audit-test clean setup

help:
	@echo "Usage: make [target]"
	@echo ""
	@echo "Development Targets:"
	@echo "  setup         Install prerequisites (Trunk, SQLx-CLI, targets)"
	@echo "  infra-up      Start PostgreSQL and OTel Collector (Docker)"
	@echo "  infra-down    Stop all infrastructure containers"
	@echo "  ingest        Start Ingest Service (Port 8081)"
	@echo "  api           Start Web API & Discovery Service (Port 8080)"
	@echo "  frontend      Start WASM Frontend (Trunk server on Port 3000)"
	@echo ""
	@echo "Security & Verification:"
	@echo "  verify        Run cryptographic audit log verification"
	@echo "  audit-test    Run the security tampering simulation example"
	@echo "  test          Run all backend unit tests"
	@echo ""
	@echo "Build & Cleanup:"
	@echo "  build         Build both backend and frontend"
	@echo "  clean         Remove build artifacts"

setup:
	rustup target add wasm32-unknown-unknown
	cargo install trunk
	cargo install sqlx-cli --no-default-features --features postgres

infra-up:
	docker-compose up -d

infra-down:
	docker-compose down

ingest:
	cd backend && cargo run --bin ingest_service

api:
	cd backend && cargo run --bin api_service

frontend:
	cd frontend && trunk serve

verify:
	cd backend && cargo run --bin verify_audit

audit-test:
	cargo run --example validate_integrity

test:
	cd backend && cargo test

build:
	cd backend && cargo build
	cd frontend && trunk build

clean:
	cd backend && cargo clean
	cd frontend && cargo clean
