use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    pub city: String,
}

impl Args {
    pub fn new() -> Self {
        let args = Args::parse();
        println!("City, {}!", args.city);
        args
    }
}
