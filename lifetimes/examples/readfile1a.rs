// https://doc.rust-lang.org/rust-by-example/scope/lifetime/methods.html

use std::env;
use std::process;
use std::string::String;

use std::fs::File;
use std::io::BufReader;

use std::io;
use std::io::BufRead;
use std::io::BufWriter;
use std::io::Write;

use std::convert::TryInto;

#[derive(Debug)]
struct FileToVec<'a> {
    counter: i32,
    filename: &'a str,
    key: &'a Vec<u32>,
    value: &'a Vec<String>,
}

impl FileToVec {
    // Annotate lifetimes as in a standalone function.
    fn add_one<'a>(&'a mut self) {
        self.counter += 1;
    }

    fn print<'a>(&'a self) {
        println!("`print`: {}", self.counter);
    }

    fn is_even(num: u32) -> bool {
        (num) & 1 == 0
    }

    fn readfile<'a>(filename: String) {
        let f = File::open(filename).unwrap();
        let file = BufReader::new(&f);

        let mut mynum: &'a i32;
        let mut myline: &'a String;

        let mut vec_key = Vec::new();
        let mut vec_value = Vec::new();

        let mut writer = BufWriter::new(io::stdout());
        for (mynum, myline) in file.lines().enumerate() {
            if is_even(mynum.try_into().unwrap()) {
                writeln!(writer, "{0}\n", mynum).unwrap();
                //writeln!(writer, "{0}\n", myline).unwrap();
                vec_key.push(&mynum);
            }
            if !is_even(mynum.try_into().unwrap()) {
                writeln!(writer, "{0}\n", mynum).unwrap();
                //writeln!(writer, "{0}\n", myline).unwrap();
                vec_value.push(&myline);
            }
        }

        for key in vec_key.iter() {
            println!("{}", key);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("You need to enter a filename");
        process::exit(1);
    }
    let filename = &args[1];
    println!("In file {}", filename);

    // Instantiate a FileToVec
    let ftv: FileToVec = FileToVec {
        filename: filename,
        counter: 0,
        key: &Vec::new(),
        value: &Vec::new(),
    };

    let _contents = FileToVec::readfile(filename.to_string());
}
