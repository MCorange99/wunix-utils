use clap::Parser;
use file_format::FileFormat;


#[derive(Debug, Parser)]
struct Args {
    /// File to inspect
    file: String
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let f_fmt = FileFormat::from_file(&args.file)?;

    println!("{}: {} {} {}", args.file, f_fmt.media_type(),  f_fmt.short_name().unwrap_or(""),  f_fmt.name());



    Ok(())
}
