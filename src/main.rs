use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    plaintext: String,

    #[arg(short, long)]
    key: String,
}

fn main() {
    let args = Args::parse();

    let plaintext = args.plaintext;
    let key = args.key;

    let encrypted = xor(&plaintext, &key);
    let decrypted = xor(&encrypted, &key);

    println!("Encrypted: {:?}", encrypted.as_bytes());
    println!("Decrypted: {}", decrypted);
}

fn xor(plaintext: &str, key: &str) -> String {
    let result: Vec<u8> = plaintext
        .as_bytes()
        .iter()
        .zip(key.as_bytes().iter().cycle())
        .map(|(a, b)| a ^ b)
        .collect();

    return String::from_utf8(result).unwrap();
}
