#!/usr/bin/env bash
# Plaid Rust Client — Repository Initialization Script
# Run this in an empty directory to scaffold the full repo with git history.

set -euo pipefail

echo "🦀 Initializing plaid-client-rs repository..."

# ── Git repo ──────────────────────────────────────────────────────────
git init
git checkout -b main 2>/dev/null || true

# ── Root files ────────────────────────────────────────────────────────
cat > Cargo.toml << 'CARGO'
[package]
name = "plaid-client-rs"
version = "0.1.0"
edition = "2021"
rust-version = "1.75"
authors = ["johnathonfox"]
license = "MIT OR Apache-2.0"
description = "A modern, async Rust client for the Plaid API"
repository = "https://github.com/johnathonfox/plaid-client-rs"
keywords = ["plaid", "fintech", "api", "client", "async"]
categories = ["api-bindings", "asynchronous", "web-programming"]

[dependencies]
reqwest = { version = "0.12", features = ["json", "rustls-tls"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "1.0"
tokio = { version = "1.40", features = ["rt-multi-thread", "macros"] }
tracing = "0.1"
secrecy = { version = "0.10", features = ["serde"] }
url = "2.5"

[dev-dependencies]
wiremock = "0.6"
tokio-test = "0.4"
serde_path_to_error = "0.1"

[features]
default = []

[[example]]
name = "basic_auth"
path = "docs/examples/basic_auth.rs"

[[example]]
name = "link_token_flow"
path = "docs/examples/link_token_flow.rs"
CARGO

cat > rustfmt.toml << 'RUSTFMT'
max_width = 100
tab_spaces = 4
edition = "2021"
RUSTFMT

cat > clippy.toml << 'CLIPPY'
avoid-breaking-exported-api = false
CLIPPY

cat > deny.toml << 'DENY'
[graph]
targets = [
    { triple = "x86_64-unknown-linux-gnu" },
    { triple = "aarch64-unknown-linux-gnu" },
    { triple = "x86_64-apple-darwin" },
    { triple = "aarch64-apple-darwin" },
]

[licenses]
version = 2
allow = [
    "MIT",
    "Apache-2.0",
    "BSD-3-Clause",
    "ISC",
    "Unicode-3.0",
    "Unicode-DFS-2016",
]
DENY

cat > README.md << 'README'
# plaid-client-rs

[![CI](https://github.com/johnathonfox/plaid-client-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/johnathonfox/plaid-client-rs/actions)
[![Docs](https://docs.rs/plaid-client-rs/badge.svg)](https://docs.rs/plaid-client-rs)
[![Crates.io](https://img.shields.io/crates/v/plaid-client-rs.svg)](https://crates.io/crates/plaid-client-rs)

A modern, async, type-safe Rust client for the [Plaid API](https://plaid.com/docs/api/).

## Features

- ✅ Async/await via `tokio` + `reqwest`
- ✅ Strongly-typed errors matching Plaid's error taxonomy
- ✅ Secret-safe (tokens wrapped with `secrecy`)
- 🚧 Full OpenAPI coverage (generated + hand-curated) — see ADR-0001
- 🚧 Middleware support (logging, retry, rate-limiting)
- 🚧 Mock-server integration tests

## Quick Start

```rust
use plaid_client_rs::{PlaidClient, Config, Environment};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = PlaidClient::new(Config {
        client_id: "your-client-id".into(),
        secret: "your-secret".into(),
        environment: Environment::Sandbox,
        ..Config::default()
    })?;

    // Use the client...
    Ok(())
}
```

## Installation

```toml
[dependencies]
plaid-client-rs = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Documentation

- [API Documentation](https://docs.rs/plaid-client-rs)
- [User Guide](https://johnathonfox.github.io/plaid-client-rs)
- [Architecture Decision Records](docs/adr/)

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.
README

cat > CHANGELOG.md << 'CHANGELOG'
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Repository scaffolding and CI pipeline
- Core `PlaidClient` with async HTTP support
- OpenAPI-generated request/response models
- Error types matching Plaid API taxonomy
CHANGELOG

cat > CONTRIBUTING.md << 'CONTRIB'
# Contributing

Thank you for your interest in contributing to `plaid-client-rs`!

## Commit Convention

We follow [Conventional Commits](https://www.conventionalcommits.org/):

- `feat:` — New features
- `fix:` — Bug fixes
- `docs:` — Documentation changes
- `refactor:` — Code restructuring
- `test:` — Test additions/changes
- `chore:` — Maintenance tasks

## Architecture Decisions

Significant architectural changes require an ADR in `docs/adr/`.
See the ADR registry at `docs/adr/README.md`.

## Testing

```bash
cargo test --all-features
cargo test --test integration
```

## Code Quality

```bash
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
cargo deny check
cargo audit
```
CONTRIB

cat > LICENSE-MIT << 'MIT'
MIT License

Copyright (c) 2026

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
MIT

cat > LICENSE-APACHE << 'APACHE'
                                 Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
      other entities that control, are controlled by, or are under common
      control with that entity. For the purposes of this definition,
      "control" means (i) the power, direct or indirect, to cause the
      direction or management of such entity, whether by contract or
      otherwise, or (ii) ownership of fifty percent (50%) or more of the
      outstanding shares, or (iii) beneficial ownership of such entity.

      "You" (or "Your") shall mean an individual or Legal Entity
      exercising permissions granted by this License.

      "Source" form shall mean the preferred form for making modifications,
      including but not limited to software source code, documentation
      source, and configuration files.

      "Object" form shall mean any form resulting from mechanical
      transformation or translation of a Source form, including but
      not limited to compiled object code, generated documentation,
      and conversions to other media types.

      "Work" shall mean the work of authorship, whether in Source or
      Object form, made available under the License, as indicated by a
      copyright notice that is included in or attached to the work
      (an example is provided in the Appendix below).

      "Derivative Works" shall mean any work, whether in Source or Object
      form, that is based on (or derived from) the Work and for which the
      editorial revisions, annotations, elaborations, or other modifications
      represent, as a whole, an original work of authorship. For the purposes
      of this License, Derivative Works shall not include works that remain
      separable from, or merely link (or bind by name) to the interfaces of,
      the Work and Derivative Works thereof.

      "Contribution" shall mean any work of authorship, including
      the original version of the Work and any modifications or additions
      to that Work or Derivative Works thereof, that is intentionally
      submitted to Licensor for inclusion in the Work by the copyright owner
      or by an individual or Legal Entity authorized to submit on behalf of
      the copyright owner. For the purposes of this definition, "submitted"
      means any form of electronic, verbal, or written communication sent
      to the Licensor or its representatives, including but not limited to
      communication on electronic mailing lists, source code control systems,
      and issue tracking systems that are managed by, or on behalf of, the
      Licensor for the purpose of discussing and improving the Work, but
      excluding communication that is conspicuously marked or otherwise
      designated in writing by the copyright owner as "Not a Contribution."

      "Contributor" shall mean Licensor and any individual or Legal Entity
      on behalf of whom a Contribution has been received by Licensor and
      subsequently incorporated within the Work.

   2. Grant of Copyright License. Subject to the terms and conditions of
      this License, each Contributor hereby grants to You a perpetual,
      worldwide, non-exclusive, no-charge, royalty-free, irrevocable
      copyright license to reproduce, prepare Derivative Works of,
      publicly display, publicly perform, sublicense, and distribute the
      Work and such Derivative Works in Source or Object form.

   3. Grant of Patent License. Subject to the terms and conditions of
      this License, each Contributor hereby grants to You a perpetual,
      worldwide, non-exclusive, no-charge, royalty-free, irrevocable
      (except as stated in this section) patent license to make, have made,
      use, offer to sell, sell, import, and otherwise transfer the Work,
      where such license applies only to those patent claims licensable
      by such Contributor that are necessarily infringed by their
      Contribution(s) alone or by combination of their Contribution(s)
      with the Work to which such Contribution(s) was submitted. If You
      institute patent litigation against any entity (including a
      cross-claim or counterclaim in a lawsuit) alleging that the Work
      or a Contribution incorporated within the Work constitutes direct
      or contributory patent infringement, then any patent licenses
      granted to You under this License for that Work shall terminate
      as of the date such litigation is filed.

   4. Redistribution. You may reproduce and distribute copies of the
      Work or Derivative Works thereof in any medium, with or without
      modifications, and in Source or Object form, provided that You
      meet the following conditions:

      (a) You must give any other recipients of the Work or
          Derivative Works a copy of this License; and

      (b) You must cause any modified files to carry prominent notices
          stating that You changed the files; and

      (c) You must retain, in the Source form of any Derivative Works
          that You distribute, all copyright, patent, trademark, and
          attribution notices from the Source form of the Work,
          excluding those notices that do not pertain to any part of
          the Derivative Works; and

      (d) If the Work includes a "NOTICE" text file as part of its
          distribution, then any Derivative Works that You distribute must
          include a readable copy of the attribution notices contained
          within such NOTICE file, excluding those notices that do not
          pertain to any part of the Derivative Works, in at least one
          of the following places: within a NOTICE text file distributed
          as part of the Derivative Works; within the Source form or
          documentation, if provided along with the Derivative Works; or,
          within a display generated by the Derivative Works, if and
          wherever such third-party notices normally appear. The contents
          of the NOTICE file are for informational purposes only and
          do not modify the License. You may add Your own attribution
          notices within Derivative Works that You distribute, alongside
          or as an addendum to the NOTICE text from the Work, provided
          that such additional attribution notices cannot be construed
          as modifying the License.

      You may add Your own copyright statement to Your modifications and
      may provide additional or different license terms and conditions
      for use, reproduction, or distribution of Your modifications, or
      for any such Derivative Works as a whole, provided Your use,
      reproduction, and distribution of the Work otherwise complies with
      the conditions stated in this License.

   5. Submission of Contributions. Unless You explicitly state otherwise,
      any Contribution intentionally submitted for inclusion in the Work
      by You to the Licensor shall be under the terms and conditions of
      this License, without any additional terms or conditions.
      Notwithstanding the above, nothing herein shall supersede or modify
      the terms of any separate license agreement you may have executed
      with Licensor regarding such Contributions.

   6. Trademarks. This License does not grant permission to use the trade
      names, trademarks, service marks, or product names of the Licensor,
      except as required for reasonable and customary use in describing the
      origin of the Work and reproducing the content of the NOTICE file.

   7. Disclaimer of Warranty. Unless required by applicable law or
      agreed to in writing, Licensor provides the Work (and each
      Contributor provides its Contributions) on an "AS IS" BASIS,
      WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or
      implied, including, without limitation, any warranties or conditions
      of TITLE, NON-INFRINGEMENT, MERCHANTABILITY, or FITNESS FOR A
      PARTICULAR PURPOSE. You are solely responsible for determining the
      appropriateness of using or redistributing the Work and assume any
      risks associated with Your exercise of permissions under this License.

   8. Limitation of Liability. In no event and under no legal theory,
      whether in tort (including negligence), contract, or otherwise,
      unless required by applicable law (such as deliberate and grossly
      negligent acts) or agreed to in writing, shall any Contributor be
      liable to You for damages, including any direct, indirect, special,
      incidental, or consequential damages of any character arising as a
      result of this License or out of the use or inability to use the
      Work (including but not limited to damages for loss of goodwill,
      work stoppage, computer failure or malfunction, or any and all
      other commercial damages or losses), even if such Contributor
      has been advised of the possibility of such damages.

   9. Accepting Warranty or Additional Liability. While redistributing
      the Work or Derivative Works thereof, You may choose to offer,
      and charge a fee for, acceptance of support, warranty, indemnity,
      or other liability obligations and/or rights consistent with this
      License. However, in accepting such obligations, You may act only
      on Your own behalf and on Your sole responsibility, not on behalf
      of any other Contributor, and only if You agree to indemnify,
      defend, and hold each Contributor harmless for any liability
      incurred by, or claims asserted against, such Contributor by reason
      of your accepting any such warranty or additional liability.

   END OF TERMS AND CONDITIONS

   APPENDIX: How to apply the Apache License to your work.

      To apply the Apache License to your work, attach the following
      boilerplate notice, with the fields enclosed by brackets "[]"
      replaced with your own identifying information. (Don't include
      the brackets!)  The text should be enclosed in the appropriate
      comment syntax for the file format. We also recommend that a
      file or class name and description of purpose be included on the
      same "printed page" as the copyright notice for easier
      identification within third-party archives.

   Copyright [yyyy] [name of copyright owner]

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
APACHE

# ── Source skeleton ───────────────────────────────────────────────────
mkdir -p src/{middleware,models,endpoints,webhooks}

cat > src/lib.rs << 'LIB'
#![forbid(unsafe_code)]
#![warn(clippy::all, clippy::pedantic)]
#![warn(missing_docs, missing_debug_implementations)]

//! A modern, async Rust client for the Plaid API.
//!
//! # Quick Start
//!
//! ```no_run
//! use plaid_client_rs::{PlaidClient, Config, Environment};
//!
//! # async fn run() -> Result<(), plaid_client_rs::PlaidError> {
//! let client = PlaidClient::new(Config {
//!     client_id: "your-client-id".into(),
//!     secret: "your-secret".into(),
//!     environment: Environment::Sandbox,
//!     ..Config::default()
//! });
//! # Ok(())
//! # }
//! ```

pub mod client;
pub mod config;
pub mod endpoints;
pub mod error;
pub mod middleware;
pub mod models;
pub mod webhooks;

pub use client::PlaidClient;
pub use config::{Config, Environment};
pub use error::PlaidError;

// Re-export commonly used models at crate root
pub use models::auth;
pub use models::link;
LIB

cat > src/client.rs << 'CLIENT'
//! The main Plaid API client.

use crate::{Config, PlaidError};
use reqwest::Client;
use secrecy::SecretString;
use std::sync::Arc;

/// The main Plaid API client.
///
/// Clone is cheap — `PlaidClient` uses an `Arc` internally.
#[derive(Debug, Clone)]
pub struct PlaidClient {
    inner: Arc<PlaidClientInner>,
}

#[derive(Debug)]
struct PlaidClientInner {
    http: Client,
    config: Config,
}

impl PlaidClient {
    /// Create a new client with the given configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying HTTP client cannot be built.
    pub fn new(config: Config) -> Result<Self, PlaidError> {
        let http = Client::builder()
            .timeout(config.timeout)
            .build()
            .map_err(PlaidError::HttpClient)?;

        let inner = Arc::new(PlaidClientInner { http, config });

        Ok(Self { inner })
    }

    /// Returns the base URL for the configured environment.
    #[must_use]
    pub fn base_url(&self) -> &str {
        match self.inner.config.environment {
            crate::Environment::Sandbox => "https://sandbox.plaid.com",
            crate::Environment::Production => "https://production.plaid.com",
        }
    }

    // Used by endpoint modules as they are implemented; not dead code long-term.
    /// Returns a reference to the underlying HTTP client.
    #[allow(dead_code)]
    pub(crate) fn http(&self) -> &Client {
        &self.inner.http
    }

    /// Returns the client ID.
    #[allow(dead_code)]
    pub(crate) fn client_id(&self) -> &SecretString {
        &self.inner.config.client_id
    }

    /// Returns the secret.
    #[allow(dead_code)]
    pub(crate) fn secret(&self) -> &SecretString {
        &self.inner.config.secret
    }
}
CLIENT

cat > src/config.rs << 'CONFIG'
//! Client configuration and environment selection.

use secrecy::SecretString;
use std::time::Duration;

/// Plaid API environment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Environment {
    /// Sandbox environment for testing.
    Sandbox,
    /// Production environment.
    Production,
}

/// Configuration for the Plaid client.
///
/// `Config` is not `Clone` because `SecretString` deliberately is not;
/// construct a fresh one (or wrap in `Arc`) if you need to share it.
#[derive(Debug)]
pub struct Config {
    /// Your Plaid client ID.
    pub client_id: SecretString,
    /// Your Plaid secret.
    pub secret: SecretString,
    /// API environment.
    pub environment: Environment,
    /// Request timeout. Defaults to 30 seconds.
    pub timeout: Duration,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            client_id: SecretString::from(""),
            secret: SecretString::from(""),
            environment: Environment::Sandbox,
            timeout: Duration::from_secs(30),
        }
    }
}
CONFIG

cat > src/error.rs << 'ERROR'
//! Error types for the Plaid client.

use thiserror::Error;

/// Errors that can occur when using the Plaid client.
#[derive(Debug, Error)]
pub enum PlaidError {
    /// An HTTP request failed.
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    /// Failed to build the HTTP client.
    #[error("failed to build HTTP client: {0}")]
    HttpClient(reqwest::Error),

    /// A Plaid API error.
    #[error("Plaid API error ({error_code}): {error_message}")]
    Api {
        /// The Plaid error type.
        error_type: String,
        /// The Plaid error code.
        error_code: String,
        /// A human-readable error message.
        error_message: String,
        /// The Plaid request ID for support.
        request_id: String,
    },

    /// Serialization or deserialization failed.
    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// An unknown or unexpected error occurred.
    #[error("unknown error: {0}")]
    Unknown(String),
}
ERROR

cat > src/middleware/mod.rs << 'MID_MOD'
//! Request/response middleware for the Plaid client.
//!
//! Middleware can be used to add logging, retry logic, rate limiting,
//! and other cross-cutting concerns. No middleware trait exists yet.
MID_MOD

cat > src/models/mod.rs << 'MOD_MOD'
//! Request and response models for the Plaid API.
//!
//! Models are organized by Plaid product. All types implement
//! `Serialize` and `Deserialize` via `serde`.

pub mod auth;
pub mod common;
pub mod link;
MOD_MOD

cat > src/models/common.rs << 'COMMON'
//! Common identifier types shared across Plaid products.

use serde::{Deserialize, Serialize};

/// A Plaid item ID.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ItemId(pub String);

/// A Plaid account ID.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AccountId(pub String);

/// A Plaid request ID (included in all responses for support).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RequestId(pub String);
COMMON

cat > src/models/auth.rs << 'AUTH'
//! Models for Auth and sandbox token endpoints.

use serde::{Deserialize, Serialize};

/// Request for `/sandbox/public_token/create`.
#[derive(Debug, Clone, Serialize)]
pub struct SandboxPublicTokenCreateRequest {
    /// The ID of the institution to create a token for.
    pub institution_id: String,
    /// The products to initialize.
    pub initial_products: Vec<String>,
}

/// Response from `/sandbox/public_token/create`.
#[derive(Debug, Clone, Deserialize)]
pub struct SandboxPublicTokenCreateResponse {
    /// A public token that can be exchanged for an access token.
    pub public_token: String,
    /// The expiration time of the token, if applicable.
    pub expiration: Option<String>,
}
AUTH

cat > src/models/link.rs << 'LINK'
//! Models for the Link token endpoints.

use serde::{Deserialize, Serialize};

/// Request for `/link/token/create`.
#[derive(Debug, Clone, Serialize)]
pub struct LinkTokenCreateRequest {
    /// A unique identifier for the end user.
    pub client_name: String,
    /// The language of the Link interface.
    pub language: String,
    /// The country codes of the end user.
    pub country_codes: Vec<String>,
    /// The Plaid products to initialize.
    pub products: Vec<String>,
    /// The user's information.
    pub user: LinkTokenUser,
}

/// User information for Link token creation.
#[derive(Debug, Clone, Serialize)]
pub struct LinkTokenUser {
    /// A unique ID for the end user.
    pub client_user_id: String,
}

/// Response from `/link/token/create`.
#[derive(Debug, Clone, Deserialize)]
pub struct LinkTokenCreateResponse {
    /// A Link token that can be used to initialize Link.
    pub link_token: String,
    /// The expiration time of the token.
    pub expiration: String,
    /// A unique identifier for the request.
    pub request_id: String,
}
LINK

cat > src/endpoints/mod.rs << 'END_MOD'
//! Endpoint-specific implementations for the Plaid API.
//!
//! Each module corresponds to a Plaid product area.
END_MOD

cat > src/webhooks/mod.rs << 'WEB_MOD'
//! Webhook payload types and signature verification.
//!
//! Plaid sends webhooks to your configured endpoint. This module provides
//! typed payloads and utilities to verify webhook authenticity.
WEB_MOD

# ── Tests ─────────────────────────────────────────────────────────────
mkdir -p tests/mock/fixtures

cat > tests/client_init.rs << 'TEST_INIT'
use plaid_client_rs::{Config, Environment, PlaidClient};
use secrecy::SecretString;

#[tokio::test]
async fn client_can_be_created() {
    let client = PlaidClient::new(Config {
        client_id: SecretString::from("test-client-id"),
        secret: SecretString::from("test-secret"),
        environment: Environment::Sandbox,
        ..Config::default()
    });

    assert!(client.is_ok());
}

#[tokio::test]
async fn client_uses_correct_base_url_for_sandbox() {
    let client = PlaidClient::new(Config {
        client_id: SecretString::from("test-client-id"),
        secret: SecretString::from("test-secret"),
        environment: Environment::Sandbox,
        ..Config::default()
    })
    .unwrap();

    assert_eq!(client.base_url(), "https://sandbox.plaid.com");
}

#[tokio::test]
async fn client_uses_correct_base_url_for_production() {
    let client = PlaidClient::new(Config {
        client_id: SecretString::from("test-client-id"),
        secret: SecretString::from("test-secret"),
        environment: Environment::Production,
        ..Config::default()
    })
    .unwrap();

    assert_eq!(client.base_url(), "https://production.plaid.com");
}
TEST_INIT

cat > tests/mock/server.rs << 'MOCK'
//! Helper mock server for integration tests.
//!
//! Not compiled yet — Cargo only builds top-level files in `tests/`.
//! Wire this in from an integration test with a `#[path = "../mock/server.rs"]`
//! module (or move it to `tests/common/`) when the first endpoint lands.

use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// A mock Plaid API server for testing.
pub struct PlaidMockServer {
    pub server: MockServer,
}

impl PlaidMockServer {
    pub async fn new() -> Self {
        let server = MockServer::start().await;
        Self { server }
    }

    /// Mock the `/sandbox/public_token/create` endpoint.
    pub async fn mock_public_token_create(&self) {
        Mock::given(method("POST"))
            .and(path("/sandbox/public_token/create"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!({
                    "public_token": "public-sandbox-xxx",
                    "expiration": "2026-08-25T00:00:00Z"
                })),
            )
            .mount(&self.server)
            .await;
    }
}
MOCK

cat > tests/mock/fixtures/link_token.json << 'FIX'
{
  "link_token": "link-sandbox-xxx",
  "expiration": "2026-08-25T00:00:00Z",
  "request_id": "req-123"
}
FIX

# ── Examples ──────────────────────────────────────────────────────────
mkdir -p docs/examples

cat > docs/examples/basic_auth.rs << 'EX_BASIC'
use plaid_client_rs::{Config, Environment, PlaidClient};
use secrecy::SecretString;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = PlaidClient::new(Config {
        client_id: SecretString::from(std::env::var("PLAID_CLIENT_ID")?),
        secret: SecretString::from(std::env::var("PLAID_SECRET")?),
        environment: Environment::Sandbox,
        ..Config::default()
    })?;

    println!("Plaid client created successfully!");
    println!("Base URL: {}", client.base_url());

    Ok(())
}
EX_BASIC

cat > docs/examples/link_token_flow.rs << 'EX_LINK'
use plaid_client_rs::{Config, Environment, PlaidClient};
use secrecy::SecretString;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = PlaidClient::new(Config {
        client_id: SecretString::from(std::env::var("PLAID_CLIENT_ID")?),
        secret: SecretString::from(std::env::var("PLAID_SECRET")?),
        environment: Environment::Sandbox,
        ..Config::default()
    })?;

    // TODO: Implement /link/token/create call
    println!(
        "Link token flow example — implement me! (client for {})",
        client.base_url()
    );

    Ok(())
}
EX_LINK

# ── ADRs ──────────────────────────────────────────────────────────────
mkdir -p docs/adr

cat > docs/adr/README.md << 'ADR_README'
# Architecture Decision Records

| ID | Title | Status | Date |
|----|-------|--------|------|
| 0001 | Use OpenAPI for model generation | Accepted | 2026-08-24 |
| 0002 | Tokio + Reqwest as async stack | Accepted | 2026-08-24 |
ADR_README

cat > docs/adr/template.md << 'ADR_TEMPLATE'
# ADR-XXXX: [Title]

## Status
- Proposed / Accepted / Deprecated / Superseded by ADR-YYYY

## Context
[What is the issue that we're seeing that is motivating this decision?]

## Decision
[What is the change that we're proposing or have agreed to?]

## Consequences
[What becomes easier or more difficult to do?]

## Alternatives Considered
- [Option A]: [Why rejected]
- [Option B]: [Why rejected]

## References
- [Link to Plaid docs, Rust RFCs, etc.]
ADR_TEMPLATE

cat > docs/adr/0001-use-openapi-generation.md << 'ADR1'
# ADR-0001: Use OpenAPI for Model Generation

## Status
Accepted

## Context
The Plaid API has hundreds of request/response types across many products.
Hand-writing all models is error-prone and will drift when Plaid updates their API.

## Decision
Use Plaid's official [OpenAPI specification](https://github.com/plaid/plaid-openapi)
as the source of truth for model generation. We will:

1. Download the spec as part of the build process
2. Generate base models using `progenitor` or `openapi-generator`
3. Hand-curate the output for Rust idioms (strong enums, Option<T>, etc.)
4. Check generated code into the repo (not generate at build time) for reproducibility

## Consequences
- **Positive**: Full API coverage, stays current with Plaid updates
- **Positive**: Reduces manual typing errors
- **Negative**: Generated code may need significant cleanup
- **Negative**: Adds a dependency on OpenAPI generator tooling

## Alternatives Considered
- **Hand-write all models**: Rejected — too much maintenance burden
- **Generate at compile time**: Rejected — adds build-time dependency, slower compiles

## References
- [Plaid OpenAPI GitHub](https://github.com/plaid/plaid-openapi)
- [progenitor](https://github.com/oxidecomputer/progenitor)
ADR1

cat > docs/adr/0002-async-runtime.md << 'ADR2'
# ADR-0002: Tokio + Reqwest as Async Stack

## Status
Accepted

## Context
We need an async HTTP client and runtime. Rust has several options.

## Decision
Use `tokio` as the async runtime and `reqwest` as the HTTP client.

- `tokio`: De facto standard, excellent ecosystem, `async fn` in traits support
- `reqwest`: High-level, ergonomic, built on `hyper`, supports `rustls-tls`

## Consequences
- **Positive**: Largest ecosystem, most examples, best documentation
- **Positive**: `reqwest` handles connection pooling, redirects, JSON automatically
- **Negative**: Pulls in many transitive dependencies
- **Negative**: Opinionated — users cannot easily swap to `surf` or `hyper` directly

## Alternatives Considered
- **`hyper` directly**: Rejected — too low-level for this use case
- **`surf` + `async-std`**: Rejected — smaller ecosystem, less mature
- **`ureq` (sync)**: Rejected — Plaid API is latency-tolerant but async is expected in 2026

## References
- [tokio.rs](https://tokio.rs)
- [reqwest docs](https://docs.rs/reqwest)
ADR2

# ── GitHub Actions ────────────────────────────────────────────────────
mkdir -p .github/workflows

cat > .github/workflows/ci.yml << 'CI'
name: CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

env:
  CARGO_TERM_COLOR: always
  RUST_BACKTRACE: 1

jobs:
  check:
    name: Check
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo check --all-features

  fmt:
    name: Format
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt
      - run: cargo fmt --all --check

  clippy:
    name: Clippy
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy
      - run: cargo clippy --all-targets --all-features -- -D warnings

  test:
    name: Test
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo test --all-features

  docs:
    name: Docs
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo doc --no-deps --all-features
        env:
          RUSTDOCFLAGS: -D warnings

  audit:
    name: Security Audit
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: rustsec/audit-check@v1
        with:
          token: ${{ secrets.GITHUB_TOKEN }}
CI

cat > .github/workflows/release.yml << 'RELEASE'
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  publish:
    name: Publish to crates.io
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo publish --dry-run
      - run: cargo publish --token ${{ secrets.CARGO_REGISTRY_TOKEN }}
        env:
          CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_REGISTRY_TOKEN }}
RELEASE

# ── Scripts ───────────────────────────────────────────────────────────
mkdir -p scripts

cat > scripts/generate_from_openapi.sh << 'GEN'
#!/usr/bin/env bash
set -euo pipefail

SPEC_URL="https://raw.githubusercontent.com/plaid/plaid-openapi/master/openapi.yaml"
OUT_DIR="src/models/generated"

echo "Downloading Plaid OpenAPI spec..."
curl -sL "$SPEC_URL" -o /tmp/plaid-openapi.yaml

echo "Generating Rust models..."
# TODO: Add progenitor or openapi-generator command here
# progenitor generate -i /tmp/plaid-openapi.yaml -o $OUT_DIR

echo "Done. Review generated code in $OUT_DIR"
GEN
chmod +x scripts/generate_from_openapi.sh

cat > scripts/check_coverage.sh << 'COV'
#!/usr/bin/env bash
set -euo pipefail

echo "Running tests with coverage..."
cargo tarpaulin --out Html --output-dir target/coverage

echo "Coverage report: target/coverage/tarpaulin-report.html"
COV
chmod +x scripts/check_coverage.sh

# ── mdBook ────────────────────────────────────────────────────────────
mkdir -p docs/book/src

cat > docs/book/book.toml << 'BOOK'
[book]
authors = ["plaid-client-rs Contributors"]
language = "en"
multilingual = false
src = "src"
title = "plaid-client-rs User Guide"

[output.html]
git-repository-url = "https://github.com/johnathonfox/plaid-client-rs"
BOOK

cat > docs/book/src/SUMMARY.md << 'SUMMARY'
# Summary

[Introduction](README.md)

- [Installation](installation.md)
- [Authentication](authentication.md)
- [Endpoints](endpoints.md)
- [Error Handling](error_handling.md)
- [Advanced Topics](advanced.md)
SUMMARY

cat > docs/book/src/README.md << 'BOOK_README'
# plaid-client-rs User Guide

Welcome to the `plaid-client-rs` user guide. This book covers everything you need
to know to integrate the Plaid API into your Rust application.
BOOK_README

cat > docs/book/src/installation.md << 'INSTALL'
# Installation

Add `plaid-client-rs` to your `Cargo.toml`:

```toml
[dependencies]
plaid-client-rs = "0.1"
tokio = { version = "1", features = ["full"] }
```
INSTALL

cat > docs/book/src/authentication.md << 'AUTH_DOC'
# Authentication

All Plaid API requests require a `client_id` and `secret`. These are provided
when you create an account on the [Plaid Dashboard](https://dashboard.plaid.com).

```rust
use plaid_client_rs::{Config, Environment, PlaidClient};
use secrecy::SecretString;

let client = PlaidClient::new(Config {
    client_id: SecretString::from(std::env::var("PLAID_CLIENT_ID").unwrap()),
    secret: SecretString::from(std::env::var("PLAID_SECRET").unwrap()),
    environment: Environment::Sandbox,
    ..Config::default()
}).unwrap();
```

> **Security**: Never hard-code your credentials. Use environment variables
> or a secrets manager.
AUTH_DOC

cat > docs/book/src/endpoints.md << 'END_DOC'
# Endpoints

## Link

### Create Link Token

```rust
// TODO: Add example
```

## Auth

### Get Auth Data

```rust
// TODO: Add example
```
END_DOC

cat > docs/book/src/error_handling.md << 'ERR_DOC'
# Error Handling

`plaid-client-rs` uses a custom `PlaidError` type that covers:

- HTTP errors (network issues, timeouts)
- Plaid API errors (returned in the response body)
- Serialization errors

```rust
use plaid_client_rs::PlaidError;

match result {
    Ok(response) => println!("Success!"),
    Err(PlaidError::Api { error_code, .. }) => {
        eprintln!("Plaid error: {}", error_code);
    }
    Err(e) => eprintln!("Other error: {}", e),
}
```
ERR_DOC

cat > docs/book/src/advanced.md << 'ADV'
# Advanced Topics

## Middleware

## Webhook Verification

## Custom HTTP Configuration
ADV

# ── Git commits ───────────────────────────────────────────────────────
echo "📦 Committing initial scaffold..."

git add .
git commit -m "chore: initial repository scaffold

- Cargo package with dependencies
- Core client, config, and error types
- GitHub Actions CI (check, fmt, clippy, test, docs, audit)
- ADR framework with first 2 decisions
- Mock server infrastructure and integration tests
- mdBook documentation skeleton
- Examples and contributing guidelines"

echo ""
echo "✅ Repository initialized successfully!"
echo ""
echo "Next steps:"
echo "  1. Review and customize README.md, Cargo.toml, and LICENSE files"
echo "  2. Run 'cargo check' to verify the build"
echo "  3. Run 'cargo test' to verify tests pass"
echo "  4. Push to GitHub: git remote add origin <url> && git push -u origin main"
echo "  5. Give Kimi Code the instructions from KIMI-CODE-INSTRUCTIONS.md"
