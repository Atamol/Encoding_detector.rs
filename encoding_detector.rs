use chardetng::EncodingDetector;
use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use colored::*;
use ctrlc;

fn detect_encoding_sequential(file_path: &str) -> io::Result<String> {
    let file_size = std::fs::metadata(file_path)?.len();
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);

    let mut detector = EncodingDetector::new();
    let mut buffer = vec![0; 1024];  // 1KBのチャンクで読み込む
    let mut total_read = 0;

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {  // ファイルの終わりに到達
            break;
        }

        detector.feed(&buffer[..bytes_read], false);
        total_read += bytes_read as u64;

        if total_read as f64 / file_size as f64 >= 0.65 {  // 65%のデータが読み込まれたらループを終了
            break;
        }
    }

    let encoding = detector.guess(None, true);

    Ok(format!(
        "{{Encoding: \"{}\", File size: \"{} KB\"}}",
        encoding.name(),
        file_size / 1024
    ))
}

fn detect_encoding_full(file_path: &str) -> io::Result<String> {
    let file_size = std::fs::metadata(file_path)?.len();

    let mut file = File::open(file_path)?;
    let mut raw_data = Vec::new();
    file.read_to_end(&mut raw_data)?;

    let mut detector = EncodingDetector::new();
    detector.feed(&raw_data, true);
    let encoding = detector.guess(None, true);

    Ok(format!(
        "{{Encoding: \"{}\", File size: \"{} KB\"}}",
        encoding.name(),
        file_size / 1024
    ))
}

fn main() -> io::Result<()> {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
        println!("{}", "\nProcess interrupted by user. Exiting...".red());
        std::process::exit(0);
    }).expect("Error setting Ctrl-C handler");

    loop {
        println!("{}", "File path?".yellow());
        let mut file_path = String::new();
        io::stdin().read_line(&mut file_path)?;
        let file_path = file_path.trim();

        if !Path::new(file_path).exists() {
            eprintln!("{}", "Error: Invalid file path or access issue. Please try again.".red());
            continue;
        }

        let file_size = std::fs::metadata(file_path)?.len();
        let result = if file_size < 5_000_000 {  // 5MB未満の場合，全体を読み込む
            detect_encoding_full(file_path)
        } else {  // 5MB以上の場合，逐次的に読み込む
            detect_encoding_sequential(file_path)
        };

        match result {
            Ok(result) => {
                println!();
                println!("{}", result.cyan());
            },
            Err(e) => eprintln!("{}", format!("Error detecting encoding: {}", e).red()),
        }

        break;
    }

    Ok(())
}
