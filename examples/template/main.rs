use gre::{layer, signature};
use noise::{NoiseFn, Perlin};
use svg::node::element::path::Data;
use svg::node::element::*;
use svg::Document;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let seed = args
        .get(1)
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(0.0);

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
