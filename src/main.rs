mod model;
use std::collections::BTreeMap;
use std::error::Error;
use std::fs::read;
use std::net::{SocketAddrV4, Ipv4Addr};
use std::path::Path;
use std::sync::Arc;

use candle_core::Module;
use candle_core::{DType, Device, Tensor};
use candle_nn::VarBuilder;
use serde_json;
use tokenizers::tokenizer;
use tokenizers::tokenizer::Tokenizer;
use xjbutil::minhttpd::{MinHttpd, HttpUri, HttpHeaders, HttpParams, HttpBody, HttpResponse};

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

type Embedding = Vec<f32>;
type Database = BTreeMap<String, Embedding>;

fn load_database() -> Database {
    match std::fs::File::open("database.bin") {
        Ok(file) => rmp_serde::from_read(file).expect("failed to read database.bin"),
        Err(_) => BTreeMap::new(),
    }
}

fn save_database(database: &Database) {
    let mut file = std::fs::File::create("database.bin").expect("failed to create database.bin");
    rmp_serde::encode::write(&mut file, database).expect("failed to write database.bin");
}

fn add_image_feature(
    database: &mut Database,
    model: &model::ClipVisionTransformer,
    path: &str,
) -> tokenizer::Result<()> {
    let img = load_image224(path)
        .expect("failed to load image")
        .unsqueeze(0)?;
    let output: Vec<f32> = model.forward(&img)?.squeeze(0)?.to_vec1()?;
    let output = normalize(&output);
    database.insert(path.to_string(), output);
    Ok(())
}

fn get_images(path: &str) -> Vec<String> {
    let mut result = Vec::new();
    fn recurse(path: &str, result: &mut Vec<String>) {
        for entry in std::fs::read_dir(path).expect("failed to read dir") {
            let entry = entry.expect("failed to read entry");
            let path = entry.path();
            if path.is_dir() {
                recurse(&path.to_string_lossy(), result);
            } else {
                let path = path.to_string_lossy();
                if path.ends_with(".jpg") || path.ends_with(".png") || path.ends_with(".jpeg") {
                    result.push(path.to_string());
                }
            }
        }
    }
    recurse(path, &mut result);
    result
}

fn find_image<'a>(
    database: &'a Database,
    model: &model::ClipTextTransformer,
    tokenizer: &Tokenizer,
    text: &str,
) -> tokenizer::Result<Vec<(&'a String, f32)>> {
    let mut text_ids = [0u32; 77];
    let encoding = tokenizer.encode(text, true)?;
    let encoding_len = encoding.get_ids().len().min(77);
    text_ids[..encoding_len].copy_from_slice(&encoding.get_ids()[..encoding_len]);
    let feature: Vec<f32> = model
        .forward(&Tensor::from_vec(text_ids.to_vec(), (1, 77), &Device::Cpu)?)?
        .squeeze(0)?
        .to_vec1()?;
    let feature = normalize(&feature);
    let mut result = Vec::new();
    for (path, embedding) in database.iter() {
        let similarity = dot_product(embedding, &feature);
        result.push((path, similarity));
    }
    result.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());
    Ok(result)
}

fn command_add_image(database: &mut Database, path: &str, model: &model::ClipVisionTransformer) {
    let images = get_images(path);
    let len = images.len();
    for (i, image) in images.iter().enumerate() {
        if database.contains_key(image) {
            println!("skipping {}/{} {}", i + 1, len, image);
            continue;
        }
        println!("processing {}/{} {}", i + 1, len, image);
        add_image_feature(database, model, &image).expect("failed to add image");
    }
}

fn command_find_image(
    database: &Database,
    model: &model::ClipTextTransformer,
    tokenizer: &Tokenizer,
    text: &str,
) {
    let result = find_image(database, model, tokenizer, text).expect("failed to find image");
    for (i, (path, similarity)) in result.iter().enumerate() {
        println!("{:2} {:.4} {}", i, similarity, path);
    }
}

fn normalize(x: &[f32]) -> Vec<f32> {
    let sum: f32 = x.iter().map(|x| x * x).sum::<f32>().sqrt();
    x.iter().map(|x| x / sum).collect()
}

fn dot_product(x: &[f32], y: &[f32]) -> f32 {
    x.iter().zip(y).map(|(x, y)| x * y).sum()
}

fn api_get_image(
    _uri: HttpUri,
    _headers: HttpHeaders,
    params: HttpParams,
    _body: HttpBody
) -> Result<HttpResponse, Box<dyn Error>> {
    let image_path = params.get("path")
        .ok_or("missing parameter 'path'")?
        .trim();
    let content_type = if image_path.ends_with(".png") {
        "image/png"
    } else if image_path.ends_with(".jpeg") || image_path.ends_with(".jpg") {
        "image/jpeg"
    } else {
        return Err("invalid image path".into());
    };

    // if image_path.contains("..") {
    //     return Err("invalid image path".into());
    // }

    let path = Path::new(image_path);
    // if path.is_absolute() {
    //     return Err("invalid image path".into());
    // }

    let content = read(path)?;
    Ok(HttpResponse::builder()
        .set_code(200)
        .add_header("Content-Type", content_type)
        .set_payload_raw(content)
        .build())
}

fn main() -> tokenizer::Result<()> {
    let mut database = load_database();
    let arg1 = std::env::args().nth(1);
    let arg2 = std::env::args().nth(2);

    let arg1 = arg1.as_ref().map(String::as_str);
    if arg1 == Some("add") && arg2.is_some() {
        let weights =
            unsafe { candle_core::safetensors::MmapedFile::new("clip/model.safetensors")? };
        let weights = weights.deserialize()?;
        let vb = VarBuilder::from_safetensors(vec![weights], DType::F32, &Device::Cpu);
        let model = model::ClipVisionTransformer::new(vb, &model::Config::vision())?;
        command_add_image(&mut database, &arg2.unwrap(), &model);
        save_database(&database);
        return Ok(());
    } else if arg1 == Some("find") && arg2.is_some() {
        let weights =
            unsafe { candle_core::safetensors::MmapedFile::new("clip/model.safetensors")? };
        let weights = weights.deserialize()?;
        let vb = VarBuilder::from_safetensors(vec![weights], DType::F32, &Device::Cpu);
        let model = model::ClipTextTransformer::new(vb, &model::Config::clip())?;
        let tokenizer = Tokenizer::from_file("./clip/tokenizer.json")?;
        command_find_image(&database, &model, &tokenizer, &arg2.unwrap());
        return Ok(());
    } else if arg1 == Some("serve") && arg2.is_some() {
        let weights =
        unsafe { candle_core::safetensors::MmapedFile::new("clip/model.safetensors")? };
        let weights = weights.deserialize()?;
        let vb = VarBuilder::from_safetensors(vec![weights], DType::F32, &Device::Cpu);
        let model = model::ClipTextTransformer::new(vb, &model::Config::clip())?;
        let tokenizer = Tokenizer::from_file("./clip/tokenizer.json")?;

        let port = arg2.unwrap().parse::<u16>()?;
        let mut httpd = MinHttpd::new();

        let database = Arc::new(database);
        let model = Arc::new(model);
        let tokenizer = Arc::new(tokenizer);

        httpd.route_fn("/api/getImage", api_get_image);

        httpd.route("/api/search", Box::new(move |_, _, params, _| {
            let query_text = params.get("text")
                .ok_or("missing parameter 'text'")?
                .trim();

            let query_result = find_image(&database, &model, &tokenizer, query_text)
                .map_err(|e| format!("failed to query: {}", e))?;

            let first_50_items = query_result
                .iter()
                .take(50)
                .collect::<Vec<_>>();

            Ok(HttpResponse::builder()
                .set_code(200)
                .add_header("Content-Type", "application/json")
                .set_payload(serde_json::to_string(&first_50_items)?)
                .build())
        }));

        httpd.route_static(
            "",
            "text/html",
            include_str!("index.html").to_string()
        );

        println!("starting server at http://127.0.0.1:{}", port);
        httpd.serve(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), port))
            .unwrap();
    }
    println!("usage: clip add <path> | clip find <text> | clip serve <port>");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_images() {
        let images = get_images(".");
        println!("{:?}", images);
    }
}
