use clap::Parser;
use std::{error::Error, fs::File, io, path::Path};
use walkdir::WalkDir;
use zip::{ZipWriter, write::SimpleFileOptions};
#[derive(Parser, Debug)]
struct Args {
    command: String,
    target: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match args.command.as_str() {
        "zip" => {
            let src = Path::new(&args.target);
            let name = src.file_name().ok_or("Invalid path")?.to_string_lossy();

            let out = File::create(format!("{}.zip", name))?;
            let mut zip = ZipWriter::new(out);
            let opts = SimpleFileOptions::default();
            println!("Zipping {} into {}.zip", src.display(), name);
            if src.is_dir() {
                for entry in WalkDir::new(src) {
                    let entry = entry?;
                    let path = entry.path();
                    let rel = path.strip_prefix(src)?;
                    if rel.as_os_str().is_empty() {
                        continue;
                    }

                    let zip_name = format!("{}/{}", name, rel.to_string_lossy().replace('\\', "/"));
                    if path.is_dir() {
                        zip.add_directory(zip_name, opts)?;
                    } else {
                        zip.start_file(zip_name, opts)?;
                        io::copy(&mut File::open(path)?, &mut zip)?;
                    }
                }
            } else {
                zip.start_file(&*name, opts)?;
                io::copy(&mut File::open(src)?, &mut zip)?;
            }

            zip.finish()?;
            println!("Created {}.zip", name);
        }
        "unzip" => {
            let mut src = std::path::PathBuf::from(&args.target);
            if !src.is_file() {
                let with_ext = std::path::PathBuf::from(format!("{}.zip", args.target));
                if with_ext.is_file() {
                    src = with_ext;
                } else {
                    return Err(format!("'{}' is not a zip file", args.target).into());
                }
            }

            let name = src.file_stem().ok_or("Invalid path")?;
            let mut zip = zip::ZipArchive::new(File::open(&src)?)?;
            println!("Unzipping {} into {}", src.display(), name.display());

            for i in 0..zip.len() {
                let mut file = zip.by_index(i)?;
                let Some(rel) = file.enclosed_name() else {
                    continue;
                };
                let outpath = Path::new(name).join(rel);

                if file.is_dir() {
                    std::fs::create_dir_all(&outpath)?;
                } else {
                    if let Some(p) = outpath.parent() {
                        std::fs::create_dir_all(p)?;
                    }
                    io::copy(&mut file, &mut File::create(&outpath)?)?;
                }
            }
            println!("Unzipped {} into {}", src.display(), name.display());
        }
        _ => println!("Invalid command. Use 'zip' or 'unzip'."),
    }
    Ok(())
}
