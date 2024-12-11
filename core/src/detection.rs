use std::collections::HashMap;

use content_type::{ContentType, CONTENT_TYPES};
use features::extract_features;
use ndarray::{Array2, ArrayViewD};

use config::ModelConfig;
use ort::{inputs, session::Session};

mod config;
pub mod content_type;
mod features;

const CONFIG: ModelConfig = ModelConfig {
    beg_size: 512,
    mid_size: 512,
    end_size: 512,
    use_inputs_at_offsets: false,
    padding_token: 256,
    block_size: 4096,
};

pub fn get_probabilities(ort: &Session, content: &str) -> Vec<f32> {
    let features_size = CONFIG.features_size();

    let features = extract_features(&CONFIG, content.as_bytes());
    let features = features.into_iter().map(|x| x as f32).collect();

    let input: Array2<f32> = Array2::from_shape_vec([1, features_size], features).unwrap();

    let mut output = ort.run(inputs!(input).unwrap()).unwrap();
    let output = output.remove("model_1").unwrap();
    let output: ArrayViewD<f32> = output.try_extract_tensor::<f32>().unwrap();

    output.iter().cloned().collect()
}

const FREQUENCY_WEIGHT: f32 = 0.05;

pub struct LookupTable {
    frequency: HashMap<ContentType, f32>,
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

impl LookupTable {
    pub fn new() -> LookupTable {
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

        LookupTable { frequency }
    }
}

fn apply_bayes(lookup: &LookupTable, probabilities: &[f32]) -> Vec<f32> {
    let vals: Vec<f32> = (0..probabilities.len())
        .map(|i| {
            let content_type = CONTENT_TYPES[i];
            let (p, freq) = (probabilities[i], lookup.frequency[&content_type]);
            return freq.powf(FREQUENCY_WEIGHT) * p;
        })
        .collect();

    let tot: f32 = vals.iter().sum();

    vals.iter().map(|x| x / tot).collect()
}

pub fn infer_content_type(ort: &Session, lookup: &LookupTable, content: &str) -> ContentType {
    let ps = get_probabilities(ort, content);
    let ps = apply_bayes(&lookup, &ps);

    let mut max_i = 0;
    for i in 0..ps.len() {
        if ps[max_i] < ps[i] {
            max_i = i;
        }
    }

    CONTENT_TYPES[max_i].clone()
}
