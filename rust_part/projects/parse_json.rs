use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Debug)]
struct Snippet {
    name: String,
    prefix_or_postfix: Vec<String>,
    is_prefix: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Resolve the path to the settings file
    let mut path = dirs::home_dir().expect("Could not find home directory");
    path.push(".config");
    path.push("zed");
    path.push("settings.json");

    // Read the JSON file
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Parse the JSON
    let json: Value = serde_json::from_str(&contents)?;

    // Navigate to the snippets section
    let snippets =
        &json["lsp"]["rust-analyzer"]["initialization_options"]["completion"]["snippets"]["custom"];

    // Extract the snippets
    let mut extracted_snippets: Vec<Snippet> = Vec::new();

    if let Some(obj) = snippets.as_object() {
        for (name, value) in obj {
            if let Some(snippet_obj) = value.as_object() {
                if let Some(prefix) = snippet_obj.get("prefix") {
                    let prefix_vec = extract_string_or_array(prefix);
                    extracted_snippets.push(Snippet {
                        name: name.to_string(),
                        prefix_or_postfix: prefix_vec,
                        is_prefix: true,
                    });
                } else if let Some(postfix) = snippet_obj.get("postfix") {
                    let postfix_vec = extract_string_or_array(postfix);
                    extracted_snippets.push(Snippet {
                        name: name.to_string(),
                        prefix_or_postfix: postfix_vec,
                        is_prefix: false,
                    });
                }
            }
        }
    }

    // Print the extracted information
    for snippet in extracted_snippets {
        println!(
            "{} -> {}: {}",
            snippet.name,
            if snippet.is_prefix {
                "prefix"
            } else {
                "postfix"
            },
            snippet.prefix_or_postfix.join(", ")
        );
    }

    Ok(())
}

fn extract_string_or_array(value: &Value) -> Vec<String> {
    if let Some(s) = value.as_str() {
        vec![s.to_string()]
    } else if let Some(arr) = value.as_array() {
        arr.iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect()
    } else {
        Vec::new()
    }
}
