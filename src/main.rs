use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::path::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"--help".to_string()) {
        println!("print to stdout: dump_ztxt <png-path>");
        println!("writes the ztxt to a file in the same directory if not specified: dump_ztxt <png-path> -o <optional-path>");
        return;
    }
    if args.len() == 1 {
        println!("ERROR: No image specified");
        return;
    }
    if args.len() >= 2 {
        let image_path = &args[1];
        let path = Path::new(image_path);
        // Opening a png file that has a zTXt chunk
        let decoder = png::Decoder::new(File::open(path).unwrap());
        let reader = decoder.read_info().unwrap();
        // If the text chunk is before the image data frames, `reader.info()` already contains the text.
        let chunks = &reader.info().compressed_latin1_text;
        let text = chunks[0].get_text().unwrap();
        // by default print to stdout if no other args provided
        if args.len() >= 3 {
            if args[2] == "-o" {
                if args.len() >= 4 {
                    // output path was specified
                    let spec = &args[3];
                    let specified = Path::new(spec);
                    if specified.has_root() {
                        // provided absolute path
                        //println!("absolute");
                        write_to_file(specified, &text);
                    } else {
                        // provided path relative to the image dir
                        let new = path.parent().unwrap().join(spec);
                        //println!("relaitve {:?}", &new.to_str());
                        write_to_file(new.as_path(), &text);
                    }
                } else {
                    // provided no output path so we assume same directory/filename with diff extension
                    let mut new = path.to_path_buf();
                    PathBuf::set_extension(&mut new, "ztxt");
                    write_to_file(new.as_path(), &text);
                }
            }
        } else {
            println!("{}", text);
        }
    }
}

fn write_to_file(path: &Path, text: &String) {
    let mut file = File::create(path).unwrap();
    write!(file, "{}", text).unwrap();
}
