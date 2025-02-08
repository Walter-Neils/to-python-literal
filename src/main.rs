use std::{
    fs::File,
    io::{stdin, Read},
};

fn bytes_to_python(bytes: &[u8]) -> String {
    // Format each byte into its hexadecimal representation, prefixed with '\x'
    let hex_parts: Vec<String> = bytes.iter().map(|b| format!("\\x{:02X}", b)).collect();
    format!("b'{}'", hex_parts.join(""))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();
    let include_print = args.iter().find(|x| *x == "--oneshot").is_some();
    let source = args.iter().skip(1).find(|x| !x.starts_with("--"));
    let mut data = Vec::new();

    if let Some(source) = source {
        let mut file = File::open(source)?;
        file.read_to_end(&mut data)?;
    } else {
        stdin().lock().read_to_end(&mut data)?;
    }

    if include_print {
        print!("python -c \"import sys; sys.stdout.buffer.write(");
    }
    print!("{}", bytes_to_python(&data));
    if include_print {
        print!(")\"");
    }
    Ok(())
}

