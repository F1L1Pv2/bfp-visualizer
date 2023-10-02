use ::core::slice::{self};
use raylib::prelude::*;
use std::io::prelude::*;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio, exit};

use brainfuck_plus_core::code_gen::generate_code;
use brainfuck_plus_core::parser::parse_file;
use brainfuck_plus_core::lexer::lex_file;

use brainfuck_plus_core::preprocess::preprocess_tokens;
use brainfuck_plus_core::prelude::*;

trait unsigned {
    fn print(&self) -> usize;
}

impl unsigned for u8 {
    fn print(&self) -> usize {
        *self as usize
    }
}

impl unsigned for u16 {
    fn print(&self) -> usize {
        *self as usize
    }
}

impl unsigned for u32 {
    fn print(&self) -> usize {
        *self as usize
    }
}

impl unsigned for u64 {
    fn print(&self) -> usize {
        *self as usize
    }
}

impl unsigned for usize {
    fn print(&self) -> usize {
        *self
    }
}

fn draw_array<T>(array: &[T], d: &mut RaylibDrawHandle, origin: Vector2)
where
    T: unsigned,
{
    let s_width = d.get_screen_width() as usize;
    let s_height = d.get_screen_height() as usize;


    let line_thick: i32 = 10;

    
    let s = (s_width as f32 / 1.5 ) / array.len() as f32;

    let padding = (s_width as f32 - line_thick as f32 * 2.) / s_width as f32;

    let offset_x = s * array.len() as f32 * padding/ 2.0;

    for (n, item) in array.iter().enumerate() {

        let x = s_width as f32 / 2.0 + (n as f32 * s) as f32 * padding - offset_x;
        let y = 200.0;

        d.draw_rectangle_lines_ex(Rectangle{
            x,
            y,
            width: s as f32,
            height: s as f32
        }, line_thick, Color::from_hex("FFFFFF").unwrap());

        d.draw_text(format!("{}",item.print()).as_str(), x as i32, y as i32, 20, Color::from_hex("FFFFFF").unwrap())

    }
}

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
        cell_count: MEM_SIZE,
    }];


    let tokens = lex_file(contents, filename.clone());
    // dbg!(&tokens);
    let tokens = preprocess_tokens(tokens, filename.clone(), path, includes, &mut tapes);
    let operations = parse_file(tokens, &tapes);

    // dbg!(&operations);

    let vs_path = Path::new(&operations[0].filename);
    let vs_content = fs::read_to_string(vs_path).unwrap();



    let window_size = (640, 480);
    let framerate = 60;

    let (mut rl, thread) = raylib::init()
        .size(window_size.0, window_size.1)
        .title("bfp-visualizer")
        .build();

    rl.set_target_fps(framerate);

    let background_color = Color::from_hex("000000").unwrap();
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

    let mut t = 0.0;

    let text = "baller".to_string();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        let (s_width, s_height) = (d.get_screen_width(), d.get_screen_height());

        d.clear_background(color1);

        d.draw_rectangle(t as i32, t as i32, s_width / 2, s_height / 2, color2);

        let mut out: String = String::new();

        for char in text.chars().take((t / 20.0) as usize) {
            out += char.to_string().as_str();
        }

        // d.draw_text(out.as_str(), s_width / 2, 50, 50, color3);

        t += 1.0;

        // draw_array(&[5 as u8, 2, 4, 3], &mut d, Vector2 { x: 0., y: 0. });

        d.draw_text(vs_content.as_str(), 0, 0, 10, foreground_color);


        drop(d);

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

    drop(process.stdin);

    std::thread::sleep(std::time::Duration::from_millis(500));
}
