use crate::*;

#[derive(Debug, Clone)]
pub struct TapeArr {
    pub name: String,
    pub pointer: usize,
    pub size: brainfuck_plus_core::prelude::Size,
    pub vec: Vec<usize>,
}

impl TapeArr {
    pub fn new(name: String, size: brainfuck_plus_core::prelude::Size, count: usize) -> TapeArr {
        TapeArr {
            name,
            pointer: 0,
            size,
            vec: vec![0; count],
        }
    }
}

pub trait Unsigned {
    fn print(&self) -> usize;
}

impl Unsigned for u8 {
    fn print(&self) -> usize {
        *self as usize
    }
}

impl Unsigned for u16 {
    fn print(&self) -> usize {
        *self as usize
    }
}

impl Unsigned for u32 {
    fn print(&self) -> usize {
        *self as usize
    }
}

impl Unsigned for u64 {
    fn print(&self) -> usize {
        *self as usize
    }
}

impl Unsigned for usize {
    fn print(&self) -> usize {
        *self
    }
}

#[allow(dead_code)]
pub fn draw_tape_arr(tape_arr: &TapeArr,  d: &mut RaylibDrawHandle, origin: Vector2, cell_width: f32){
    draw_array(tape_arr.vec.as_slice(), d, origin, cell_width);

    // draw pointer
    let pointer_origin: Vector2 = Vector2::new(origin.x + tape_arr.pointer as f32 * cell_width, origin.y + cell_width);
    let pointer_origin: Vector2 = Vector2::new(pointer_origin.x + cell_width/2.0, pointer_origin.y + cell_width/2.0);

    // d.draw_rectangle(pointer_origin.x as i32, pointer_origin.y as i32, cell_width as i32, cell_width as i32, Color::RED);
    // d.draw_circle(pointer_origin.x as i32, pointer_origin.y as i32, cell_width/2.0, Color::RED);

    let v1: Vector2 = Vector2::new(pointer_origin.x - cell_width/2.5, pointer_origin.y + cell_width/2.5);
    let v2: Vector2 = Vector2::new(pointer_origin.x + cell_width/2.5, pointer_origin.y + cell_width/2.5);
    let v3: Vector2 = Vector2::new(pointer_origin.x, pointer_origin.y - cell_width/2.5);
    

    d.draw_triangle(v1, v2, v3, Color::RED);
}

#[allow(dead_code)]
pub fn draw_array<T>(array: &[T], d: &mut RaylibDrawHandle, origin: Vector2, cell_width: f32)
where
    T: Unsigned,
{
    let s_width = d.get_screen_width() as usize;
    // let s_height = d.get_screen_height() as usize;

    let line_thick: i32 = 5;

    // let s = (s_width as f32 / 1.5) / array.len() as f32;

    let s = cell_width;

    let padding = (s_width as f32 - line_thick as f32 * 2.) / s_width as f32;

    // let padding = 1.0;

    let offset_x = s * array.len() as f32 * padding / 2.0;

    for (n, item) in array.iter().enumerate() {
        let x = origin.x + (n as f32 * s) * padding;// - offset_x;
        let y = origin.y;

        d.draw_rectangle_lines_ex(
            Rectangle {
                x,
                y,
                width: s,
                height: s,
            },
            line_thick,
            Color::from_hex("FFFFFF").unwrap(),
        );

        let font_size: i32 = 25;

        let texty: String = format!("{}", item.print());

        let size_str = raylib::text::measure_text(texty.as_str(), font_size);

        d.draw_text(
            texty.as_str(),
            x as i32 + (s / 2.0) as i32 - size_str / 2,
            y as i32 + (s / 2.0) as i32 - font_size / 2,
            font_size,
            Color::from_hex("FFFFFF").unwrap(),
        );
    }
}
