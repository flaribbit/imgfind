mod model;
use candle_core::Module;
use candle_core::{DType, Device, Tensor};
use candle_nn::VarBuilder;
use tokenizers::tokenizer::{Result, Tokenizer};

pub fn load_image224<P: AsRef<std::path::Path>>(p: P) -> candle_core::Result<Tensor> {
    let img = image::io::Reader::open(p)?
        .decode()
        .map_err(candle_core::Error::wrap)?
        .resize_to_fill(224, 224, image::imageops::FilterType::Triangle);
    let img = img.to_rgb8();
    let data = img.into_raw();
    let data = Tensor::from_vec(data, (224, 224, 3), &Device::Cpu)?.permute((2, 0, 1))?;
    let mean = Tensor::new(&[0.485f32, 0.456, 0.406], &Device::Cpu)?.reshape((3, 1, 1))?;
    let std = Tensor::new(&[0.229f32, 0.224, 0.225], &Device::Cpu)?.reshape((3, 1, 1))?;
    (data.to_dtype(DType::F32)? / 255.)?
        .broadcast_sub(&mean)?
        .broadcast_div(&std)
}

fn norm(x: &Tensor) -> candle_core::Result<Tensor> {
    x.powf(2.0)?.sum(1)?.sqrt()
}

fn main() -> Result<()> {
    let tokenizer = Tokenizer::from_file("./clip/tokenizer.json")?;
    let encoding = tokenizer.encode("two cats", true)?;
    println!("{:?}", encoding.get_ids());
    let weights = unsafe { candle_core::safetensors::MmapedFile::new("clip/model.safetensors")? };
    let weights = weights.deserialize()?;
    let vb = VarBuilder::from_safetensors(vec![weights], DType::F32, &Device::Cpu);

    let text_model = model::ClipTextTransformer::new(vb.clone(), &model::Config::clip())?;
    let encoding: Vec<_> = encoding
        .get_ids()
        .iter()
        .copied()
        .chain(std::iter::repeat(0))
        .take(77)
        .collect();
    let output1 = text_model.forward(&Tensor::from_vec(encoding, (1, 77), &Device::Cpu)?)?;
    println!("{}", output1);

    let vision_model = model::ClipVisionTransformer::new(vb, &model::Config::vision())?;
    // let img = load_image224("./clip/cat.jpg")?.unsqueeze(0)?;
    let img = Tensor::zeros((1, 3, 224, 224), DType::F32, &Device::Cpu)?;
    let output2 = vision_model.forward(&img)?;
    println!("{}", output2);

    // calculate cosine similarity of output1 and output2
    let similarity = output1
        .broadcast_mul(&output2)?
        .sum(1)?
        .div(&norm(&output1)?)?
        .div(&norm(&output2)?)?;
    println!("similarity = {}", similarity);

    Ok(())
}
