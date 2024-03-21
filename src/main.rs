use std::f32::consts::PI;

use macroquad::{color, prelude::*};

#[macroquad::main("BasicShapes")]

async fn main() {
    let phi: f32 = (1.0 + (5.0_f32).powf(0.5)) / 2.0;
    let icovert = [
        (phi, 1.0, 0.0),
        (phi, -1.0, 0.0),
        (-phi, -1.0, 0.0),
        (-phi, 1.0, 0.0),
        (1.0, 0.0, phi),
        (-1.0, 0.0, phi),
        (-1.0, 0.0, -phi),
        (1.0, 0.0, -phi),
        (0.0, phi, 1.0),
        (0.0, phi, -1.0),
        (0.0, -phi, -1.0),
        (0.0, -phi, 1.0),
    ];
    let col = [
        BEIGE, BLACK, BLANK, BLUE, BROWN, DARKBLUE, DARKBROWN, DARKGRAY, DARKGREEN, DARKPURPLE,
        GOLD, GRAY, GREEN, LIGHTGRAY, LIME, MAGENTA, MAROON, ORANGE, PINK, PURPLE, RED, SKYBLUE,
        VIOLET, WHITE, YELLOW,
    ];
    let mut offset: f32 = 0.0;
    let mut scaling: f32 = 1.0;
    let mut out_scale: f32 = 1.0;
    loop {
        clear_background(GRAY);
        if is_key_down(KeyCode::Up) {
            scaling += 0.05;
        }
        if is_key_down(KeyCode::Down) {
            scaling -= 0.05;
        }

        if is_key_down(KeyCode::LeftShift) {
            out_scale += 0.25;
        }

        if is_key_down(KeyCode::RightShift) {
            out_scale -= 0.25;
        }

        if is_key_down(KeyCode::Right) {
            offset += 0.05;
        }
        if is_key_down(KeyCode::Left) {
            offset -= 0.05;
        }

        let pos: Vec2 = mouse_position().into();
        let centrepos: Vec2 = Vec2 {
            x: (screen_width() / 2.0),
            y: (screen_height() / 2.0),
        };

        let x: f32 = pos.x;
        let y: f32 = pos.y;
        let r_o: f32 = screen_height() / 2.0;
        let r_i: f32 =
            ((pos.x - centrepos.x).powf(2.0) + (pos.y - centrepos.y).powf(2.0)).powf(0.5);

        for i in icovert {
            for j in 0..12 {
                if ((i.0 + icovert[j].0).powf(2.0)
                    + (i.1 + icovert[j].1).powf(2.0)
                    + (i.2 + icovert[j].2).powf(2.0))
                .powf(0.5)
                    > 2.510
                {
                    line_proj2(
                        i.0 * 50.0,
                        i.1 * 50.0,
                        i.2 * 50.0,
                        icovert[j].0 * 50.0,
                        icovert[j].1 * 50.0,
                        icovert[j].2 * 50.0,
                        offset,
                        scaling,
                        col[j],
                    );
                };
            }
        }
        /*/
        for i in 1..12 {
            //print!("{}", icovert[1].0);
            line(
                (icovert[i].0 * (1.0 + offset) + scaling.cos() * icovert[i].2) * 50.0,
                (icovert[i].1 * (1.0 + offset) + scaling.sin() * icovert[i].2) * 50.0,
                (icovert[i + 1].0 * (1.0 + offset) + scaling.cos() * icovert[i + 1].2) * 50.0,
                (icovert[i + 1].1 * (1.0 + offset) + scaling.sin() * icovert[i + 1].2) * 50.0,
                2.0,
                RED,
            );
        }
        */
        /*        let a = 50.0 * (1.0 + offset) + scaling.cos() * 50.0;
        let b = 50.0 * (1.0 + offset) + scaling.sin() * 50.0;
        let c = 50.0 / (1.0 + offset) - scaling.cos() * 50.0;
        let d = 50.0 / (1.0 + offset) - scaling.sin() * 50.0;

        line(a, b, -a, b, 2.0, RED);
        line(a, b, a, -b, 2.0, RED);
        line(-c, d, c, d, 2.0, RED);
        line(c, -d, c, d, 2.0, RED);

        line(-a, -b, -a, b, 2.0, BLUE);
        line(-a, -b, a, -b, 2.0, RED);
        line(-c, -d, -c, d, 2.0, RED);
        line(-c, -d, c, -d, 2.0, RED);

        line(a, b, c, d, 2.0, RED);
        line(-a, b, -c, d, 2.0, RED);
        line(-a, -b, -c, -d, 2.0, RED);
        line(a, -b, c, -d, 2.0, RED);
        //line_proj(50.0, 50.0, 50.0, -50.0, -50.0, -50.0, 0.2, RED);
         */
        next_frame().await;
    }
}

fn line(x1: f32, y1: f32, x2: f32, y2: f32, thicc: f32, colour: Color) {
    let x1 = x1 + screen_width() / 2.0;
    let x2 = x2 + screen_width() / 2.0;
    let y1 = y1 + screen_height() / 2.0;
    let y2 = y2 + screen_height() / 2.0;
    draw_line(x1, y1, x2, y2, thicc, colour);
}

fn line_proj(x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32, sf: f32, colour: Color) {
    let x1 = (x1 + 1.0) * (z1 * sf).powf(z1 / (z1.abs()));
    let x2 = (x2 + 1.0) * (z2 * sf).powf(z2 / (z2.abs()));
    let y1 = (y1 + 1.0) * (z1 * sf).powf(z1 / (z1.abs()));
    let y2 = (y2 + 1.0) * (z2 * sf).powf(z2 / (z2.abs()));
    line(x1, y1, x2, y2, 2.0, colour);
}

fn line_proj2(
    x1: f32,
    y1: f32,
    z1: f32,
    x2: f32,
    y2: f32,
    z2: f32,
    yrot: f32,
    xrot: f32,
    colour: Color,
) {
    let a: f32 = 0.0; //PI/3.0;
    let b: f32 = -0.0; //PI/3.0;
    let theta: f32 = yrot;
    let phi: f32 = xrot;

    let x1 = x1 * (theta.cos()) + z1 * (theta.sin());
    let x2 = x2 * (theta.cos()) + z2 * (theta.sin());
    let y1 = y1 * (phi.cos()) - z1 * phi.sin();
    let y2 = y2 * (phi.cos()) - z2 * phi.sin();

    //OVERCOMPLICATED/ can be greatly reduced
    //let x1 = x1 * (theta.cos() * b.cos() + a.cos() * b.sin() * theta.sin()) - y1 * (a.sin() * theta.sin()) + z1 * (a.cos() * b.cos() * theta.sin() - b.sin() * b.cos());
    //let x2 = x2 * (theta.cos() * b.cos() + a.cos() * b.sin() * theta.sin()) - y2 * (a.sin() * theta.sin()) + z2 * (a.cos() * b.cos() * theta.sin() - b.sin() * b.cos());
    //let y1 = x1 * (a.sin() * b.sin()) + y1 * (b.cos()) + z1 * (a.sin() * b.cos());
    //let y2 = x2 * (a.sin() * b.sin()) + y2 * (b.cos()) + z2 * (a.sin() * b.cos());
    //let x1 = x1 * (b.cos() * theta.cos() - b.sin() * theta.sin()) + z1 * (theta.sin() * b.cos() - b.sin() * theta.cos());
    //let x2 = x2 * (b.cos() * theta.cos() - b.sin() * theta.sin()) + z2 * (theta.sin() * b.cos() - b.sin() * theta.cos());
    //let y1 = x1 * (theta.cos() * a.sin() * b.sin() + a.sin() * b.cos() * theta.sin())
    //    + y1 * (a.cos() * (1.0 - theta.cos()))
    //    + z1 * (theta.sin() * a.sin() * b.sin() + theta.cos() * a.sin() * b.cos())
    //    + a.cos() * theta.cos();
    //let y2 = x2 * (theta.cos() * a.sin() * b.sin() + a.sin() * b.cos() * theta.sin())
    //    + y2 * (a.cos() * (1.0 - theta.cos()))
    //    + z2 * (theta.sin() * a.sin() * b.sin() + theta.cos() * a.sin() * b.cos())
    //    + a.cos() * theta.cos();
    line(x1, y1, x2, y2, 2.0, colour);
}
/*
                    draw_line(
                        (x * c + centrepos.x * d) / (c + d)
                            + phi.to_radians().cos() * (r_i * c + r_o * d) / (c + d),
                        (y * c + centrepos.y * d) / (c + d)
                            + phi.to_radians().sin() * (r_i * c + r_o * d) / (c + d),
                        (x * a + centrepos.x * b) / (a + b)
                            + theta.to_radians().cos() * (r_i * a + r_o * b) / (a + b),
                        (y * a + centrepos.y * b) / (a + b)
                            + theta.to_radians().sin() * (r_i * a + r_o * b) / (a + b),
                        2.0,
                        col[j],
                    );
                } else {
                    draw_line(
                        (x * c + centrepos.x * d) / (c + d)
                            + theta.to_radians().cos() * (r_i * c + r_o * d) / (c + d),
                        (y * c + centrepos.y * d) / (c + d)
                            + theta.to_radians().sin() * (r_i * c + r_o * d) / (c + d),
                        (x * a + centrepos.x * b) / (a + b)
                            + phi.to_radians().cos() * (r_i * a + r_o * b) / (a + b),
                        (y * a + centrepos.y * b) / (a + b)
                            + phi.to_radians().sin() * (r_i * a + r_o * b) / (a + b),
                        2.0,
                        col[j],
                    );
                }
*/

/*for i in 0..72 {
    let theta: f32 = 5.0 * i as f32;
    let phi: f32 = theta + 5.0 * offset.sin();
    for j in 2..15 {
        let col = [
            GOLD, GRAY, GREEN, LIGHTGRAY, LIME, MAGENTA, MAROON, ORANGE, PINK, PURPLE, RED,
            SKYBLUE, VIOLET, WHITE, YELLOW,
        ];
        //[GOLD, GRAY, GREEN, LIGHTGRAY, LIME, MAGENTA, MAROON, ORANGE, PINK, PURPLE, RED,SKYBLUE, VIOLET, WHITE, YELLOW,];
        let a: f32 = (j as f32);
        let b: f32 = (17.0 - a);
        let c: f32 = a + 1.0 * scaling.sin();
        let d: f32 = b - 1.0 * scaling.sin();
        if (a as i32 % 2 == 0) {
            line(10.0, 10.0, 100.0, 100.0, 2.0, RED);
        }
    }
}
 */
