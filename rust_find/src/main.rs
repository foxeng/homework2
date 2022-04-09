mod find;

use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    name = "rust_find",
    about = "A command line utility for searching for files with regexes"
)]
struct Opt {
    /// List of directories to search in.
    #[structopt(short, long, required = true)]
    dirs: Vec<String>,

    /// List of patterns to use.
    #[structopt(short, long, required = true)]
    patterns: Vec<String>,

    /// Match files above size <size>.
    #[structopt(short, long, default_value = "0")]
    size: u64,
}

fn main() {
    let opt = Opt::from_args();

    for dir in &opt.dirs {
        let file = match find::File::from(dir) {
            Err(e) => {
                eprintln!("get metadata for {}: {}", dir, e);
                continue;
            }
            Ok(file) => file,
        };
        for f in find::traverse(&file, &opt.patterns, opt.size) {
            println!("{}", f.path);
        }
    }
}
