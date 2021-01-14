extern crate gre;

use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::Document;

fn cistercian_gen(
    data: Data,
    x: f32,
    y: f32,
    scale: f32,
    n: u32,
    flipx: bool,
    flipy: bool,
) -> Data {
    let m = |mut dx, dy| {
        if flipx {
            dx *= -1.0;
        };
        if flipy {
            return (x + scale * dx, y - (3.0 - dy) * scale);
        } else {
            return (x + scale * dx, y - dy * scale);
        }
    };
    match n % 10 {
        1 => data.move_to(m(0.0, 3.0)).line_to(m(1.0, 3.0)),
        2 => data.move_to(m(0.0, 2.0)).line_to(m(1.0, 2.0)),
        3 => data.move_to(m(0.0, 3.0)).line_to(m(1.0, 2.0)),
        4 => data.move_to(m(0.0, 2.0)).line_to(m(1.0, 3.0)),
        5 => data
            .move_to(m(0.0, 3.0))
            .line_to(m(1.0, 3.0))
            .line_to(m(0.0, 2.0)),
        6 => data.move_to(m(1.0, 3.0)).line_to(m(1.0, 2.0)),
        7 => data
            .move_to(m(1.0, 2.0))
            .line_to(m(1.0, 3.0))
            .line_to(m(0.0, 3.0)),
        8 => data
            .move_to(m(0.0, 2.0))
            .line_to(m(1.0, 2.0))
            .line_to(m(1.0, 3.0)),
        9 => data
            .move_to(m(0.0, 2.0))
            .line_to(m(1.0, 2.0))
            .line_to(m(1.0, 3.0))
            .line_to(m(0.0, 3.0)),
        _ => data,
    }
}

// https://en.wikipedia.org/wiki/Cistercian_numerals
// use 2x3mm base. use scale to scale that. centered on bottom-center
fn cistercian(n: u32, x: f32, y: f32, scale: f32) -> Data {
    let mut data = Data::new();
    data = data.move_to((x, y));
    data = data.line_to((x, y - 3.0 * scale));
    data = cistercian_gen(data, x, y, scale, n % 10, false, false);
    data = cistercian_gen(data, x, y, scale, (n / 10) % 10, true, false);
    data = cistercian_gen(data, x, y, scale, (n / 100) % 10, false, true);
    data = cistercian_gen(data, x, y, scale, (n / 1000) % 10, true, true);
    return data;
}

fn main() {
    let mut paths = vec![];

    for i in 0..1000 {
        let x = i % 30;
        let y = i / 30;
        paths.push(
            Path::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 0.2)
                .set(
                    "d",
                    cistercian(i, 15.0 + (x as f32) * 9.0, 30.0 + (y as f32) * 11.0, 2.0),
                ),
        );
    }

    let mut document = Document::new()
        .set(
            "xmlns:inkscape",
            "http://www.inkscape.org/namespaces/inkscape",
        )
        .set("style", "background:white")
        .set("viewBox", (0, 0, 297, 420))
        .set("width", "297mm")
        .set("height", "420mm")
        .add(gre::signature(1.0, (265.0, 405.0)));
    for path in paths {
        document = document.add(path);
    }

    svg::save("image.svg", &document).unwrap();
}
