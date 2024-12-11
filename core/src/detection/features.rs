use super::ModelConfig;

fn copy_at(from: &[u8], offset: usize, size: usize) -> Vec<u8> {
    from[offset..offset+size].to_vec()
}

pub fn extract_features(config: &ModelConfig, content: &[u8]) -> Vec<i32> {
    let ModelConfig {
        block_size,
        mid_size,
        ..
    } = config;

    let content_len = content.len();
    let buf_size = content_len.min(*block_size);

    let beg = copy_at(content, 0, buf_size);
    let beg = strip_prefix(&beg);

    let end = copy_at(content, content_len - buf_size, buf_size);
    let end = strip_suffix(&end);

    let mid_len = content_len.min(*mid_size);
    let mid_off = (content_len - mid_len) / 2;
    let mid = copy_at(content, mid_off, mid_len);

    let mut features = vec![config.padding_token; config.features_size()];
    let split_features = config.split_features(&mut features);
    
    copy_features(split_features.beg, beg, 0);
    copy_features(split_features.mid, &mid, 1);
    copy_features(split_features.end, end, 2);

    for (offset, features) in split_features.off {
        let mut buffer = Vec::new();
        if offset + features.len() <= content_len {
            buffer = copy_at(content, offset, features.len());
        }
        copy_features(features, &buffer, 0);
    }

    features
}

fn copy_features(dst: &mut [i32], src: &[u8], align: usize) {
    let len = std::cmp::min(dst.len(), src.len());
    let dst_len = dst.len(); // borrowing issue: cannot inline below
    let dst = &mut dst[(dst_len - len) * align / 2..][..len];
    let src = &src[(src.len() - len) * align / 2..][..len];
    for (dst, src) in dst.iter_mut().zip(src.iter()) {
        *dst = *src as i32;
    }
}

fn strip_prefix(xs: &[u8]) -> &[u8] {
    strip(xs, |xs| xs.split_first())
}

fn strip_suffix(xs: &[u8]) -> &[u8] {
    strip(xs, |xs| xs.split_last())
}

fn strip(mut xs: &[u8], mut split: impl FnMut(&[u8]) -> Option<(&u8, &[u8])>) -> &[u8] {
    while let Some((&x, ys)) = split(xs) {
        if !is_whitespace(x) {
            break;
        }
        xs = ys;
    }
    xs
}

fn is_whitespace(x: u8) -> bool {
    x.is_ascii_whitespace() || x == 0x0b
}
