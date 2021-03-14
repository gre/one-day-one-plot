use clap::Clap;
use gre::*;
use noise::*;
use rand::prelude::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(opts: Opts) -> Vec<Group> {
    let spacing = 0.5;

    let colors = vec!["red", "black"];
    let dim = (297., 210.);
    let pad = 10.0;
    let boundaries = (pad, pad, dim.0 - pad, dim.1 - pad);
    let perlin = Perlin::new();
    let mut rng =
        SmallRng::from_seed([opts.seed as u8; 16]);
    for _i in 0..50 {
        rng.gen::<f64>();
    }

    let initial_angle_off = rng.gen_range(2.0, 5.0);
    let f1 = rng.gen_range(0.2, 8.0);
    let f2 = rng.gen_range(1.0, 20.0);
    let perlin_amp = rng.gen_range(0.4, 2.0);
    let ran = rng.gen_range(0.7, 1.4);

    let xpad = 60.0;
    let ypad = 30.0;
    let col = 3;
    let row = 2;
    let r = 40.0;

    let initial: Vec<(usize, (f64, f64), (f64, f64))> = (0
        ..(col * row))
        .flat_map(|ci| {
            let yi = ci / col;
            let xi = ci - yi * col;
            let c = (
                xpad + (dim.0 - 2. * xpad)
                    * (xi as f64 + 0.5)
                    / (col as f64),
                -((ci % 2) as f64) * r / 2.0
                    + ypad
                    + (dim.1 - 2. * ypad)
                        * (yi as f64 + 0.5)
                        / (row as f64),
            );
            let mut poly: Vec<(f64, f64)> = Vec::new();
            let res = (r / spacing) as usize;
            for j in 0..3 {
                let a = (0.25
                    + j as f64
                    + (ci % 2) as f64 * 0.5)
                    * PI
                    * 2.
                    / 3.;
                poly.push((
                    c.0 + r * a.cos(),
                    c.1 + r * a.sin(),
                ));
            }
            poly.push(poly[0]);

            let mut points = Vec::new();
            let mut last = poly[0];
            for i in 1..poly.len() {
                let p = poly[i];
                for j in 0..res {
                    let v = j as f64 / (res as f64);
                    points.push((
                        ci,
                        c,
                        (
                            last.0 + (p.0 - last.0) * v,
                            last.1 + (p.1 - last.1) * v,
                        ),
                    ));
                }
                last = p;
            }
            points
        })
        .collect();

    let initial_positions: Vec<(f64, f64)> = initial
        .iter()
        .map(|(_i, _c, p)| p.clone())
        .collect();

    let initial_a: Vec<f64> = initial
        .iter()
        .map(|(_i, c, p)| {
            (p.1 - c.1).atan2(p.0 - c.0) + initial_angle_off
        })
        .collect();

    let build_route = |p: (f64, f64), i, route_i| {
        let px = p.0 / dim.0;
        let py = p.1 / dim.1;
        let (ci, _c, _origin) = initial[route_i];
        let base_a = initial_a[route_i];
        let a: f64 = i as f64 * 0.01
            + base_a
            + perlin_amp
                * (2.0
                    * perlin.get([
                        f1 * px,
                        f1 * py,
                        opts.seed
                            + ci as f64
                            + route_i as f64 * 0.01 * ran,
                    ])
                    + 1.0
                        * perlin.get([
                            f2 * px,
                            f2 * py,
                            100.0
                                + opts.seed
                                + route_i as f64
                                    * 0.1
                                    * ran,
                        ]));
        let amp = 1.0;
        let d = (amp * a.cos(), amp * a.sin());
        let next = (p.0 + d.0, p.1 + d.1);
        let ends = i > 1000;
        if ends {
            return None;
        }
        if let Some(c) =
            collide_segment_boundaries(p, next, boundaries)
        {
            return Some((c, true));
        }
        Some((next, false))
    };

    let routes = build_routes_with_collision_par(
        initial_positions,
        &build_route,
    );

    colors
        .iter()
        .enumerate()
        .map(|(i, color)| {
            let mut data = routes
                .iter()
                .enumerate()
                .filter_map(|(ri, r)| {
                    let (ci, _c, _origin) = initial[ri];
                    if ci % colors.len() == i {
                        Some(r)
                    } else {
                        None
                    }
                })
                .fold(Data::new(), |data, r| {
                    render_route(data, r.clone())
                });

            for j in 0..(col * row) {
                if j % colors.len() == i {
                    data = render_route(
                        data,
                        initial
                            .iter()
                            .filter_map(
                                |&(ci, _c, origin)| {
                                    if ci == j {
                                        Some(origin)
                                    } else {
                                        None
                                    }
                                },
                            )
                            .collect(),
                    );
                }
            }

            let mut l = layer(color);
            l = l.add(base_path(color, 0.2, data));
            if i == colors.len() - 1 {
                l = l.add(signature(
                    1.0,
                    (220.0, 150.0),
                    color,
                ));
            }
            l
        })
        .collect()
}

#[derive(Clap)]
#[clap()]
struct Opts {
    #[clap(short, long, default_value = "103.0")]
    seed: f64,
}

fn main() {
    let opts: Opts = Opts::parse();
    let groups = art(opts);
    let mut document = base_a4_landscape("white");
    for g in groups {
        document = document.add(g);
    }
    svg::save("image.svg", &document).unwrap();
}
