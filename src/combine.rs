use std::{path::Path, fs::File, io::{self, Write, Read}};

/// Combine 1 or more text files split by lines
pub fn combine(wordlists: Vec<String>,
           remove_duplicates: bool,
           sort: bool,
           output: Option<String>,
           delimiter: Option<String>) {
    let mut out_writer = match output {
      Some(x) => {
        let path = Path::new(&x);
        Box::new(File::create(&path).unwrap()) as Box<dyn Write>
      }
      None => Box::new(io::stdout()) as Box<dyn Write>,
    };
    // Combine the wordlists
    let mut wordlist = String::new();
    for wordlist_path in wordlists {
      //File::open(wordlist_path).unwrap();
      let mut file = match File::open(wordlist_path) {
        Ok(x) => x,
        Err(e) => {
          match e.kind() {
            io::ErrorKind::NotFound => {
              eprintln!("File not found: {}", e);
              std::process::exit(1);
            },
            _ => {
              eprintln!("Error: {}", e);
              std::process::exit(1);
            }
          }
        }
      };
      let mut contents = String::new();
      file.read_to_string(&mut contents).unwrap();
      wordlist.push_str(&contents);
    }
    // Remove duplicates
    let delim = delimiter.unwrap_or("\n".to_string());
    let mut wordlist: Vec<&str> = wordlist.split(&delim).collect();
    if remove_duplicates {
      wordlist.sort();
      wordlist.dedup();
    }
    // Sort the wordlist
    if sort {
      wordlist.sort();
    }
    // Write the wordlist to a file or stdout
    out_writer.write(wordlist.join("\n").as_bytes()).unwrap();
  }
