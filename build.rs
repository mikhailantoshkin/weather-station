use std::fs;
use std::path::Path;

fn main() {
    generate_icon_array();
    linker_be_nice();
    println!("cargo:rustc-link-arg=-Tdefmt.x");
    // make sure linkall.x is the last linker script (otherwise might cause problems with flip-link)
    println!("cargo:rustc-link-arg=-Tlinkall.x");
}

fn generate_icon_array() {
    let dest_path = Path::new("src/icons.rs");
    let icon_dir = Path::new("src/icons/");

    let mut generated_code = String::new();

    generated_code.push_str("pub static ICONS: &[(&str, &[u8])] = &[\n");

    if let Ok(entries) = fs::read_dir(icon_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(file_name) = path.file_name().and_then(|f| f.to_str()) {
                generated_code.push_str(&format!(
                    "    (\"{}\", include_bytes!(\"{}/{}\")),\n",
                    file_name, "icons", file_name
                ));
            }
        }
    }

    generated_code.push_str("];\n");

    fs::write(dest_path, generated_code).expect("Failed to write icons.rs");

    println!("cargo:rerun-if-changed={}", icon_dir.to_str().unwrap());
}

fn linker_be_nice() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        let kind = &args[1];
        let what = &args[2];

        match kind.as_str() {
            "undefined-symbol" => match what.as_str() {
                "_defmt_timestamp" => {
                    eprintln!();
                    eprintln!("💡 `defmt` not found - make sure `defmt.x` is added as a linker script and you have included `use defmt_rtt as _;`");
                    eprintln!();
                }
                "_stack_start" => {
                    eprintln!();
                    eprintln!("💡 Is the linker script `linkall.x` missing?");
                    eprintln!();
                }
                _ => (),
            },
            // we don't have anything helpful for "missing-lib" yet
            _ => {
                std::process::exit(1);
            }
        }

        std::process::exit(0);
    }

    println!(
        "cargo:rustc-link-arg=-Wl,--error-handling-script={}",
        std::env::current_exe().unwrap().display()
    );
}
