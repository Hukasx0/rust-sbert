use std::path::PathBuf;

use rust_bert::Config;
use serde::Deserialize;
use tch::{Kind, Tensor};

#[derive(Debug, Deserialize)]
pub struct PoolingConfig {
    pub word_embedding_dimension: i64,
    pub pooling_mode_cls_token: bool,
    pub pooling_mode_mean_tokens: bool,
    pub pooling_mode_max_tokens: bool,
    pub pooling_mode_mean_sqrt_len_tokens: bool,
}

impl Config<PoolingConfig> for PoolingConfig {}

pub struct Pooling {
    _conf: PoolingConfig,
}

impl Pooling {
    pub fn new<P: Into<PathBuf>>(root: P) -> Pooling {
        let pooling_dir = root.into().join("1_Pooling");
        println!("Loading conf {:?}", pooling_dir);

        let config_file = pooling_dir.join("config.json");
        let _conf = PoolingConfig::from_file(&config_file);

        Pooling { _conf }
    }

    pub fn forward(&self, token_embeddings: &Tensor) -> Tensor {
        const DIM : [i64; 1] = [1];

        let dims = token_embeddings.size();
        let size = [dims[0], dims[2]];

        let sum_mask = tch::Tensor::ones(&size, (Kind::Float, token_embeddings.device())) * dims[1];
        let sum_embeddings = token_embeddings.sum1(&DIM, false, Kind::Float);
        sum_embeddings / sum_mask
    }
}
