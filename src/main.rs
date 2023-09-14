mod model;
use tokenizers::tokenizer::{Result, Tokenizer};

fn main() -> Result<()> {
    let mut tokenizer = Tokenizer::from_file("./clip/tokenizer.json")?;
    let encoding = tokenizer.encode("Hey there!", true)?;
    println!("{:?}", encoding.get_ids());
    Ok(())
}
