// Copyright 2024 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[derive(Debug)]
pub struct ModelConfig {
    pub beg_size: usize,
    pub mid_size: usize,
    pub end_size: usize,
    pub use_inputs_at_offsets: bool,
    pub padding_token: i32,
    pub block_size: usize,
}

pub struct SplitFeatures<'a> {
    pub beg: &'a mut [i32],
    pub mid: &'a mut [i32],
    pub end: &'a mut [i32],
    pub off: Vec<(usize, &'a mut [i32])>,
}

impl ModelConfig {
    pub fn features_size(&self) -> usize {
        let offsets_size = if self.use_inputs_at_offsets { 4 * 8 } else { 0 };
        self.beg_size + self.mid_size + self.end_size + offsets_size
    }

    pub fn split_features<'a>(&self, features: &'a mut [i32]) -> SplitFeatures<'a> {
        let (beg, features) = features.split_at_mut(self.beg_size);
        let (mid, features) = features.split_at_mut(self.mid_size);
        let (end, mut features) = features.split_at_mut(self.end_size);
        let mut off = Vec::new();
        if self.use_inputs_at_offsets {
            for offset in [0x8000, 0x8800, 0x9000, 0x9800] {
                let (head, tail) = features.split_at_mut(8);
                features = tail;
                off.push((offset, head));
            }
        }
        debug_assert!(features.is_empty());
        SplitFeatures { beg, mid, end, off }
    }
}
