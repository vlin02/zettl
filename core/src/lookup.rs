use std::collections::HashMap;

use crate::detection::format::{Format, FORMATS};

pub struct Table {
    pub frequency: HashMap<Format, f32>,
    pub format_by_key: HashMap<String, Format>,
}

fn so_frequency(format: Format) -> f32 {
    match format {
        Format::JavaScript => 0.11635059103195514 + 0.07184242458097091, // typescript
        Format::Html => 0.04936800389,
        Format::Css => 0.04936800389,
        Format::Python => 0.09533163890725034 + 0.011569268200339506, // lua
        Format::Shell => 0.063345467410228,
        Format::Java => 0.05660189986748719,
        Format::C => {
            0.03781114907535851 + 0.050640375132280056 + 0.04290994404669913 + 0.011153420040777946
        } // c#, c++, dart
        Format::Php => 0.0339847253384973,
        Format::PowerShell => 0.02584465278230349,
        Format::Go => 0.025146400275576988,
        Format::Rust => 0.02345818088153603,
        Format::Scala => 0.017580446447136078, // kotlin
        _ => 0.0,
    }
}

impl Table {
    pub fn new() -> Table {
        let total_freq: f32 = [
            Format::JavaScript,
            Format::Html,
            Format::Css,
            Format::Python,
            Format::Shell,
            Format::Java,
            Format::C,
            Format::Php,
            Format::PowerShell,
            Format::Go,
            Format::Rust,
            Format::Scala,
        ]
        .iter()
        .map(|&x| so_frequency(x))
        .sum();

        let leftover_freq = 1.0 - total_freq;
        let mut frequency: HashMap<Format, f32> = HashMap::new();

        let mut leftover_formats: Vec<Format> = Vec::new();

        for format in FORMATS {
            let freq = so_frequency(format);
            if freq > 0.0 || !format.is_text() {
                frequency.insert(format, freq);
            } else {
                leftover_formats.push(format);
            }
        }

        let base_freq = leftover_freq / leftover_formats.len() as f32;
        for format in leftover_formats {
            frequency.insert(format, base_freq);
        }

        let mut format_by_key = HashMap::new();
        for format in FORMATS {
            format_by_key.insert(format.key().to_string(), format);
        }
        
        Table {
            frequency,
            format_by_key,
        }
    }
}
