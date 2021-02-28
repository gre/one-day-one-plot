use gre::*;
use noise::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

fn parametric(p: f64) -> (f64, f64) {
    (
        0.6 * (2.0 * PI * p).cos()
            + 0.4 * (38.0 * PI * p).cos(),
        0.6 * (2.0 * PI * p).sin()
            + 0.4 * (38.0 * PI * p).sin(),
    )
}

fn art(seed: f64) -> Vec<Group> {
    let colors =
        vec!["cyan", "pink", "red", "brown", "black"];
    let pad = 5.0;
    let width = 297.0;
    let height = 210.0;
    let size = 60.0;
    let bounds = (pad, pad, width - pad, height - pad);

    let line_length = 1000.0;
    let granularity = 1.0;
    let samples = 2000;

    let perlin = Perlin::new();
    let get_angle =
        |(x, y), (ox, oy), initial_angle, length| {
            initial_angle
                + 10.0
                    * (perlin.get([0.1 * x, 0.1 * y, seed])
                        + perlin.get([
                            0.01 * x,
                            0.01 * y,
                            seed,
                        ]))
                    * (length / line_length)
        };

    let mut routes = Vec::new();

    for s in 0..samples {
        let sp = s as f64 / (samples as f64);
        let o = parametric(sp);
        let dt = 0.0001;
        let o2 = parametric(sp + dt);
        let initial_angle = (o.1 - o2.1).atan2(o.0 - o2.0);
        let mut p = (
            width * 0.5 + size * o.0,
            height * 0.5 + size * o.1,
        );
        let mut route = Vec::new();
        route.push(p);
        for l in 0..((line_length / granularity) as usize) {
            if out_of_boundaries(p, bounds) {
                break;
            }
            let angle = get_angle(
                p,
                o,
                initial_angle,
                l as f64 * granularity,
            );
            p = follow_angle(p, angle, granularity);
            route.push(p);
        }
        routes.push(route);
    }

    routes = routes
        .iter()
        .map(|route| round_route(route.clone(), 0.01))
        .collect();
    routes = collide_routes_parallel(routes);

    let mut groups = Vec::new();

    for (i, color) in colors.iter().enumerate() {
        let mut data = Data::new();
        for (j, route) in routes.iter().enumerate() {
            if j % colors.len() == i {
                data = render_route(data, route.clone());
            }
        }

        let mut g = layer(color);

        g = g.add(base_path(color, 0.2, data));

        if i == colors.len() - 1 {
            g = g.add(signature(1.0, (250.0, 190.0), color))
        }

        groups.push(g);
    }

    groups
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let seed = args
        .get(1)
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(0.0);
    let groups = art(seed);
    let mut document = base_a4_landscape("white");
    for g in groups {
        document = document.add(g);
    }
    svg::save("image.svg", &document).unwrap();
}
