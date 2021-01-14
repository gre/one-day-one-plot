use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::Document;

// https://docs.rs/svg/0.8.0/svg/

fn main() {
    let data = Data::new()
        .move_to((50, 50))
        .line_by((50, 100))
        .line_by((100, 50))
        .line_by((50, -100))
        .close();

    let path = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 1)
        .set("d", data);

    let document = Document::new()
        .set(
            "xmlns:inkscape",
            "http://www.inkscape.org/namespaces/inkscape",
        )
        .set("viewBox", (0, 0, 297, 210))
        .set("width", "297mm")
        .set("height", "210mm")
        .add(path)
        .add(gre::signature(1.0, (265.0, 195.0)));

    svg::save("image.svg", &document).unwrap();
}
