# httpdump

A fast, headless HTTP server that accepts all requests and dumps them to stdout. Fills the gap left by httpbin (which requires Python and is often overkill) or writing custom Node/Express scripts just to inspect a webhook payload locally.

## Status

**built, untested**: the Axum-based request dumper is built. It has not been heavily tested with varied HTTP payloads in this sandbox environment.

## Installation

```sh
cargo install --path .
```

## Usage

```sh
httpdump --port 8080
```
