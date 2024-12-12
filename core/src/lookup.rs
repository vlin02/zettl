use std::collections::HashMap;

use crate::detection::content_type::{ContentType, CONTENT_TYPES};

pub struct Table {
    pub frequency: HashMap<ContentType, f32>,
    pub content_type_by_key: HashMap<String, ContentType>,
}

fn so_frequency(content_type: ContentType) -> f32 {
    match content_type {
        ContentType::JavaScript => 0.11635059103195514 + 0.07184242458097091, // typescript
        ContentType::Html => 0.04936800389,
        ContentType::Css => 0.04936800389,
        ContentType::Python => 0.09533163890725034 + 0.011569268200339506, // lua
        ContentType::Shell => 0.063345467410228,
        ContentType::Java => 0.05660189986748719,
        ContentType::C => {
            0.03781114907535851 + 0.050640375132280056 + 0.04290994404669913 + 0.011153420040777946
        } // c#, c++, dart
        ContentType::Php => 0.0339847253384973,
        ContentType::PowerShell => 0.02584465278230349,
        ContentType::Go => 0.025146400275576988,
        ContentType::Rust => 0.02345818088153603,
        ContentType::Scala => 0.017580446447136078, // kotlin
        _ => 0.0,
    }
}

impl Table {
    pub fn new() -> Table {
        let total_freq: f32 = [
            ContentType::JavaScript,
            ContentType::Html,
            ContentType::Css,
            ContentType::Python,
            ContentType::Shell,
            ContentType::Java,
            ContentType::C,
            ContentType::Php,
            ContentType::PowerShell,
            ContentType::Go,
            ContentType::Rust,
            ContentType::Scala,
        ]
        .iter()
        .map(|&x| so_frequency(x))
        .sum();

        let leftover_freq = 1.0 - total_freq;
        let mut frequency: HashMap<ContentType, f32> = HashMap::new();

        let mut leftover_content_types: Vec<ContentType> = Vec::new();

        for content_type in CONTENT_TYPES {
            let freq = so_frequency(content_type);
            if freq > 0.0 || !content_type.is_text() {
                frequency.insert(content_type, freq);
            } else {
                leftover_content_types.push(content_type);
            }
        }

        let base_freq = leftover_freq / leftover_content_types.len() as f32;
        for content_type in leftover_content_types {
            frequency.insert(content_type, base_freq);
        }

        let mut content_type_by_key = HashMap::new();
        for content_type in CONTENT_TYPES {
            content_type_by_key.insert(content_type.key().to_string(), content_type);
        }
        
        Table {
            frequency,
            content_type_by_key,
        }
    }
}
