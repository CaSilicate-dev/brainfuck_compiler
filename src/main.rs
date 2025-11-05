use clap::Parser;
use std::io::Write;

#[derive(Parser, Debug)]
#[command(name = "brainfuck_compiler")]
struct Cli {
    /// Input file
    input: String,

    /// Output file
    output: String,

    /// Save C Code
    #[arg(short)]
    c: bool,

    /// Do not compile to binary
    #[arg(short)]
    d: bool,

    /// brainfuck tape length
    #[arg(short, default_value = "30000")]
    l: i32,
}
fn main() {
    let cli = Cli::parse();
    let input_file = cli.input;
    let output_file = cli.output;
    let save_c = cli.c;
    let compiletb = cli.d;
    let tlength = cli.l;

    let filename = input_file;
    let bfcode;
    match std::fs::read_to_string(filename) {
        Ok(c) => {
            bfcode = c;
        }
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
    let mut ccode = r#"
        #include <stdio.h>
        #include <stdlib.h>
        #define TAPE_SIZE "#.to_string();
    ccode += tlength.to_string().as_str();
    ccode += "\n";
    ccode +=r#"
        int main(){unsigned char tape[TAPE_SIZE] = {0};unsigned char *ptr = tape;
    "#;
    for (i, c) in bfcode.chars().enumerate() {
        match c {
            '>' => {
                ccode.push_str("++ptr;");
            }
            '<' => {
                ccode.push_str("--ptr;");
            }
            '+' => {
                ccode.push_str("++*ptr;");
            }
            '-' => {
                ccode.push_str("--*ptr;");
            }
            '.' => {
                ccode.push_str("putchar(*ptr);");
            }
            ',' => {
                ccode.push_str("*ptr = getchar();");
            }
            '[' => {
                ccode.push_str("while (*ptr){");
            }
            ']' => {
                ccode.push_str("}");
            }
            '\n' => {}
            _ => {
                eprintln!("Invalid char: {}:{}", i, c);
            }
        }
    }
    ccode.push_str(
        r#"return 0;}
    "#,
    );
    let mut file = std::fs::File::create("output.c").unwrap();
    file.write_all(ccode.as_bytes()).unwrap();

    if !compiletb {
        let command = std::process::Command::new("gcc")
            .arg("output.c")
            .arg("-o")
            .arg(format!("{}", output_file))
            .output()
            .expect("msg");
        if command.status.success() {
        } else {
            eprintln!(
                "Error to compile C code: {}",
                String::from_utf8_lossy(&command.stderr)
            );
        }
    } else {
        std::process::exit(0);
    }
    if !save_c {
        match std::fs::remove_file("output.c") {
            Ok(()) => {}
            Err(e) => {
                eprintln!("Failed to delete file: {}", e);
            }
        }
    }
}
