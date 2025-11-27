use colored::*;
use regex::Regex;
use std::env;
use std::fs;
use std::fs::read_dir;
use std::io;
use std::io::Write;
use std::time::Instant;

mod toml_extract; // extract and print the version information according to the toml file

fn show_banner() {
    // banner ref: https://manytools.org/hacker-tools/ascii-banner/

    //logo design: "ticks", use "█" to replace "/\" chars, "_" replaced with space
    let banner = String::from(
        "

\t ███████╗   ██╗   ██╗        ███████╗   
\t ██╔════╝   ██║   ██║        ██╔════╝   
\t █████╗     ██║   ██║        █████╗     
\t ██╔══╝     ██║   ██║        ██╔══╝     
\t ██║        ██║   ███████╗   ███████╗   
\t ╚═╝        ╚═╝   ╚══════╝   ╚══════╝ 
\t 
\t ███████╗   ██╗   ███╗   ██╗   ██████╗    
\t ██╔════╝   ██║   ████╗  ██║   ██╔══██╗   
\t █████╗     ██║   ██╔██╗ ██║   ██║  ██║   
\t ██╔══╝     ██║   ██║╚██╗██║   ██║  ██║   
\t ██║        ██║   ██║ ╚████║   ██████╔╝   
\t ╚═╝        ╚═╝   ╚═╝  ╚═══╝   ╚═════╝    

A file location and content search tool.

",
    );

    // Print the banner in cyan color
    colour_print(&banner, "yellow")
}

// Function to print text in different colors
fn colour_print(text: &str, colour: &str) {
    match colour {
        "flush_green" => {
            print!("\x1b[2K\r"); // Clear the line and move to the beginning
            io::stdout().flush().unwrap();
            print!(" {}", text.bright_green().bold());
            io::stdout().flush().unwrap();
        }
        "green" => {
            print!("\x1b[2K\r"); // Clear the line and move to the beginning
            println!("{}", text.bright_green().bold());
        }
        "red" => {
            print!("\x1b[2K\r"); // Clear the line and move to the beginning
            println!("{}", text.bright_red().bold());
        }
        "cyan" => {
            print!("\x1b[2K\r"); // Clear the line and move to the beginning
            println!("{}", text.bright_cyan().bold());
        }
        "purple" => {
            print!("\x1b[2K\r"); // Clear the line and move to the beginning
            println!("{}", text.bright_purple().bold());
        }
        "blue" => {
            print!("\x1b[2K\r"); // Clear the line and move to the beginning
            println!("{}", text.bright_blue().bold());
        }
        "yellow" => {
            print!("\x1b[2K\r"); // Clear the line and move to the beginning
            println!("{}", text.bright_yellow().bold());
        }
        _ => {
            print!("\x1b[2K\r"); // Clear the line and move to the beginning
            println!("{}", text.bright_yellow().bold());
        }
    }
} // end of colour_print()

fn main() {
    // Print a welcome message
    show_banner();

    // Display version information from Cargo.toml
    toml_extract::main();

    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    // Show usage if not enough arguments are provided
    if args.len() < 2 {
        println!(
            "Usage: {} <filename_regex> [--help] [--search <string_regex>]",
            args[0]
        );
        return;
    }

    // Show help message if --help is present
    if args.contains(&"--help".to_string()) {
        println!(
            "Usage: {} <filename_regex> [--help] [--search <string_regex>]\n\nOptions:\n  --help         Show this help message\n  --search       Specify a regular expression to search for in the file\n\nDescription:\n  This program searches for files matching a specified filename regular expression in the directory tree and prints their paths and contents if found. If a search string regular expression is provided, it highlights the lines matching the regex in yellow.
            ",
            args[0]
        );
        let msg = format!(" See also: --bighelp\n\n");
        colour_print(&msg, "green");
        return;
    }
    // Show extended help if --bighelp is present
    if args.contains(&"--bighelp".to_string()) {
        bighelp();
        // println!("  Usages without regex:");
        // let msg = format!(r#"        {0} example.txt --search keystring"#, args[0]);
        // colour_print(&msg, "green");

        // println!("\n  Usages with regex:");
        // let msg = format!(
        //     r#"        {0} -- "example\\.txt" --search "keystring""#,
        //     args[0]
        // );
        // colour_print(&msg, "green");

        // println!("\n  Explanation:");
        // let msg = format!(
        //     r#" 
        // + The "example\\.txt" is a regular expression to match
        // the filename example.txt.

        // + The double backslash (\\) is used to escape the dot (.) 
        // in the filename.

        // + The "--search keystring" is used to specify the 
        // regular expression to search for the string keystring 
        // within the file.
        //     "#
        // );
        // colour_print(&msg, "yellow");

        // println!("  Simple commands:");
        // let msg = format!(
        //     r#"
        // + Find all `.rs` files containing the word `main`:
        // cargo run -- ".*\.rs$" --search main

        // + Search all files for a particular text
        // cargo run -- ".*" --search your_text_here
        // "#
        // );
        // colour_print(&msg, "green");

        // println!("  Search all files for a term:");
        // let msg = format!(
        //    r#"
        // + Search all files for a term:
        // Find all `.rs` files containing the word `main`:
        
        // cargo run -- ".*\.rs$" --search "term"
        // cargo run -- "\.rs"  --search "term"
        // "#);

        // //        \n\t file_find  \"\\.rs\"  --search \"term\"");
        // colour_print(&msg, "green");

        return;
    }

    // Compile the filename regex from the first argument
    let filename_regex = Regex::new(&args[1]).expect("Invalid filename regex");
    // If --search is present, compile the search regex from the next argument
    let search_regex = if let Some(index) = args.iter().position(|x| x == "--search") {
        args.get(index + 1)
            .map(|s| Regex::new(s).expect("Invalid search regex"))
    } else {
        None
    };

    // Start timer for search duration
    let start_time = Instant::now();
    // Get directory listing for current directory
    let paths = fs::read_dir("./").unwrap();

    // Recursively search files; print message if not found
    if !recurse_files(paths, &filename_regex, search_regex.as_ref()) {
        println!("  File not found");
    }

    // determine elapsed time
    let duration = start_time.elapsed();

    // Print search completion message with duration
    let msg = format!("  Search completed in {:.2?}", duration);
    colour_print(&msg, "purple");

    // println!("Search completed in {:.2?}", duration);
}

// Recursively search files and directories for matches
fn recurse_files(paths: fs::ReadDir, filename_regex: &Regex, search_regex: Option<&Regex>) -> bool {
    let mut found = false;
    for path in paths {
        let path = path.unwrap().path();
        // If path is a directory, recurse into it
        if path.is_dir() {
            let sub_paths = read_dir(path.clone()).unwrap();
            if recurse_files(sub_paths, filename_regex, search_regex) {
                found = true;
            }
        // If path is a file, check if filename matches regex
        } else if let Some(file_name) = path.file_name().and_then(|f| f.to_str()) {
            if filename_regex.is_match(file_name) {
                // Print the file path in yellow
                // moving this code below to print only when the search succeeds
                // let msg = format!("  {}", path.display());
                // colour_print(&msg, "yellow");

                // Try to read file contents
                let contents = fs::read_to_string(&path)
                    .unwrap_or_else(|_| "  Unable to read file contents".to_string());
                // If a search regex is provided, print matching lines
                if let Some(search) = search_regex {
                    for (i, line) in contents.lines().enumerate() {
                        if search.is_match(line) {
                // Print the file path in yellow
                let msg = format!("  {}", path.display());
                colour_print(&msg, "green");

                            println!("    Line {}: {}", i + 1, line.cyan().bold());
                        }
                    }
                } else {
                    // println!("Contents:\n{}", contents);
                }
                found = true;
            }
        }
    }
    found
}

// Print out the help message
fn bighelp() {
    println!("  This is a big help message.");
    println!("  It can contain more detailed information about the program.");
    println!("  You can customize it as needed.");

        println!("  Usages without regex:");
        let msg = format!(r#"         example.txt --search keystring"#);
        colour_print(&msg, "green");

        println!("\n  Usages with regex:");
        let msg = format!(
            r#"         -- "example\\.txt" --search "keystring""#,
        );
        colour_print(&msg, "green");

        println!("\n  Explanation:");
        let msg = format!(
            r#" 
        + The "example\\.txt" is a regular expression to match
        the filename example.txt.

        + The double backslash (\\) is used to escape the dot (.) 
        in the filename.

        + The "--search keystring" is used to specify the 
        regular expression to search for the string keystring 
        within the file.
            "#
        );
        colour_print(&msg, "yellow");

        println!("  Simple commands:");
        let msg = format!(
            r#"
        + Find all `.rs` files containing the word `term`:
        cargo run -- ".*\.rs$" --search main

        + Search all files for a particular text
        cargo run -- ".*" --search your_text_here
        "#
        );
        colour_print(&msg, "green");

        println!("  Search all files for a term:");
        let msg = format!(
           r#"
        + Search all files for a term:
        Find all `.rs` files containing the word `main`:
        
        cargo run -- ".*\.rs$" --search "term"
        cargo run -- "\.rs"  --search "term"
        "#);

        //        \n\t file_find  \"\\.rs\"  --search \"term\"");
        colour_print(&msg, "green");
    
}