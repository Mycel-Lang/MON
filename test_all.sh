#!/usr/bin/env bash

# Rigorous testing script for mon CLI
# Tests all valid and invalid MON files

set -e
shopt -s nullglob

CARGO_RUN="cargo run --quiet --"
TESTS_DIR="tests"
PASSED=0
FAILED=0

echo "========================================="
echo "  MON CLI - Rigorous Testing Suite"
echo "========================================="
echo ""

# Test valid files (should pass check and compile)
echo "Testing VALID files (tests/ok/)..."
echo "-----------------------------------------"

for file in "$TESTS_DIR"/ok/*.mon; do
    [ -f "$file" ] || continue
    filename=$(basename "$file")
    echo -n "Testing $filename... "
    
    # Test check command
    if $CARGO_RUN check "$file" > /dev/null 2>&1; then
        # Test compile to JSON
        if $CARGO_RUN compile "$file" --to json > /dev/null 2>&1; then
            # Test compile to YAML
            if $CARGO_RUN compile "$file" --to yaml > /dev/null 2>&1; then
                # Test compile to TOML
                if $CARGO_RUN compile "$file" --to toml > /dev/null 2>&1; then
                    echo "✓ PASS"
                    ((PASSED++))
                else
                    echo "✗ FAIL (TOML compilation failed)"
                    ((FAILED++))
                fi
            else
                echo "✗ FAIL (YAML compilation failed)"
                ((FAILED++))
            fi
        else
            echo "✗ FAIL (JSON compilation failed)"
            ((FAILED++))
        fi
    else
        echo "✗ FAIL (check failed)"
        ((FAILED++))
    fi
done

echo ""
echo "Testing INVALID files (tests/bad/)..."
echo "-----------------------------------------"

for file in "$TESTS_DIR"/bad/*.mon; do
    [ -f "$file" ] || continue
    filename=$(basename "$file")
    echo -n "Testing $filename... "
    
    # These should FAIL check (exit code != 0)
    if $CARGO_RUN check "$file" > /dev/null 2>&1; then
        echo "✗ FAIL (should have failed but passed)"
        ((FAILED++))
    else
        echo "✓ PASS (correctly rejected)"
        ((PASSED++))
    fi
done

echo ""
echo "Testing CROSS-FILE imports..."
echo "-----------------------------------------"

# Test cross-file imports
for file in "$TESTS_DIR"/tests/*_main.mon; do
    if [ -f "$file" ]; then
        filename=$(basename "$file")
        echo -n "Testing $filename... "
        
        if $CARGO_RUN check "$file" > /dev/null 2>&1; then
            if $CARGO_RUN compile "$file" --to json > /dev/null 2>&1; then
                echo "✓ PASS"
                ((PASSED++))
            else
                echo "✗ FAIL (compilation failed)"
                ((FAILED++))
            fi
        else
            echo "✗ FAIL (check failed)"
            ((FAILED++))
        fi
    fi
done

echo ""
echo "========================================="
echo "  Test Results"
echo "========================================="
echo "PASSED: $PASSED"
echo "FAILED: $FAILED"
echo "TOTAL:  $((PASSED + FAILED))"
echo ""

if [ $FAILED -eq 0 ]; then
    echo "✓ All tests passed!"
    exit 0
else
    echo "✗ Some tests failed"
    exit 1
fi
