use std::{env::args, process::exit};

const VERSION:&str = "0.1.0";

fn main() {
    let args:Vec<String> = args().collect();
    
    if args.len() != 1 {
        exit(1);
    }

    println!("a2v {VERSION}");
    println!("A simple tool (that uses ffmpeg under the hood)\nto convert mp3 audio to mp4 video.");
    loop {
        println!("Enter mp3 file name:");
        let mp3_file = input();
        if mp3_file.is_empty() {
            continue;
        }
        println!("Enter output mp4 file name:");
        let mp4_file = input();
        if mp4_file.is_empty() {
            continue;
        }
        println!("Enter image file name(optional):");
        let image = input();

        a2v(&mp3_file, &mp4_file, &image);

    }
}

fn input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("internal error!");
    input.trim().to_string()
}

fn a2v(mp3_file_name:&str, mp4_file_name:&str, image_file_name:&str){
    dbg!(mp3_file_name);
    dbg!(mp4_file_name);
    dbg!(image_file_name);
}