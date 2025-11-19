#!/usr/bin/env bash
set -e

# This script runs Playwright E2E tests in a Docker container or directly based on the LEPTOS_ENV environment variable.
# usage of TEST_FILTER: pass a filter to run specific tests, e.g.
# running a single test file with:
#   TEST_FILTER="tests/setup-id-input.spec.ts" cargo leptos end-to-end
# running a specific test name with grep:
#   TEST_FILTER='-g "Valid input"' cargo leptos end-to-end

ENGINE=${CONTAINER_ENGINE:-docker}
ENVIRONMENT=${LEPTOS_ENV:-DEV}
TEST_DIR="$(pwd)"
IMAGE="mcr.microsoft.com/playwright:v1.44.1-jammy"

echo "Detected LEPTOS_ENV=$ENVIRONMENT"

if [[ "$ENVIRONMENT" == "DEV" ]]; then
  echo "🔧 Running Playwright E2E tests via Docker (local DEV mode)..."
  $ENGINE run --rm -it \
    --network=host \
    -v "$TEST_DIR":/app \
    -w /app \
    "$IMAGE" \
    bash -c "npx playwright test ${TEST_FILTER:-}"
elif [[ "$ENVIRONMENT" == "PROD" ]]; then
  echo "⚙️  Running Playwright E2E tests directly (CI mode)..."
  npx playwright test --reporter=html || exit 1
  echo "✅ Playwright tests finished (CI)"
else
  echo "❗️Unknown LEPTOS_ENV value: $ENVIRONMENT"
  exit 1
fi
