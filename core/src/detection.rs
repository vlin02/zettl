use content_type::{ContentType, CONTENT_TYPES};
use features::extract_features;
use ndarray::{Array2, ArrayViewD};

use config::ModelConfig;
use ort::{inputs, session::Session};

use crate::lookup;

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

fn apply_bayes(lookup: &lookup::Table, probabilities: &[f32]) -> Vec<f32> {
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

pub fn infer_content_type(ort: &Session, lookup: &lookup::Table, content: &str) -> ContentType {
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
