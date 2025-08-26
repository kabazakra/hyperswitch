#!/usr/bin/env bash

# Script to run single Square connector test in single thread
# Usage: ./scripts/run_square_test.sh

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if we're in the right directory
if [[ ! -f "Cargo.toml" ]]; then
    print_error "This script must be run from the hyperswitch root directory"
    exit 1
fi

# Check if auth.toml exists
if [[ ! -f "crates/router/tests/connectors/auth.toml" ]]; then
    print_error "auth.toml file not found at crates/router/tests/connectors/auth.toml"
    print_info "Please ensure you have created the auth.toml file with your Square credentials"
    exit 1
fi

# Check if Square test file exists
if [[ ! -f "crates/router/tests/connectors/square.rs" ]]; then
    print_error "Square test file not found at crates/router/tests/connectors/square.rs"
    exit 1
fi

print_info "Running Square connector test: should_only_authorize_payment"
print_info "Using auth.toml for configuration"
print_info "Running in single thread mode"

# Set environment variables
export CONNECTOR_AUTH_FILE_PATH="$(pwd)/crates/router/tests/connectors/auth.toml"
export RUST_TEST_THREADS=1

print_info "Environment variables set:"
print_info "  CONNECTOR_AUTH_FILE_PATH: $CONNECTOR_AUTH_FILE_PATH"
print_info "  RUST_TEST_THREADS: $RUST_TEST_THREADS"

# Run the specific test
print_info "Executing test..."
cargo test --package router --test connectors square::should_only_authorize_payment -- --nocapture --test-threads=1

if [[ $? -eq 0 ]]; then
    print_success "Square connector test completed successfully!"
else
    print_error "Square connector test failed!"
    exit 1
fi

