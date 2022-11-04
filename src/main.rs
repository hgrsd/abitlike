use clap::Parser;

mod scored_word;

/// Fuzzy-search for a given term within a specified haystack file
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Target text to fuzzy-search for
    #[arg(short, long)]
    search_target: String,

    /// Haystack file
    #[arg(short, long)]
    file: String,

    /// How many matches to return
    #[arg(short, long, default_value_t = 10)]
    num_hits: i16,
}

fn main() {
    let args = Args::parse();
    let mut heap: std::collections::BinaryHeap<_> = std::collections::BinaryHeap::new();
    let file = std::fs::read_to_string(args.file).expect("unable to open file");
    for line in file.lines() {
        for word in line.split_whitespace() {
            let scored = scored_word::ScoredWord {
                score: strsim::levenshtein(&args.search_target.to_lowercase(), &word.to_lowercase()),
                word: word.to_string(),
            };
            heap.push(std::cmp::Reverse(scored))
        }
    }
    for _ in 0..args.num_hits {
        if let Some(result) = heap.pop() {
            println!("{:?}", result.0.word);
        }
    }
}
