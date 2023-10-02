use ::core::panic;
use ::core::slice::{self};
use raylib::prelude::*;
use std::{env, cell};
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::Write;
use std::path::Path;
use std::process::{exit, Command, Stdio};

use brainfuck_plus_core::lexer::lex_file;
use brainfuck_plus_core::parser::parse_file;

use brainfuck_plus_core::prelude::*;
use brainfuck_plus_core::preprocess::preprocess_tokens;

mod prelude;
use prelude::*;

fn usage(filename: String) {
    let mut arr = filename.split('/').collect::<Vec<&str>>();
    arr.reverse();

    println!("USAGE: {} <options> <filename>", arr[0]);
    println!("-i | -I include folder path (add only one)");
    println!("-l | -L libs folder path (add only one)");
    println!("-o | -O out file name (if not provided bf+ will use name of file)");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut arg_i: usize = 1;
    let argc: usize = args.len();
    let mut filename: String = String::new();
    let mut out_file_path: String = String::new();

    let mut includes: Vec<String> = Vec::new();
    let mut libs: Vec<String> = Vec::new();

    let std_lib_path = {
        let arr = args[0].split('/').collect::<Vec<&str>>();
        let len = arr.len() - 1;
        let mut out = String::new();

        for folder in arr.iter().take(len) {
            out += folder;
            out += "/";
        }

        out
    };

    includes.push(std_lib_path);

    // dbg!(std_lib_path);

    // exit(1);

    // if args.len() < 2 {
    //     println!("USAGE: {} <filename>", args[0]);
    //     exit(1);
    // }

    while arg_i < argc {
        let arg = args[arg_i].clone();

        match arg.as_str() {
            "-I" => {
                arg_i += 1;
                if arg_i < argc {
                    includes.push(args[arg_i].clone())
                }
            }

            "-i" => {
                arg_i += 1;
                if arg_i < argc {
                    includes.push(args[arg_i].clone())
                }
            }

            "-l" => {
                arg_i += 1;
                if arg_i < argc {
                    libs.push(args[arg_i].clone())
                }
            }

            "-L" => {
                arg_i += 1;
                if arg_i < argc {
                    libs.push(args[arg_i].clone())
                }
            }

            "-o" => {
                arg_i += 1;
                if arg_i < argc {
                    out_file_path = args[arg_i].clone();
                }
            }

            "-O" => {
                arg_i += 1;
                if arg_i < argc {
                    out_file_path = args[arg_i].clone();
                }
            }

            _ => {
                filename = arg.clone();
            }
        }

        arg_i += 1;
    }

    if !libs.is_empty() {
        usage(args[0].clone());
        println!("Libs are not currently implemented");
        exit(1);
    }

    if filename == String::new() {
        usage(args[0].clone());
        println!("Filename Wasn't provided");
        exit(1);
    }

    if out_file_path == String::new() {
        out_file_path = filename.replace(".bf", "");
    }

    //check if extension is .bf
    if !filename.ends_with(".bf") {
        println!("Brain fuck plus files must have .bf extension");
        exit(1);
    }

    let contents =
        fs::read_to_string(filename.clone()).expect("Something went wrong reading the file");

    let path: String = {
        let mut temp = String::new();
        let arr = filename.split('/').collect::<Vec<&str>>();
        let len = arr.len();

        for folder in arr.iter().take(len - 1) {
            temp += folder;
            temp += "/";
        }

        temp
    };

    // dbg!(path);

    let mut file_content: String = String::new();

    let mut tapes: Vec<Tape> = vec![Tape {
        name: "main".to_string(),
        size: Size::Byte,
        cell_count: 1024,
    }];

    let tokens = lex_file(contents, filename.clone());
    // dbg!(&tokens);
    let tokens = preprocess_tokens(tokens, filename, path, includes, &mut tapes);
    let operations = parse_file(tokens, &tapes);

    let mut tape_arrays: Vec<TapeArr> = Vec::new();

    for tape in tapes{
        let tapearr = TapeArr::new(tape.name.clone(),tape.size.clone(), tape.cell_count);
        // println!("{}", tapearr.vec.len());
        // exit(1);
        tape_arrays.push(tapearr);
    }

    // dbg!(&tape_arrays);

    // exit(1);

    // dbg!(&operations);

    let vs_path = Path::new(&operations[0].filename);
    let vs_content = fs::read_to_string(vs_path).unwrap();

    let window_size = (640, 480);
    let framerate = 60;

    let (mut rl, thread) = raylib::init()
        .size(window_size.0, window_size.1)
        .title("bfp-visualizer")
        .msaa_4x()
        .build();

    rl.set_target_fps(framerate);

    // let background_color = Color::from_hex("000000").unwrap();
    let background_color = Color::from_hex("181818").unwrap();
    let foreground_color = Color::from_hex("FFFFFF").unwrap();

    //https://www.canva.com/colors/color-wheel/

    let color1 = Color::from_hex("4EB1A0").unwrap();
    let color2 = Color::from_hex("6F4EB1").unwrap();
    let color3 = Color::from_hex("B14E5F").unwrap();
    let color4 = Color::from_hex("90B14E").unwrap();

    let mut process = match Command::new("ffmpeg")
        .args([
            "-loglevel",
            "verbose",
            "-y",
            "-f",
            "rawvideo",
            "-pix_fmt",
            "rgba",
            "-s",
            format!("{}x{}", window_size.0, window_size.1).as_str(),
            "-r",
            format!("{}", framerate).as_str(),
            "-i",
            "-",
            "-c:v",
            "libx264",
            "-vb",
            "2500k",
            "-c:a",
            "aac",
            "-ab",
            "200k",
            "-pix_fmt",
            "yuv420p",
            "output.mp4",
        ])
        .stdin(Stdio::piped())
        .spawn()
    {
        Err(why) => panic!("Couldn't execute ffmpeg: {}", why),
        Ok(child) => child,
    };

    let child_stdin = process.stdin.as_mut().unwrap();

    let mut record: bool = false;

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.draw_text(vs_content.as_str(), 0, 0, 15, Color::BLUE);
        
        let (s_width, s_height) = (d.get_screen_width(), d.get_screen_height());

        d.clear_background(background_color);

        if d.is_key_pressed(KeyboardKey::KEY_SPACE) {
            record = true;
        }

        if d.is_key_pressed(KeyboardKey::KEY_D) {
            tape_arrays[0].pointer += 1;
        }

        if d.is_key_pressed(KeyboardKey::KEY_A) {
            tape_arrays[0].pointer -= 1;
        }

        if d.is_key_pressed(KeyboardKey::KEY_W) {
            let pointer = tape_arrays[0].pointer;
            tape_arrays[0].vec[pointer] += 1;
        }
        if d.is_key_pressed(KeyboardKey::KEY_S) {
            let pointer = tape_arrays[0].pointer;
            tape_arrays[0].vec[pointer] -= 1;
        }

        // draw_array(&[5, 2, 4, 3], &mut d, Vector2 { x: 0., y: 0. });

        // draw_tape_arr(&tape_arrays[0], &mut d, Vector2 { x: 10.0, y: 200.0 }, 50.0);

        let cell_width = 50.0;
        let tapes_origin: Vector2 = Vector2::new(10.0, 50.0);

        for (n,tape_arr) in tape_arrays.iter().enumerate(){
            draw_tape_arr(tape_arr, &mut d, Vector2::new(tapes_origin.x, tapes_origin.y + n as f32 * cell_width*2.0), cell_width)
        }



        drop(d);

        if record {
            let img = rl.get_screen_data(&thread);

            let buf: &[u8] = unsafe {
                slice::from_raw_parts(
                    img.data as *const u8,
                    s_width as usize * s_height as usize * 4,
                )
            };

            child_stdin.write_all(buf).unwrap();

            child_stdin.flush().unwrap();
        }
    }

    drop(process.stdin);

    std::thread::sleep(std::time::Duration::from_millis(500));
}
