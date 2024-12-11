use features::extract_features;
use ndarray::{Array2, ArrayViewD};

use config::ModelConfig;
use ort::{inputs, session::Session};

mod config;
mod content_type;
mod features;

const CONFIG: ModelConfig = ModelConfig {
    beg_size: 512,
    mid_size: 512,
    end_size: 512,
    use_inputs_at_offsets: false,
    padding_token: 256,
    block_size: 4096,
};

pub fn predict_content_type(session: &Session, content: &str) -> Vec<f32> {
    let features_size = CONFIG.features_size();

    let features = extract_features(&CONFIG, content.as_bytes());
    let features = features.into_iter().map(|x| x as f32).collect();

    let input: Array2<f32> = Array2::from_shape_vec([1, features_size], features).unwrap();

    let mut output = session.run(inputs!(input).unwrap()).unwrap();
    let output = output.remove("model_1").unwrap();
    let output: ArrayViewD<f32> = output.try_extract_tensor::<f32>().unwrap();

    output.iter().cloned().collect()
}
