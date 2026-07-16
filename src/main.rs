use std::io::Write;

use pure_rv32i::compile_string;

fn main() {
    let mut args = std::env::args().skip(1);
    let Some(input_filename) = args.next() else {
        return;
    };

    let file_contents = match std::fs::read_to_string(input_filename) {
        Ok(a) => a,
        Err(a) => {
            println!("{a}");
            return;
        }
    };

    let binary = match compile_string(&file_contents) {
        Ok(a) => a,
        Err(a) => {
            println!("{a}");
            return;
        }
    };

    let mut file = match std::fs::File::create("output.bin") {
        Ok(a) => a,
        Err(a) => {
            println!("{a}");
            return;
        }
    };

    if let Err(a) = file.write_all(binary.as_slice()) {
        println!("{a}");
    }
}
