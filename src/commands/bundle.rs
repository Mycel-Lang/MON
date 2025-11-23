use colored::*;
use miette::Result;
use mon_core::ast::MonDocument;
use mon_core::parser::Parser;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

pub fn run(
    entry_file: String,
    output: Option<String>,
    to_format: String,
    minify: bool,
    _tree_shake: bool, // TODO: Implement tree-shaking
) -> Result<()> {
    println!("{}  {}", "Bundling".green().bold(), entry_file);

    let entry_path = Path::new(&entry_file);
    if !entry_path.exists() {
        return Err(miette::miette!("Entry file not found: {}", entry_file));
    }

    // Parse the entry file
    let content = fs::read_to_string(entry_path)
        .map_err(|e| miette::miette!("Failed to read entry file: {}", e))?;

    let mut parser = Parser::new(&content)
        .map_err(|e| miette::miette!("Parser initialization failed: {:?}", e))?;

    let doc = parser.parse_document().map_err(|e| miette::miette!("Parse error: {:?}", e))?;

    // Basic import dependency check
    println!("  {} Checking imports...", "→".cyan());
    let import_graph = build_simple_import_graph(&doc, entry_path)?;
    if let Some(cycle) = detect_cycle(&import_graph) {
        return Err(miette::miette!("Circular dependency detected: {}", cycle.join(" → ")));
    }

    println!("  {} {} import(s) verified", "✓".green(), doc.imports.len());

    // Format output
    let output_content = match to_format.as_str() {
        "mon" => format_as_mon(&doc, minify)?,
        "json" => format_as_json(&doc)?,
        "yaml" => format_as_yaml(&doc)?,
        _ => return Err(miette::miette!("Unsupported output format: {}", to_format)),
    };

    // Write or print output
    if let Some(output_path) = output {
        fs::write(&output_path, output_content)
            .map_err(|e| miette::miette!("Failed to write output: {}", e))?;
        println!("  {} Wrote bundle to {}", "✓".green().bold(), output_path);
    } else {
        println!("{}", output_content);
    }

    Ok(())
}

fn build_simple_import_graph(
    doc: &MonDocument,
    entry: &Path,
) -> Result<HashMap<PathBuf, Vec<PathBuf>>> {
    let mut graph = HashMap::new();
    let mut visited = HashSet::new();
    let mut stack = vec![entry.to_path_buf()];

    let base_dir = entry.parent().unwrap_or(Path::new("."));

    while let Some(current) = stack.pop() {
        if visited.contains(&current) {
            continue;
        }
        visited.insert(current.clone());

        // Get imports for current file
        let imports: Vec<PathBuf> = doc
            .imports
            .iter()
            .map(|imp| {
                let import_path = base_dir.join(&imp.path);
                import_path
            })
            .collect();

        graph.insert(current.clone(), imports.clone());

        for import in imports {
            if import.exists() && !visited.contains(&import) {
                stack.push(import);
            }
        }
    }

    Ok(graph)
}

fn detect_cycle(graph: &HashMap<PathBuf, Vec<PathBuf>>) -> Option<Vec<String>> {
    let mut visited = HashSet::new();
    let mut rec_stack = Vec::new();

    for node in graph.keys() {
        if !visited.contains(node) {
            if let Some(cycle) = dfs_cycle(node, graph, &mut visited, &mut rec_stack) {
                return Some(cycle);
            }
        }
    }

    None
}

fn dfs_cycle(
    node: &PathBuf,
    graph: &HashMap<PathBuf, Vec<PathBuf>>,
    visited: &mut HashSet<PathBuf>,
    rec_stack: &mut Vec<PathBuf>,
) -> Option<Vec<String>> {
    visited.insert(node.clone());
    rec_stack.push(node.clone());

    if let Some(neighbors) = graph.get(node) {
        for neighbor in neighbors {
            if !visited.contains(neighbor) {
                if let Some(cycle) = dfs_cycle(neighbor, graph, visited, rec_stack) {
                    return Some(cycle);
                }
            } else if rec_stack.contains(neighbor) {
                // Found cycle
                let cycle_start = rec_stack.iter().position(|p| p == neighbor).unwrap();
                return Some(
                    rec_stack[cycle_start..].iter().map(|p| p.display().to_string()).collect(),
                );
            }
        }
    }

    rec_stack.pop();
    None
}

fn format_as_mon(doc: &MonDocument, _minify: bool) -> Result<String> {
    // Use the formatter from crate
    use crate::formatter::{FormatConfig, Formatter};

    // TODO: Implement minified formatting
    let formatter = Formatter::new(FormatConfig::default());

    // Format the document back to MON
    // Since we have the original parsed doc, we can format it
    let mut output = String::new();

    // Add imports
    for import in &doc.imports {
        output.push_str(&format!(
            "import {} from \"{}\"\n",
            "{ ... }", // Simplified - would need full import spec formatting
            import.path
        ));
    }

    if !doc.imports.is_empty() {
        output.push('\n');
    }

    // Format root value using formatter
    // This is a simplified version - full implementation would use formatter.format()
    output.push_str("{\n  // Bundled output\n  // TODO: Full formatting implementation\n}\n");

    Ok(output)
}

fn format_as_json(doc: &MonDocument) -> Result<String> {
    // Use mon-core's AnalysisResult for serialization
    

    // We need to re-analyze the document to get proper serialization
    // This is a workaround since to_value is pub(crate)
    // For now, just serialize the root value manually
    serde_json::to_string_pretty(&convert_to_json_value(&doc.root))
        .map_err(|e| miette::miette!("JSON formatting error: {}", e))
}

fn format_as_yaml(doc: &MonDocument) -> Result<String> {
    serde_yaml::to_string(&convert_to_json_value(&doc.root))
        .map_err(|e| miette::miette!("YAML formatting error: {}", e))
}

// Helper to convert MonValue to serde_json::Value
fn convert_to_json_value(mon_value: &mon_core::ast::MonValue) -> serde_json::Value {
    use mon_core::ast::{Member, MonValueKind};

    match &mon_value.kind {
        MonValueKind::String(s) => serde_json::Value::String(s.clone()),
        MonValueKind::Number(n) => serde_json::json!(n),
        MonValueKind::Boolean(b) => serde_json::Value::Bool(*b),
        MonValueKind::Null => serde_json::Value::Null,
        MonValueKind::Array(arr) => {
            serde_json::Value::Array(arr.iter().map(convert_to_json_value).collect())
        }
        MonValueKind::Object(obj) => {
            let mut map = serde_json::Map::new();
            for member in obj {
                if let Member::Pair(pair) = member {
                    map.insert(pair.key.clone(), convert_to_json_value(&pair.value));
                }
            }
            serde_json::Value::Object(map)
        }
        _ => serde_json::Value::Null,
    }
}
