//! Contrastive Language-Image Pre-Training
//!
//! Contrastive Language-Image Pre-Training (CLIP) is an architecture trained on
//! pairs of images with related texts.
//!
//! https://github.com/openai/CLIP
//! https://github.com/huggingface/candle/blob/main/candle-transformers/src/models/stable_diffusion/clip.rs
use candle_core::{DType, Device, Result, Tensor, D};
use candle_nn as nn;
use candle_nn::Module;
use nn::{Conv2dConfig, Embedding};

#[derive(Debug, Clone, Copy)]
pub enum Activation {
    QuickGelu,
    Gelu,
}

impl Module for Activation {
    fn forward(&self, xs: &Tensor) -> Result<Tensor> {
        match self {
            Activation::QuickGelu => xs * nn::ops::sigmoid(&(xs * 1.702f64)?)?,
            Activation::Gelu => xs.gelu(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    vocab_size: usize,
    embed_dim: usize,       // aka config.hidden_size
    activation: Activation, // aka config.hidden_act
    intermediate_size: usize,
    pub max_position_embeddings: usize,
    // The character to use for padding, use EOS when not set.
    pub pad_with: Option<String>,
    num_hidden_layers: usize,
    num_attention_heads: usize,
    #[allow(dead_code)]
    projection_dim: usize,
}

pub struct VisionConfig {
    hidden_size: usize,
    image_size: usize,
    patch_size: usize,
    num_attention_heads: usize,
    intermediate_size: usize,
}

impl Config {
    // The config details can be found in the "text_config" section of this json file:
    // https://huggingface.co/openai/clip-vit-base-patch32/blob/main/config.json
    pub fn clip() -> Self {
        Self {
            vocab_size: 49408,
            embed_dim: 512,
            activation: Activation::QuickGelu,
            intermediate_size: 2048,
            max_position_embeddings: 77,
            pad_with: Some("!".to_string()),
            num_hidden_layers: 12,
            num_attention_heads: 8,
            projection_dim: 512,
        }
    }

    pub fn vision() -> Self {
        Self {
            vocab_size: 49408,
            embed_dim: 768,
            activation: Activation::QuickGelu,
            intermediate_size: 3072,
            max_position_embeddings: 77,
            pad_with: Some("!".to_string()),
            num_hidden_layers: 12,
            num_attention_heads: 8,
            projection_dim: 512,
        }
    }

    // https://huggingface.co/openai/clip-vit-large-patch14/blob/main/config.json
    pub fn v1_5() -> Self {
        Self {
            vocab_size: 49408,
            embed_dim: 768,
            intermediate_size: 3072,
            max_position_embeddings: 77,
            pad_with: None,
            num_hidden_layers: 12,
            num_attention_heads: 12,
            projection_dim: 768,
            activation: Activation::QuickGelu,
        }
    }

    // https://huggingface.co/stabilityai/stable-diffusion-2-1/blob/main/text_encoder/config.json
    pub fn v2_1() -> Self {
        Self {
            vocab_size: 49408,
            embed_dim: 1024,
            intermediate_size: 4096,
            max_position_embeddings: 77,
            pad_with: Some("!".to_string()),
            num_hidden_layers: 23,
            num_attention_heads: 16,
            projection_dim: 512,
            activation: Activation::Gelu,
        }
    }

    // https://huggingface.co/stabilityai/stable-diffusion-xl-base-1.0/blob/main/text_encoder/config.json
    pub fn sdxl() -> Self {
        Self {
            vocab_size: 49408,
            embed_dim: 768,
            intermediate_size: 3072,
            max_position_embeddings: 77,
            pad_with: Some("!".to_string()),
            num_hidden_layers: 12,
            num_attention_heads: 12,
            projection_dim: 768,
            activation: Activation::QuickGelu,
        }
    }

    // https://huggingface.co/stabilityai/stable-diffusion-xl-base-1.0/blob/main/text_encoder_2/config.json
    pub fn sdxl2() -> Self {
        Self {
            vocab_size: 49408,
            embed_dim: 1280,
            intermediate_size: 5120,
            max_position_embeddings: 77,
            pad_with: Some("!".to_string()),
            num_hidden_layers: 32,
            num_attention_heads: 20,
            projection_dim: 1280,
            activation: Activation::Gelu,
        }
    }
}

// CLIP Text Model
// https://github.com/huggingface/transformers/blob/674f750a57431222fa2832503a108df3badf1564/src/transformers/models/clip/modeling_clip.py
#[derive(Debug)]
struct ClipTextEmbeddings {
    token_embedding: candle_nn::Embedding,
    position_embedding: candle_nn::Embedding,
    position_ids: Tensor,
}

impl ClipTextEmbeddings {
    fn new(vs: candle_nn::VarBuilder, c: &Config) -> Result<Self> {
        let token_embedding =
            candle_nn::embedding(c.vocab_size, c.embed_dim, vs.pp("token_embedding"))?;
        let position_embedding = candle_nn::embedding(
            c.max_position_embeddings,
            c.embed_dim,
            vs.pp("position_embedding"),
        )?;
        let position_ids =
            Tensor::arange(0u32, c.max_position_embeddings as u32, vs.device())?.unsqueeze(0)?;
        Ok(ClipTextEmbeddings {
            token_embedding,
            position_embedding,
            position_ids,
        })
    }
}

impl Module for ClipTextEmbeddings {
    fn forward(&self, xs: &Tensor) -> Result<Tensor> {
        let token_embedding = self.token_embedding.forward(xs)?;
        let position_embedding = self.position_embedding.forward(&self.position_ids)?;
        token_embedding.broadcast_add(&position_embedding)
    }
}

#[derive(Debug)]
struct ClipAttention {
    k_proj: candle_nn::Linear,
    v_proj: candle_nn::Linear,
    q_proj: candle_nn::Linear,
    out_proj: candle_nn::Linear,
    head_dim: usize,
    scale: f64,
    num_attention_heads: usize,
}

impl ClipAttention {
    fn new(vs: candle_nn::VarBuilder, c: &Config) -> Result<Self> {
        let embed_dim = c.embed_dim;
        let num_attention_heads = c.num_attention_heads;
        let k_proj = candle_nn::linear(embed_dim, embed_dim, vs.pp("k_proj"))?;
        let v_proj = candle_nn::linear(embed_dim, embed_dim, vs.pp("v_proj"))?;
        let q_proj = candle_nn::linear(embed_dim, embed_dim, vs.pp("q_proj"))?;
        let out_proj = candle_nn::linear(embed_dim, embed_dim, vs.pp("out_proj"))?;
        let head_dim = embed_dim / num_attention_heads;
        let scale = (head_dim as f64).powf(-0.5);
        Ok(ClipAttention {
            k_proj,
            v_proj,
            q_proj,
            out_proj,
            head_dim,
            scale,
            num_attention_heads,
        })
    }

    fn shape(&self, xs: &Tensor, seq_len: usize, bsz: usize) -> Result<Tensor> {
        xs.reshape((bsz, seq_len, self.num_attention_heads, self.head_dim))?
            .transpose(1, 2)?
            .contiguous()
    }

    fn forward(&self, xs: &Tensor, causal_attention_mask: Option<&Tensor>) -> Result<Tensor> {
        let in_dtype = xs.dtype();
        let (bsz, seq_len, embed_dim) = xs.dims3()?;
        let query_states = (self.q_proj.forward(xs)? * self.scale)?;
        let proj_shape = (bsz * self.num_attention_heads, seq_len, self.head_dim);
        let query_states = self
            .shape(&query_states, seq_len, bsz)?
            .reshape(proj_shape)?
            .to_dtype(DType::F32)?;
        let key_states = self
            .shape(&self.k_proj.forward(xs)?, seq_len, bsz)?
            .reshape(proj_shape)?
            .to_dtype(DType::F32)?;
        let value_states = self
            .shape(&self.v_proj.forward(xs)?, seq_len, bsz)?
            .reshape(proj_shape)?
            .to_dtype(DType::F32)?;
        let attn_weights = query_states.matmul(&key_states.transpose(1, 2)?)?;

        let src_len = key_states.dim(1)?;
        let attn_weights = if let Some(causal_attention_mask) = causal_attention_mask {
            attn_weights
                .reshape((bsz, self.num_attention_heads, seq_len, src_len))?
                .broadcast_add(causal_attention_mask)?
        } else {
            attn_weights.reshape((bsz, self.num_attention_heads, seq_len, src_len))?
        };
        let attn_weights =
            attn_weights.reshape((bsz * self.num_attention_heads, seq_len, src_len))?;
        let attn_weights = candle_nn::ops::softmax(&attn_weights, D::Minus1)?;

        let attn_output = attn_weights.matmul(&value_states)?.to_dtype(in_dtype)?;
        let attn_output = attn_output
            .reshape((bsz, self.num_attention_heads, seq_len, self.head_dim))?
            .transpose(1, 2)?
            .reshape((bsz, seq_len, embed_dim))?;
        self.out_proj.forward(&attn_output)
    }
}

#[derive(Debug)]
struct ClipMlp {
    fc1: candle_nn::Linear,
    fc2: candle_nn::Linear,
    activation: Activation,
}

impl ClipMlp {
    fn new(vs: candle_nn::VarBuilder, c: &Config) -> Result<Self> {
        let fc1 = candle_nn::linear(c.embed_dim, c.intermediate_size, vs.pp("fc1"))?;
        let fc2 = candle_nn::linear(c.intermediate_size, c.embed_dim, vs.pp("fc2"))?;
        Ok(ClipMlp {
            fc1,
            fc2,
            activation: c.activation,
        })
    }
}

impl ClipMlp {
    fn forward(&self, xs: &Tensor) -> Result<Tensor> {
        let xs = self.fc1.forward(xs)?;
        self.fc2.forward(&self.activation.forward(&xs)?)
    }
}

#[derive(Debug)]
struct ClipEncoderLayer {
    self_attn: ClipAttention,
    layer_norm1: candle_nn::LayerNorm,
    mlp: ClipMlp,
    layer_norm2: candle_nn::LayerNorm,
}

impl ClipEncoderLayer {
    fn new(vs: candle_nn::VarBuilder, c: &Config) -> Result<Self> {
        let self_attn = ClipAttention::new(vs.pp("self_attn"), c)?;
        let layer_norm1 = candle_nn::layer_norm(c.embed_dim, 1e-5, vs.pp("layer_norm1"))?;
        let mlp = ClipMlp::new(vs.pp("mlp"), c)?;
        let layer_norm2 = candle_nn::layer_norm(c.embed_dim, 1e-5, vs.pp("layer_norm2"))?;
        Ok(ClipEncoderLayer {
            self_attn,
            layer_norm1,
            mlp,
            layer_norm2,
        })
    }

    fn forward(&self, xs: &Tensor, causal_attention_mask: Option<&Tensor>) -> Result<Tensor> {
        let residual = xs;
        let xs = self.layer_norm1.forward(xs)?;
        let xs = self.self_attn.forward(&xs, causal_attention_mask)?;
        let xs = (xs + residual)?;

        let residual = &xs;
        let xs = self.layer_norm2.forward(&xs)?;
        let xs = self.mlp.forward(&xs)?;
        xs + residual
    }
}

#[derive(Debug)]
struct ClipEncoder {
    layers: Vec<ClipEncoderLayer>,
}

impl ClipEncoder {
    fn new(vs: candle_nn::VarBuilder, c: &Config) -> Result<Self> {
        let vs = vs.pp("layers");
        let mut layers: Vec<ClipEncoderLayer> = Vec::new();
        for index in 0..c.num_hidden_layers {
            let layer = ClipEncoderLayer::new(vs.pp(&index.to_string()), c)?;
            layers.push(layer)
        }
        Ok(ClipEncoder { layers })
    }

    fn forward(&self, xs: &Tensor, causal_attention_mask: Option<&Tensor>) -> Result<Tensor> {
        let mut xs = xs.clone();
        for layer in self.layers.iter() {
            xs = layer.forward(&xs, causal_attention_mask)?;
        }
        Ok(xs)
    }
}

/// A CLIP transformer based model.
#[derive(Debug)]
pub struct ClipTextTransformer {
    embeddings: ClipTextEmbeddings,
    encoder: ClipEncoder,
    final_layer_norm: candle_nn::LayerNorm,
    text_projection: Tensor,
}

impl ClipTextTransformer {
    pub fn new(vs: candle_nn::VarBuilder, c: &Config) -> Result<Self> {
        let text_projection = vs.get((512, 512), "text_projection.weight")?;
        let vs = vs.pp("text_model");
        let embeddings = ClipTextEmbeddings::new(vs.pp("embeddings"), c)?;
        let encoder = ClipEncoder::new(vs.pp("encoder"), c)?;
        let final_layer_norm = candle_nn::layer_norm(c.embed_dim, 1e-5, vs.pp("final_layer_norm"))?;
        Ok(Self {
            embeddings,
            encoder,
            final_layer_norm,
            text_projection,
        })
    }

    // https://github.com/huggingface/transformers/blob/674f750a57431222fa2832503a108df3badf1564/src/transformers/models/clip/modeling_clip.py#L678
    fn build_causal_attention_mask(bsz: usize, seq_len: usize, device: &Device) -> Result<Tensor> {
        let mask: Vec<_> = (0..seq_len)
            .flat_map(|i| (0..seq_len).map(move |j| if j > i { f32::MIN } else { 0. }))
            .collect();
        let mask = Tensor::from_slice(&mask, (seq_len, seq_len), device)?;
        mask.broadcast_as((bsz, seq_len, seq_len))
    }
}

impl Module for ClipTextTransformer {
    fn forward(&self, xs: &Tensor) -> Result<Tensor> {
        let argmax: Vec<u32> = xs.argmax(1)?.to_vec1()?;
        let (bsz, seq_len) = xs.dims2()?;
        let xs = self.embeddings.forward(xs)?;
        let causal_attention_mask = Self::build_causal_attention_mask(bsz, seq_len, xs.device())?;
        let xs = self.encoder.forward(&xs, Some(&causal_attention_mask))?;
        let xs = self.final_layer_norm.forward(&xs)?;
        let (a, b, c) = xs.dims3()?;
        let ids: Vec<u32> = argmax
            .iter()
            .enumerate()
            .map(|(i, &x)| (i as u32) * (a as u32) + x)
            .collect();
        xs.reshape((a * b, c))?
            .index_select(&Tensor::from_vec(ids, a, &Device::Cpu)?, 0)?
            .matmul(&self.text_projection)
    }
}

#[derive(Debug)]
struct ClipVisionEmbeddings {
    class_embedding: Tensor,
    patch_embedding: candle_nn::Conv2d,
    position_embedding: Embedding,
    position_ids: Tensor,
}

impl ClipVisionEmbeddings {
    fn new(vs: candle_nn::VarBuilder, c: &Config) -> Result<Self> {
        let class_embedding = vs.get(768, "class_embedding")?;
        let patch_embedding = candle_nn::conv2d_no_bias(
            3,
            c.embed_dim,
            32,
            Conv2dConfig {
                stride: 32,
                padding: 0,
                ..Default::default()
            },
            vs.pp("patch_embedding"),
        )?;
        let position_embedding = candle_nn::embedding(50, 768, vs.pp("position_embedding"))?;
        // let position_embedding = vs.get((50, 768), "position_embedding.weight")?;
        let position_ids = Tensor::arange(0u32, 50, vs.device())?.unsqueeze(0)?;
        Ok(Self {
            class_embedding,
            patch_embedding,
            position_embedding,
            position_ids,
        })
    }
}

impl Module for ClipVisionEmbeddings {
    fn forward(&self, x: &Tensor) -> Result<Tensor> {
        // let x = self.patch_embedding.forward(x)?;
        // let x = x.reshape((x.dim(0)?, x.dim(1)?, ()))?;
        // let x = x.permute((0, 2, 1))?;
        // let x = Tensor::cat(&[self.class_embedding.unsqueeze(0)?.unsqueeze(0)?, x], 1)?;
        // let x = x.broadcast_add(&self.position_embedding)?;
        // Ok(x)
        let batch_size = x.dim(0)?;
        let patch_embeds = self.patch_embedding.forward(x)?;
        // println!("patch_embeds: {}", patch_embeds);
        let patch_embeds = patch_embeds.flatten(2, 3)?.transpose(1, 2)?;
        // println!("patch_embeds: {}", patch_embeds);
        let class_embeds = self.class_embedding.expand((batch_size, 1, 768))?; // correct
        let embeddings = Tensor::cat(&[class_embeds, patch_embeds], 1)?;
        embeddings.add(&self.position_embedding.forward(&self.position_ids)?)
    }
}

#[derive(Debug)]
pub struct ClipVisionTransformer {
    embeddings: ClipVisionEmbeddings,
    pre_layrnorm: candle_nn::LayerNorm,
    encoder: ClipEncoder,
    post_layernorm: candle_nn::LayerNorm,
    visual_projection: Tensor,
}

impl ClipVisionTransformer {
    pub fn new(vs: candle_nn::VarBuilder, c: &Config) -> Result<Self> {
        let visual_projection = vs.get((512, 768), "visual_projection.weight")?;
        let vs = vs.pp("vision_model");
        let embeddings = ClipVisionEmbeddings::new(vs.pp("embeddings"), c)?;
        let pre_layrnorm = candle_nn::layer_norm(c.embed_dim, 1e-5, vs.pp("pre_layrnorm"))?;
        let encoder = ClipEncoder::new(vs.pp("encoder"), c)?;
        let post_layernorm = candle_nn::layer_norm(c.embed_dim, 1e-5, vs.pp("post_layernorm"))?;
        Ok(Self {
            embeddings,
            pre_layrnorm,
            encoder,
            post_layernorm,
            visual_projection,
        })
    }
}

impl Module for ClipVisionTransformer {
    fn forward(&self, xs: &Tensor) -> Result<Tensor> {
        use candle_core::IndexOp;
        // let x = self.embeddings.forward(xs)?;
        // let x = self.pre_layrnorm.forward(&x)?;
        // let x = self
        //     .encoder
        //     .forward(&x, &Tensor::zeros((8, 50, 50), DType::F32, xs.device())?)?;
        // println!("x: {}", x);
        // let x = x.i((.., 0, ..))?;
        // let x = self.post_layernorm.forward(&x)?;
        // x.matmul(&self.visual_projection.t()?)

        let hidden_states = self.embeddings.forward(xs)?; // correct
        let hidden_states = self.pre_layrnorm.forward(&hidden_states)?;
        // println!("hidden_states: {}", hidden_states); //correct
        let encoder_outputs = self.encoder.forward(&hidden_states, None)?;
        println!("encoder_outputs: {}", encoder_outputs); // wrong
        let last_hidden_state = encoder_outputs.get(0)?;
        let pooled_output = last_hidden_state.i((.., 0, ..))?;
        let pooled_output = self.post_layernorm.forward(&pooled_output)?;
        Ok(pooled_output)
    }
}
