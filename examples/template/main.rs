use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::Document;

// https://docs.rs/svg/0.8.0/svg/

fn main() {
    let data = Data::new()
        .move_to((10, 10))
        .line_by((0, 50))
        .line_by((50, 0))
        .line_by((0, -50))
        .close();

    let path = Path::new()
        .set("fill", "none")
        .set("stroke", "red")
        .set("stroke-width", 3)
        .set("d", data);

    let document = Document::new().set("viewBox", (0, 0, 70, 70)).add(path);

    svg::save("image.svg", &document).unwrap();
}
