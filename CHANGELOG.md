# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Repository scaffolding and CI pipeline
- Core `PlaidClient` with async HTTP support
- Request pipeline: JSON POST with `client_id`/`secret` body injection and Plaid error-body mapping to `PlaidError::Api`
- Optional `Config::base_url` override for pointing the client at a mock server
- `sandbox_public_token_create` endpoint (`/sandbox/public_token/create`)
- Mock-server integration tests for the sandbox public token flow
- OpenAPI-generated request/response models
- Error types matching Plaid API taxonomy
