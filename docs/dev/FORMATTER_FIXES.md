# Industrial-Grade Formatter Requirements

## Critical Issues to Fix

### 1. Indentation/Brace Balancing

**Problem**: Struct closing brace not properly indented

```mon
// WRONG
User: #struct {
id(Number),
name(String),
},

// CORRECT
User: #struct {
    id(Number),
    name(String),
},
```

### 2. Anchor Syntax

**Problem**: Formatter produces wrong anchor placement

```mon
// WRONG (current)
base_user: &base_user { id: 1, name: "Alice" }

// CORRECT (opinionated)
&base_user: { id: 1, name: "Alice" }
```

**Rationale**: In MON, `&name` defines an anchor. The anchor IS the key, not a value modifier.

### 3. Graceful Error Handling

**Problem**: Crashes on malformed input
**Solution**:

- Catch parse errors gracefully
- Provide helpful error messages
- Suggest running `mon check` first
- Never panic on user input

### 4. Comprehensive Test Coverage

**Missing features in tests**:

- Complex nesting
- All anchor/alias variations
- Spread operators
- Type validations (`::`)
- Enum values
- Import variations
- Comments in various positions
- Edge cases (empty structures, trailing commas, etc.)

## Implementation Plan

### Phase 1: Fix Indentation

- Ensure type definitions use proper depth
- Fix struct/enum brace alignment
- Add indentation tests

### Phase 2: Fix Anchor Handling

- When anchor is present on value, treat it as the key
- Format as `&anchor_name: value`
- Never format as `key: &anchor value`

### Phase 3: Graceful Error Handling

- Wrap parser errors with helpful context
- Suggest fixes for common errors
- Add `--skip-invalid` flag option

### Phase 4: Comprehensive Tests

- Create test files exercising ALL features
- Add fuzzing/property tests
- Test error conditions
- Test round-trip formatting

## Industrial Standards

- Never crash on user input
- Provide actionable error messages
- Consistent, predictable output
- Fast (< 100ms for typical files)
- Idempotent (format(format(x)) == format(x))
- Preserve semantics (parse(format(parse(x))) == parse(x))
