use crate::errors::MonCliError;
use miette::Result;
use mon_core::api::analyze;
use serde_json::Value as JsonValue;
use std::fs;
use std::path::Path;

pub fn run(
    file: String,
    to: String,
    toml_null_value: Option<String>,
    output_dir: Option<String>,
    generate_docs: bool,
) -> Result<()> {
    let content =
        fs::read_to_string(&file).map_err(|e| miette::miette!("Failed to read file: {}", e))?;

    let result = analyze(&content, &file).map_err(MonCliError::from_mon_error)?;

    // If output_dir is specified, do comprehensive export
    if let Some(dir) = output_dir {
        return comprehensive_export(&result, &file, &dir, toml_null_value, generate_docs);
    }

    // Otherwise, standard single-format output
    let output = match to.as_str() {
        "json" => {
            result.to_json().map_err(|e| miette::miette!("Failed to serialize to JSON: {}", e))?
        }
        "yaml" => {
            result.to_yaml().map_err(|e| miette::miette!("Failed to serialize to YAML: {}", e))?
        }
        "toml" => {
            let mut value = result.to_value();

            // Replace null values if requested
            if let Some(replacement) = toml_null_value {
                replace_nulls(&mut value, &replacement);
            } else {
                // Remove null values for TOML compatibility
                remove_nulls(&mut value);
            }

            toml::to_string_pretty(&value)
                .map_err(|e| miette::miette!("Failed to serialize to TOML: {}", e))?
        }
        "json-schema" => {
            // Generate JSON schema from the structure
            let json_str = result
                .to_json()
                .map_err(|e| miette::miette!("Failed to serialize to JSON: {}", e))?;
            let json_value: JsonValue = serde_json::from_str(&json_str)
                .map_err(|e| miette::miette!("Failed to parse JSON: {}", e))?;

            generate_json_schema(&json_value)?
        }
        _ => {
            return Err(miette::miette!(
                "Unsupported format: {}. Supported formats: json, yaml, toml, json-schema",
                to
            ));
        }
    };

    println!("{}", output);

    Ok(())
}

fn comprehensive_export(
    result: &mon_core::api::AnalysisResult,
    input_file: &str,
    output_dir: &str,
    toml_null_value: Option<String>,
    generate_docs: bool,
) -> Result<()> {
    use std::fs;
    use std::path::PathBuf;

    // Create organized directory structure
    let json_dir = PathBuf::from(output_dir).join("json");
    let yaml_dir = PathBuf::from(output_dir).join("yaml");
    let toml_dir = PathBuf::from(output_dir).join("toml");

    fs::create_dir_all(&json_dir)
        .map_err(|e| miette::miette!("Failed to create json directory: {}", e))?;
    fs::create_dir_all(&yaml_dir)
        .map_err(|e| miette::miette!("Failed to create yaml directory: {}", e))?;
    fs::create_dir_all(&toml_dir)
        .map_err(|e| miette::miette!("Failed to create toml directory: {}", e))?;

    let base_name = Path::new(input_file).file_stem().and_then(|s| s.to_str()).unwrap_or("output");

    // 1. Generate JSON data
    let json_data =
        result.to_json().map_err(|e| miette::miette!("Failed to serialize to JSON: {}", e))?;
    let json_value: JsonValue = serde_json::from_str(&json_data)
        .map_err(|e| miette::miette!("Failed to parse JSON: {}", e))?;

    let json_path = json_dir.join(format!("{}.json", base_name));
    fs::write(&json_path, &json_data)
        .map_err(|e| miette::miette!("Failed to write JSON file: {}", e))?;
    println!("✓ Generated: {}", json_path.display());

    // 2. Generate JSON Schema
    let schema = generate_json_schema(&json_value)?;
    let schema_path = json_dir.join(format!("{}.schema.json", base_name));
    fs::write(&schema_path, &schema)
        .map_err(|e| miette::miette!("Failed to write schema file: {}", e))?;
    println!("✓ Generated: {}", schema_path.display());

    // 3. Generate YAML
    let yaml_data =
        result.to_yaml().map_err(|e| miette::miette!("Failed to serialize to YAML: {}", e))?;
    let yaml_path = yaml_dir.join(format!("{}.yaml", base_name));
    fs::write(&yaml_path, &yaml_data)
        .map_err(|e| miette::miette!("Failed to write YAML file: {}", e))?;
    println!("✓ Generated: {}", yaml_path.display());

    // 4. Generate TOML (with smart null handling)
    let mut toml_value = result.to_value();
    let null_count = count_nulls(&toml_value);

    if null_count > 0 {
        if let Some(replacement) = toml_null_value {
            replace_nulls(&mut toml_value, &replacement);
            match toml::to_string_pretty(&toml_value) {
                Ok(toml_data) => {
                    let toml_path = toml_dir.join(format!("{}.toml", base_name));
                    fs::write(&toml_path, &toml_data)
                        .map_err(|e| miette::miette!("Failed to write TOML file: {}", e))?;
                    println!("✓ Generated: {}", toml_path.display());
                }
                Err(e) => {
                    println!("⚠ TOML generation failed: {}", e);
                }
            }
        } else {
            println!(
                "⚠ TOML skipped: {} null value(s) detected. Use --toml-null-value \"replacement\" to generate TOML.",
                null_count
            );
        }
    } else {
        // No nulls, safe to generate TOML
        match toml::to_string_pretty(&toml_value) {
            Ok(toml_data) => {
                let toml_path = toml_dir.join(format!("{}.toml", base_name));
                fs::write(&toml_path, &toml_data)
                    .map_err(|e| miette::miette!("Failed to write TOML file: {}", e))?;
                println!("✓ Generated: {}", toml_path.display());
            }
            Err(e) => {
                println!("⚠ TOML generation failed: {}", e);
            }
        }
    }

    // 5. Generate documentation if requested
    if generate_docs {
        let docs_dir = PathBuf::from(output_dir).join("docs");
        fs::create_dir_all(&docs_dir)
            .map_err(|e| miette::miette!("Failed to create docs directory: {}", e))?;

        generate_documentation(&json_value, &docs_dir, base_name, input_file, null_count)?;
    }

    // 6. Create README
    let readme_content = create_readme(base_name, input_file, generate_docs, null_count);
    let readme_path = PathBuf::from(output_dir).join("README.md");
    fs::write(&readme_path, readme_content)
        .map_err(|e| miette::miette!("Failed to write README: {}", e))?;
    println!("✓ Generated: {}", readme_path.display());

    println!("Comprehensive export complete! Organized structure in: {}", output_dir);

    Ok(())
}

fn count_nulls(value: &mon_core::serialization::Value) -> usize {
    use mon_core::serialization::Value;

    match value {
        Value::Null => 1,
        Value::Object(map) => map.values().map(count_nulls).sum(),
        Value::Array(arr) => arr.iter().map(count_nulls).sum(),
        _ => 0,
    }
}

fn replace_nulls(value: &mut mon_core::serialization::Value, replacement: &str) {
    use mon_core::serialization::Value;

    match value {
        Value::Object(map) => {
            for (_, v) in map.iter_mut() {
                if matches!(v, Value::Null) {
                    *v = Value::String(replacement.to_string());
                } else {
                    replace_nulls(v, replacement);
                }
            }
        }
        Value::Array(arr) => {
            for item in arr.iter_mut() {
                if matches!(item, Value::Null) {
                    *item = Value::String(replacement.to_string());
                } else {
                    replace_nulls(item, replacement);
                }
            }
        }
        _ => {}
    }
}

fn remove_nulls(value: &mut mon_core::serialization::Value) {
    use mon_core::serialization::Value;

    match value {
        Value::Object(map) => {
            // Remove null entries
            let keys_to_remove: Vec<_> = map
                .iter()
                .filter(|(_, v)| matches!(v, Value::Null))
                .map(|(k, _)| k.clone())
                .collect();

            for key in keys_to_remove {
                map.remove(&key);
            }

            // Recursively process remaining values
            for (_, v) in map.iter_mut() {
                remove_nulls(v);
            }
        }
        Value::Array(arr) => {
            for item in arr.iter_mut() {
                remove_nulls(item);
            }
        }
        _ => {}
    }
}

fn generate_json_schema(value: &JsonValue) -> Result<String> {
    let schema = infer_schema(value, "Root");
    serde_json::to_string_pretty(&schema)
        .map_err(|e| miette::miette!("Failed to generate JSON schema: {}", e))
}

fn infer_schema(value: &JsonValue, _name: &str) -> JsonValue {
    use serde_json::json;

    match value {
        JsonValue::Null => json!({ "type": "null" }),
        JsonValue::Bool(_) => json!({ "type": "boolean" }),
        JsonValue::Number(n) => {
            if n.is_f64() {
                json!({ "type": "number" })
            } else {
                json!({ "type": "integer" })
            }
        }
        JsonValue::String(_) => json!({ "type": "string" }),
        JsonValue::Array(arr) => {
            if arr.is_empty() {
                json!({ "type": "array", "items": {} })
            } else {
                // Infer from first element
                let items_schema = infer_schema(&arr[0], "item");
                json!({ "type": "array", "items": items_schema })
            }
        }
        JsonValue::Object(obj) => {
            let mut properties = serde_json::Map::new();
            let mut required = Vec::new();

            for (key, val) in obj.iter() {
                properties.insert(key.clone(), infer_schema(val, key));
                if !matches!(val, JsonValue::Null) {
                    required.push(key.clone());
                }
            }

            json!({
                "type": "object",
                "properties": properties,
                "required": required
            })
        }
    }
}

fn create_readme(base_name: &str, input_file: &str, has_docs: bool, null_count: usize) -> String {
    let mut content = format!(
        "# MON Export: {}\n\n\
        Generated from: `{}`\n\n\
        ## Directory Structure\n\n\
        ```\n\
        ├── json/\n\
        │   ├── {}.json          # JSON data\n\
        │  └── {}.schema.json   # JSON Schema for validation\n\
        ├── yaml/\n\
        │   └── {}.yaml          # YAML data\n\
        ├── toml/\n\
        │   └── {}.toml          # TOML data{}\n",
        base_name,
        input_file,
        base_name,
        base_name,
        base_name,
        base_name,
        if null_count > 0 { " (if nulls were replaced)" } else { "" }
    );

    if has_docs {
        content.push_str(&"└── docs/\n\
            ├── structure.md     # Data structure documentation\n\
            └── stats.json       # Statistics and metadata\n\
            ```\n\n".to_string());
    } else {
        content.push_str("```\n\n");
    }

    content.push_str(&format!(
        "## Usage\n\n\
        ### Validation\n\n\
        The JSON schema can be used for validation:\n\n\
        **JavaScript (ajv)**:\n\
        ```javascript\n\
        const Ajv = require('ajv');\n\
        const schema = require('./json/{}.schema.json');\n\
        const data = require('./json/{}.json');\n\
        \n\
        const ajv = new Ajv();\n\
        const valid = ajv.compile(schema)(data);\n\
        ```\n\n\
        **Python (jsonschema)**:\n\
        ```python\n\
        import json\n\
        from jsonschema import validate\n\
        \n\
        with open('json/{}.schema.json') as f:\n\
            schema = json.load(f)\n\
        with open('json/{}.json') as f:\n\
            data = json.load(f)\n\
        validate(instance=data, schema=schema)\n\
        ```\n\n",
        base_name, base_name, base_name, base_name
    ));

    if null_count > 0 {
        content.push_str(&format!(
            "## TOML Note\n\n\
            ⚠ {} null value(s) detected. TOML was {}\n\n\
            To generate TOML with null replacement:\n\
            ```bash\n\
            mon compile {} --output-dir ./output --toml-null-value \"N/A\"\n\
            ```\n\n",
            null_count,
            if null_count > 0 { "skipped (incompatible)" } else { "generated successfully" },
            input_file
        ));
    }

    if has_docs {
        content.push_str(
            "## Documentation\n\n\
            Comprehensive documentation generated in `docs/`:\n\
            - `structure.md` - Data structure, types, and relationships\n\
            - `stats.json` - Metadata and statistics\n\n",
        );
    }

    content.push_str("---\n\n*Maximum compatibility achieved!* 🎉\n");

    content
}

fn generate_documentation(
    value: &JsonValue,
    docs_dir: &std::path::PathBuf,
    base_name: &str,
    input_file: &str,
    null_count: usize,
) -> Result<()> {
    use std::fs;

    // Generate structure documentation
    let structure_md = generate_structure_doc(value, base_name, input_file);
    let structure_path = docs_dir.join("structure.md");
    fs::write(&structure_path, structure_md)
        .map_err(|e| miette::miette!("Failed to write structure.md: {}", e))?;
    println!("✓ Generated: {}", structure_path.display());

    // Generate statistics
    let stats = collect_statistics(value, null_count);
    let stats_json = serde_json::to_string_pretty(&stats)
        .map_err(|e| miette::miette!("Failed to serialize stats: {}", e))?;
    let stats_path = docs_dir.join("stats.json");
    fs::write(&stats_path, stats_json)
        .map_err(|e| miette::miette!("Failed to write stats.json: {}", e))?;
    println!("✓ Generated: {}", stats_path.display());

    Ok(())
}

fn generate_structure_doc(value: &JsonValue, base_name: &str, input_file: &str) -> String {
    let mut doc = format!(
        "# Data Structure Documentation\n\n\
        **Source**: `{}`  \n\
        **Generated**: `{}`\n\n\
        ## Overview\n\n\
        This document provides an algorithmic breakdown of the data structure.\n\n",
        input_file, base_name
    );

    doc.push_str("## Type Hierarchy\n\n");
    doc.push_str(&analyze_structure(value, 0));

    doc.push_str("\n## Field Reference\n\n");
    doc.push_str(&list_all_fields(value, ""));

    doc
}

fn analyze_structure(value: &JsonValue, depth: usize) -> String {
    let indent = "  ".repeat(depth);

    match value {
        JsonValue::Object(obj) => {
            let mut result = format!("{}├─ Object ({} fields)\n", indent, obj.len());
            for (key, val) in obj.iter() {
                result.push_str(&format!("{}  ├─ `{}`", indent, key));
                let type_info = match val {
                    JsonValue::Null => " [null]",
                    JsonValue::Bool(_) => " [boolean]",
                    JsonValue::Number(_) => " [number]",
                    JsonValue::String(_) => " [string]",
                    JsonValue::Array(arr) => {
                        result.push_str(&format!(" [array, {} items]\n", arr.len()));
                        if !arr.is_empty() {
                            result.push_str(&analyze_structure(&arr[0], depth + 2));
                        }
                        continue;
                    }
                    JsonValue::Object(_) => {
                        result.push('\n');
                        result.push_str(&analyze_structure(val, depth + 2));
                        continue;
                    }
                };
                result.push_str(type_info);
                result.push('\n');
            }
            result
        }
        _ => format!("{}└─ {}\n", indent, type_name(value)),
    }
}

fn list_all_fields(value: &JsonValue, prefix: &str) -> String {
    let mut result = String::new();

    if let JsonValue::Object(obj) = value {
        for (key, val) in obj.iter() {
            let full_path =
                if prefix.is_empty() { key.clone() } else { format!("{}.{}", prefix, key) };

            let type_str = match val {
                JsonValue::Null => "null (optional)",
                JsonValue::Bool(_) => "boolean",
                JsonValue::Number(_) => "number",
                JsonValue::String(_) => "string",
                JsonValue::Array(_) => "array",
                JsonValue::Object(_) => "object",
            };

            result.push_str(&format!("- `{}`: {}\n", full_path, type_str));

            if let JsonValue::Object(_) = val {
                result.push_str(&list_all_fields(val, &full_path));
            }
        }
    }

    result
}

fn type_name(value: &JsonValue) -> &str {
    match value {
        JsonValue::Null => "null",
        JsonValue::Bool(_) => "boolean",
        JsonValue::Number(_) => "number",
        JsonValue::String(_) => "string",
        JsonValue::Array(_) => "array",
        JsonValue::Object(_) => "object",
    }
}

fn collect_statistics(value: &JsonValue, null_count: usize) -> serde_json::Map<String, JsonValue> {
    use serde_json::json;

    let mut stats = serde_json::Map::new();

    let (total_fields, max_depth, type_counts) = analyze_value(value, 0);

    stats.insert("total_fields".to_string(), json!(total_fields));
    stats.insert("max_nesting_depth".to_string(), json!(max_depth));
    stats.insert("null_values_count".to_string(), json!(null_count));
    stats.insert("type_distribution".to_string(), json!(type_counts));

    stats
}

fn analyze_value(
    value: &JsonValue,
    depth: usize,
) -> (usize, usize, serde_json::Map<String, JsonValue>) {
    use serde_json::json;

    let mut total = 0;
    let mut max_depth = depth;
    let mut types = serde_json::Map::new();

    // Initialize type counters
    for t in &["null", "boolean", "number", "string", "array", "object"] {
        types.insert(t.to_string(), json!(0));
    }

    match value {
        JsonValue::Object(obj) => {
            total += obj.len();
            *types.get_mut("object").unwrap() =
                json!(types.get("object").unwrap().as_u64().unwrap() + 1);

            for val in obj.values() {
                let (child_fields, child_depth, child_types) = analyze_value(val, depth + 1);
                total += child_fields;
                max_depth = max_depth.max(child_depth);

                for (k, v) in child_types {
                    let current = types.get(&k).unwrap().as_u64().unwrap();
                    let add = v.as_u64().unwrap();
                    types.insert(k, json!(current + add));
                }
            }
        }
        JsonValue::Array(arr) => {
            *types.get_mut("array").unwrap() =
                json!(types.get("array").unwrap().as_u64().unwrap() + 1);

            for val in arr {
                let (child_fields, child_depth, child_types) = analyze_value(val, depth + 1);
                total += child_fields;
                max_depth = max_depth.max(child_depth);

                for (k, v) in child_types {
                    let current = types.get(&k).unwrap().as_u64().unwrap();
                    let add = v.as_u64().unwrap();
                    types.insert(k, json!(current + add));
                }
            }
        }
        JsonValue::Null => {
            *types.get_mut("null").unwrap() =
                json!(types.get("null").unwrap().as_u64().unwrap() + 1);
        }
        JsonValue::Bool(_) => {
            *types.get_mut("boolean").unwrap() =
                json!(types.get("boolean").unwrap().as_u64().unwrap() + 1);
        }
        JsonValue::Number(_) => {
            *types.get_mut("number").unwrap() =
                json!(types.get("number").unwrap().as_u64().unwrap() + 1);
        }
        JsonValue::String(_) => {
            *types.get_mut("string").unwrap() =
                json!(types.get("string").unwrap().as_u64().unwrap() + 1);
        }
    }

    (total, max_depth, types)
}
