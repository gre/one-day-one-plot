use gre::*;
use rand::prelude::*;
use rayon::prelude::*;
use svg::node::element::path::Data;
use svg::node::element::*;

pub fn render_route_curve(
    data: Data,
    route: Vec<(f64, f64)>,
) -> Data {
    let mut first = true;
    let mut d = data;
    let mut last = route[0];
    for p in route {
        if first {
            first = false;
            d = d.move_to(p);
        } else {
            d = d.quadratic_curve_to((
                last.0,
                last.1,
                (p.0 + last.0) / 2.,
                (p.1 + last.1) / 2.,
            ));
        }
        last = p;
    }
    return d;
}

fn art(seed0: u8, minutes: i64) -> Vec<Group> {
    let mut groups = Vec::new();

    let get_color =
        image_get_color("images/man_jump.jpg").unwrap();

    let routes: Vec<Vec<(f64, f64)>> = (0..10)
        .into_par_iter()
        .map(|i| {
            let mut rng = SmallRng::from_seed([
                seed0, i as u8, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0,
            ]);

            let mut samples = sample_2d_candidates_f64(
                &|p| {
                    0.1 * smoothstep(
                        0.5,
                        0.0,
                        grayscale(get_color(p)),
                    )
                },
                300,
                200,
                &mut rng,
            );

            samples = tsp(
                samples,
                time::Duration::minutes(minutes),
            );

            samples.push(samples[0]);

            samples
        })
        .collect();

    // samples = route_spiral(samples);

    let mut data = Data::new();

    let bounds = (0., 0., 297., 210.);

    for route in routes {
        let pts = route
            .iter()
            .map(|&p| project_in_boundaries(p, bounds))
            .collect();

        data = render_route_curve(data, pts);
    }

    let color = "black";
    groups.push(
        layer(color).add(base_path(color, 0.2, data)),
    );

    groups
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let seed = args
        .get(1)
        .and_then(|s| s.parse::<u8>().ok())
        .unwrap_or(0);
    let minutes = args
        .get(2)
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or(0);
    let groups = art(seed, minutes);
    let mut document = base_a4_landscape("white");
    for g in groups {
        document = document.add(g);
    }
    document = document.add(signature(
        1.0,
        (185.0, 160.0),
        "black",
    ));
    svg::save("jump.svg", &document).unwrap();
}
