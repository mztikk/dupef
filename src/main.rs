use clap::Parser;
use pathwalker::PathWalker;
use std::{
    collections::{hash_map::Entry, HashMap},
    fs, io,
    path::PathBuf,
};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// Input file
    #[clap(parse(from_os_str))]
    input: PathBuf,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    // println!("{:?}", args.input);

    let mut dupes: HashMap<u32, Vec<PathBuf>> = HashMap::new();

    // PathWalker::new(args.input).for_each(|x| println!("{:?}", x.path()));

    for entry in PathWalker::new(args.input)
        .map(|x| x.path())
        .filter(|x| x.is_file())
    {
        let file_content = fs::read(&entry).unwrap();
        let hash = xx_hash::xx_hash32(&file_content);

        match dupes.entry(hash) {
            Entry::Vacant(map_entry) => {
                map_entry.insert(vec![entry]);
            }
            Entry::Occupied(map_entry) => {
                map_entry.into_mut().push(entry);
            }
        };
    }

    // print all dupes
    for paths in dupes.values() {
        if paths.len() > 1 {
            println!("Duplicates:");
            println!("{:?}", paths);
            println!();
        }
    }

    Ok(())
}
