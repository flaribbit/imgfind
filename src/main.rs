mod model;
use candle_core::Module;
use candle_core::{DType, Device, Tensor};
use candle_nn::VarBuilder;
use tokenizers::tokenizer::{Result, Tokenizer};

fn main() -> Result<()> {
    let tokenizer = Tokenizer::from_file("./clip/tokenizer.json")?;
    let encoding = tokenizer.encode("a cat", true)?;
    println!("{:?}", encoding.get_ids());
    let weights = unsafe { candle_core::safetensors::MmapedFile::new("clip/model.safetensors")? };
    let weights = weights.deserialize()?;
    let vb = VarBuilder::from_safetensors(vec![weights], DType::F32, &Device::Cpu);
    let text_model = model::ClipTextTransformer::new(vb, &model::Config::clip())?;
    let encoding: Vec<_> = encoding
        .get_ids()
        .iter()
        .copied()
        .chain(std::iter::repeat(0))
        .take(77)
        .collect();
    let output = text_model.forward(&Tensor::from_vec(encoding, (1, 77), &Device::Cpu)?)?;
    println!("{}", output);
    Ok(())
}
