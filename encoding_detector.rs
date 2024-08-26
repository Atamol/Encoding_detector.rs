use chardetng::EncodingDetector;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::path::Path;
use serde_json::json;
use std::env;

fn detect_encoding(file_path: &str) -> io::Result<serde_json::Value> {
    let file_size = std::fs::metadata(file_path)?.len();
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);

    let mut detector = EncodingDetector::new();
    let mut buffer = vec![0; 1024];  // 1KBのチャンクで読み込む

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {  // ファイルの終わりに到達
            break;
        }

        detector.feed(&buffer[..bytes_read], false);
        if detector.confident() {  // 確信が得られたらループを終了
            break;
        }
    }

    let encoding = detector.guess(None, true);

    Ok(json!({
        "file_size": file_size,
        "encoding": encoding.name(),
        "confidence": if detector.confident() { 1.0 } else { 0.5 }
    }))
}

fn main() -> io::Result<()> {
    println!("File path?");
    let mut file_path = String::new();
    io::stdin().read_line(&mut file_path)?;
    let file_path = file_path.trim();

    if !Path::new(file_path).exists() {
        eprintln!("Error: Invalid file path or access issue. Please try again.");
        std::process::exit(1);
    }

    match detect_encoding(file_path) {
        Ok(result) => println!("{}", result.to_string()),
        Err(e) => eprintln!("Error detecting encoding: {}", e),
    }

    Ok(())
}
