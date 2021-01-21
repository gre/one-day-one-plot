use gre::smoothstep;
use noise::{NoiseFn, Perlin};
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;
use svg::Document;

fn layer(id: &str) -> Group {
    return Group::new()
        .set("inkscape:groupmode", "layer")
        .set("inkscape:label", id);
}

struct Config {
    seed: f64,
    lines: usize,
    length: f64,
}

fn main() {
    let mut groups = Vec::new();

    let configs = vec![Config {
        seed: 0.0,
        lines: 200,
        length: 200.0,
    }];

    for c in configs {
        let color = "black";

        let perlin = Perlin::new();

        // give the field angle (not the length)
        let field = |(x, y): (f64, f64)| {
            PI / 2.0
                + (1.0 - (2.0 * x - 1.0).abs())
                    * (0.1 + y)
                    * (perlin.get([8.0 * x, 8.0 * y, c.seed])
                        + perlin.get([32.0 * x, 32.0 * y, c.seed])
                        + perlin.get([80.0 * x, 80.0 * y, c.seed]))
        };

        let mut data = Data::new();

        let boundaries = (10.0, 10.0, 260.0, 190.0);
        let lines = c.lines;
        let precision = 1.0;
        let iterations = (c.length / precision) as usize;
        for l in 0..lines {
            let mut p = (
                boundaries.0 + (boundaries.2 - boundaries.0) * (l as f64) / (lines as f64),
                boundaries.1,
            );
            let mut first = true;
            for _i in 0..iterations {
                let normalized = (
                    (p.0 - boundaries.0) / (boundaries.2 - boundaries.0),
                    (p.1 - boundaries.1) / (boundaries.3 - boundaries.1),
                );
                let angle = field(normalized);
                let (px, py) = p;
                p = (p.0 + precision * angle.cos(), p.1 + precision * angle.sin());
                if p.0 < boundaries.0
                    || p.0 > boundaries.2
                    || p.1 < boundaries.1
                    || p.1 > boundaries.3
                {
                    break;
                }
                let x = px + 0.1 * (l as f64);
                let y = py;
                if first {
                    first = false;
                    data = data.move_to((x, y));
                } else {
                    data = data.line_to((x, y));
                }
            }
        }

        let path = Path::new()
            .set("fill", "none")
            .set("stroke", color)
            .set("stroke-width", 0.4)
            .set("d", data);

        groups.push(layer(color).add(path));
    }

    let mut document = Document::new()
        .set(
            "xmlns:inkscape",
            "http://www.inkscape.org/namespaces/inkscape",
        )
        .set("style", "background: white")
        .set("viewBox", (0, 0, 297, 210))
        .set("width", "297mm")
        .set("height", "210mm")
        .add(gre::signature(1.0, (175.0, 195.0), "black"));
    for g in groups {
        document = document.add(g);
    }

    svg::save("image.svg", &document).unwrap();
}
