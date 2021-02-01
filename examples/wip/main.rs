use gre::*;
use noise::{NoiseFn, Perlin};
use rand::prelude::*;
use svg::node::element::path::Data;
use svg::node::element::*;
use svg::Document;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let default = &String::from("images/bird.png");
    let path = args.get(1).unwrap_or(default);
    let seconds = args.get(2).and_then(|s| s.parse::<i64>().ok()).unwrap_or(5);
    let count = args
        .get(2)
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(3000);
    let get_color = image_get_color(path).unwrap();
    let mut rng = SmallRng::from_seed([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

    // select all pixels that are suitable to drawing
    let mut candidates = Vec::new();
    let dim = 200;
    for x in 0..dim {
        for y in 0..dim {
            let p = ((x as f64) / (dim as f64), (y as f64) / (dim as f64));
            let c = get_color(p);
            let g = grayscale(c);
            if g < 0.5 {
                candidates.push(p);
            }
        }
    }

    // pick <count> random samples out
    rng.shuffle(&mut candidates);
    candidates.truncate(count);

    let perlin = Perlin::new();

    let mut groups = Vec::new();

    let mut data = Data::new();

    let color = "black";
    groups.push(
        layer(color).add(
            Path::new()
                .set("fill", "none")
                .set("stroke", color)
                .set("stroke-width", 0.2)
                .set("d", data),
        ),
    );

    // Make svg
    let mut document = Document::new()
        .set(
            "xmlns:inkscape",
            "http://www.inkscape.org/namespaces/inkscape",
        )
        .set("style", "background: white")
        .set("viewBox", (0, 0, 297, 210))
        .set("width", "297mm")
        .set("height", "210mm")
        .add(signature(1.0, (260.0, 190.0), "black"));
    for g in groups {
        document = document.add(g);
    }
    svg::save("image.svg", &document).unwrap();
}
