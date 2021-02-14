use gre::*;
use rand::prelude::*;
use svg::node::element::path::Data;
use svg::node::element::*;

fn scale_a4_portrait((x, y): (f64, f64)) -> (f64, f64) {
    (x * 210., y * 297.)
}

fn art() -> Vec<Group> {
    let duration = time::Duration::hours(4);
    let samples = 5000;
    let res = 400;

    let get_color =
        image_get_color("images/bitcoin_portrait.png")
            .unwrap();

    let f = |p| {
        let r = grayscale(get_color(p));
        0.1 < r && r < 0.8
    };

    let mut rng = SmallRng::from_seed([
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ]);
    let candidates =
        sample_2d_candidates(&f, res, samples, &mut rng);
    let tour =
        travelling_salesman::simulated_annealing::solve(
            &candidates,
            duration,
        );
    let data = render_route(
        Data::new(),
        tour.route
            .iter()
            .map(|&i| scale_a4_portrait(candidates[i]))
            .collect(),
    );
    vec![Group::new().add(
        layer("bitcoin")
            .add(base_path("orange", 0.2, data)),
    )]
}

fn main() {
    let groups = art();
    let mut document = base_a4_portrait("white");
    for g in groups {
        document = document.add(g);
    }
    document = document.add(signature(
        1.0,
        (175.0, 280.0),
        "black",
    ));
    svg::save("bitcoin.svg", &document).unwrap();
}
