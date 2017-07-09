use std::env;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Write;
use std::io::BufReader;
use std::io::BufRead;


fn main() {
    let mut owrp_ifile:Option<File> = None;
    let mut owrp_ofile:Option<File> = None;

    let mut owrp_ifilename:Option<String> = None;
    let mut owrp_ofilename:Option<String> = None;

    let args: Vec<String> = env::args().collect();

    let mut i = 1;

    while i+1 < args.len() {
        let arg_key = &args[i];
        let arg_value = args[i+1].clone();

        match arg_key.as_ref() {
            "-i" => { owrp_ifilename = Some(arg_value); }
            "-o" => { owrp_ofilename = Some(arg_value); }
            _ => { panic!("Unrecognized argument!"); }
        }

        i += 2;
    }

    if let Some(filename) = owrp_ifilename {
        match File::open(&filename) {
            Err(why) => panic!("Couldn't open file \"{}\": {}", filename, why.description()),
            Ok(file) => { owrp_ifile = Some(file) },
        };
    }

    if let Some(filename) = owrp_ofilename {
        match File::create(&filename) {
            Err(why) => panic!("Couldn't create {}: {}", filename, why.description()),
            Ok(file) => { owrp_ofile = Some(file) },
        }
    }

    match owrp_ifile {
        Some(ifile) => {
            let ifilebuf = BufReader::new(&ifile);

            match owrp_ofile {
                Some(mut ofile) => {
                    for owrp_line in ifilebuf.lines() {
                        let str_line = match owrp_line {
                            Err(why) => panic!("Couldn't read from file! Reason: {}", why),
                            Ok(line) => line
                        };

                        let line: String = str_line.to_string();

                        match ofile.write(line.as_bytes()) {
                            Err(e) => panic!("Can't write to file! Reason: {}", e),
                            Ok(_) => {}
                        }
                        match ofile.write(b"\r\n") {
                            Err(why) => panic!("Can't write to file! Reason: {}", why),
                            Ok(_) => {}
                        }
                    }
                },
                None => {
                    let stdout = io::stdout();
                    let mut handle = stdout.lock();

                    for line in ifilebuf.lines() {
                        let str_line = match line {
                            Err(why) => panic!("Can't read from file! Reason: {}", why),
                            Ok(line) => line
                        };

                        let line: String = str_line.to_string();

                        match handle.write(line.as_bytes()) {
                            Err(why) => panic!("Can't write to STDOUT! Reason: {}", why),
                            Ok(_) => {}
                        };
                        match handle.write(b"\r\n") {
                            Err(why) => panic!("Can't write to STDOUT! Reason: {}", why),
                            Ok(_) => {}
                        }
                    }
                }
            }
        },
        None => {
            let stdin = io::stdin();

            match owrp_ofile {
                Some(mut ofile) => {
                    for line in stdin.lock().lines() {
                        let str_line = match line {
                            Err(why) => panic!("Can't read from STDIN! Reason: {}", why),
                            Ok(line) => line
                        };

                        let line: String = str_line.to_string();

                        match ofile.write(line.as_bytes()) {
                            Err(e) => panic!("Can't write to file! Reason: {}", e),
                            Ok(_) => {}
                        }
                        match ofile.write(b"\r\n") {
                            Err(why) => panic!("Can't write to file! Reason: {}", why),
                            Ok(_) => {}
                        }
                    }
                },
                None => {
                    let stdout = io::stdout();
                    let mut handle = stdout.lock();

                    for line in stdin.lock().lines() {
                        let str_line = match line {
                            Err(why) => panic!("Couldn't read from STDIN! Reason: {}", why),
                            Ok(line) => line
                        };

                        let line: String = str_line.to_string();

                        match handle.write(line.as_bytes()) {
                            Err(why) => panic!("Couldn't write to STDOUT! Reason: {}", why),
                            Ok(_) => {}
                        }
                        match handle.write(b"\r\n") {
                            Err(why) => panic!("Can't write to STDOUT! Reason: {}", why),
                            Ok(_) => {}
                        }
                    }
                }
            }
        }
    }
}
