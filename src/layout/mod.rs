use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};

pub fn update_layout(canvas: &mut WindowCanvas, texture: &Texture) {
    yard(canvas, texture, 0, 0);
    yard(canvas, texture, 520, 0);
    yard(canvas, texture, 0, 520);
    yard(canvas, texture, 520, 520);
    let mut r1: Rect;
    let mut r2: Rect;

    r1 = Rect::new(48, 608, 16, 16);
    for j in 0..6 {
        for i in 0..18 {
            r2 = Rect::new(280 + 40 * j, 45 * i, 40, 45);
            canvas.copy(texture, r1, r2).unwrap();
        }
    }
    r1 = Rect::new(48, 608, 16, 16);
    for j in 0..6 {
        for i in 0..18 {
            r2 = Rect::new(45 * i, 280 + 40 * j, 45, 40);
            canvas.copy(texture, r1, r2).unwrap();
        }
    }

    for i in 0..20 {
        r1 = Rect::new(22, 640, 6, 15);
        r2 = Rect::new(397, 0 + 40 * i, 10, 40);
        canvas.copy(texture, r1, r2).unwrap();
    }
    for i in 0..20 {
        r1 = Rect::new(54, 640, 6, 15);
        r2 = Rect::new(0 + 40 * i, 397, 40, 10);
        canvas.copy(texture, r1, r2).unwrap();
    }
    canvas.present();
}

fn yard(canvas: &mut WindowCanvas, texture: &Texture, abs_x: i32, abs_y: i32) {
    // top left
    let mut src = Rect::new(208, 608, 16, 16);
    let mut dst = Rect::new(abs_x + 0, abs_y + 0, 32, 32);
    canvas.copy(texture, src, dst).unwrap();

    // top side
    src = Rect::new(272, 608, 16, 16);
    for i in 0..7 {
        dst = Rect::new(abs_x + 32 + i * 32, abs_y + 0, 32, 32);
        canvas.copy(texture, src, dst).unwrap();
    }

    // top right
    src = Rect::new(304, 608, 16, 16);
    dst = Rect::new(abs_x + 250, abs_y + 0, 32, 32);
    canvas.copy(texture, src, dst).unwrap();

    // left side
    src = Rect::new(208, 640, 16, 16);
    for i in 0..7 {
        dst = Rect::new(abs_x + 0, abs_y + 32 + i * 32, 32, 32);
        canvas.copy(texture, src, dst).unwrap();
    }

    // right side
    src = Rect::new(304, 640, 16, 16);
    for i in 0..7 {
        dst = Rect::new(abs_x + 250, abs_y + 32 + i * 32, 32, 32);
        canvas.copy(texture, src, dst).unwrap();
    }

    // bottom right
    src = Rect::new(304, 704, 16, 16);
    dst = Rect::new(abs_x + 250, abs_y + 250, 32, 32);
    canvas.copy(texture, src, dst).unwrap();

    // bottom side
    src = Rect::new(272, 704, 16, 16);
    for i in 0..7 {
        dst = Rect::new(abs_x + 32 + i * 32, abs_y + 250, 32, 32);
        canvas.copy(texture, src, dst).unwrap();
    }

    // bottom left
    src = Rect::new(208, 704, 16, 16);
    dst = Rect::new(abs_x + 0, abs_y + 250, 32, 32);
    canvas.copy(texture, src, dst).unwrap();

    //inside
    src = Rect::new(336, 640, 16, 16);
    for i in 0..7 {
        for j in 0..7 {
            dst = Rect::new(abs_x + 30 + j * 32, abs_y + 30 + i * 32, 32, 32);
            canvas.copy(texture, src, dst).unwrap();
        }
    }
}
