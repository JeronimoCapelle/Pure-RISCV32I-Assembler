mod lexical_analysis;
mod structures;
mod symbol_resolution;
mod syntax_analysis;

fn main() {
    let mut args = std::env::args().skip(1);
    let input_filename = args.next().expect("Filename not provided");

    let file_contents = std::fs::read_to_string(input_filename).expect("Failed to open file");

    let tokens = lexical_analysis::tokenize(&file_contents).unwrap();

    let (labels, tokens) = symbol_resolution::collect_symbols(&tokens).unwrap();

    let program = syntax_analysis::parse(&tokens, &labels).unwrap();
}
