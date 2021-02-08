use geo::{Line};
use gre::line_intersection::*;
use gre::*;
use noise::*;
use svg::node::element::path::Data;
use svg::node::element::*;

fn round_point ((x,y):(f64,f64)) -> (f64, f64) {
    let mag = 1000.0; // TODO play with diff values
    ((x*mag).round()/mag, (y*mag).round()/mag)
}

fn art(seed: f64, dim: usize, force:f64, length: f64) -> Vec<Group> {
    let perlin = Perlin::new();
    let mut groups = Vec::new();

    let field = |xp: f64, yp: f64| -> f64 {
(        yp-0.5).atan2(xp-0.5)+force *(
        4.0 * perlin.get([2. * xp, 2. * yp, seed + 1.])
            + 2.0 * perlin.get([4. * xp, 4. * yp, seed + 2.])
            + 0.5 * perlin.get([20. * xp, 20. * yp, seed + 3.]))
    };

    // VLine is our struct define below that record vertex lines
    let mut vlines: Vec<VLine> = Vec::new();

    let pad = 2.0;
    let squares = vec!["white", "gold", "orange"];
    for (group, color) in squares.iter().enumerate() {
        let mut group_vlines = Vec::new();
        let gf = group as f64;
        let sz = 10.0 + gf * 20.0;
        let o = 105. - sz;
        let x_o = o;
        let y_o = 30.0 + o;
        let width = 2. * sz;
        let height = 2. * sz;

        for x in 0..dim {
            let xp = (x as f64 + 0.5) / (dim as f64);
            for y in 0..dim {
                let yp = (y as f64 + 0.5) / (dim as f64);
                if x.min(y)!=0 && x.max(y)!=dim-1 {
                    continue;
                }
                let origin = round_point((x_o + width * xp, y_o + height * yp));
                let mut vline = VLine::new(group, origin);
                let granularity = 1.0;
                for _i in 0..((length/granularity) as usize) {
                    let cur = vline.current();
                    if cur.0 < pad || cur.1 < pad || cur.0 > 210.-pad || cur.1 > 297.-pad {
                        break;
                    }
                    let angle = field((cur.0 - x_o) / width, (cur.1 - y_o) / height);
                    let next = round_point(vline.follow_angle(angle, granularity));
                    let collision = vlines.iter().find_map(|vl| vl.collides(cur, next));
                    if let Some(point) = collision {
                        vline.go(point);
                        break;
                    }
                    vline.go(next);
                }
                vlines.push(vline.clone());
                group_vlines.push(vline);
            }
        }
        let data = group_vlines.iter().fold(Data::new(), |data, vl| vl.draw(data));
        groups.push(layer(color).add(base_path(color, 0.2, data)));
    }

    groups
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let seed = args
        .get(1)
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(0.0);
    let lines = args
        .get(2)
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(50);
    let force = args
        .get(3)
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(1.0);
    let length = args
        .get(4)
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(100.0);
    let groups = art(seed, lines, force, length);
    let mut document = base_a4_portrait("black");
    for g in groups {
        document = document.add(g);
    }
    document = document.add(signature(1.0, (180.0, 280.0), "white"));
    svg::save("image.svg", &document).unwrap();
}

#[derive(Clone)]
struct VLine {
    group: usize,
    points: Vec<(f64, f64)>,
    aabb: ((f64, f64), (f64, f64)),
}
impl VLine {
    fn new(group: usize, initial: (f64, f64)) -> Self {
        let mut points = Vec::new();
        points.push(initial);
        VLine {
            group,
            points,
            aabb: (initial, initial),
        }
    }

    fn current(self: &Self) -> (f64, f64) {
        self.points[0]
    }

    fn follow_angle(self: &Self, a: f64, amp: f64) -> (f64, f64) {
        let cur = self.points[0];
        let p = (cur.0 + amp * a.cos(), cur.1 + amp * a.sin());
        p
    }

    fn go(self: &mut Self, p: (f64, f64)) {
        self.points.insert(0, p);
        if p.0 < self.aabb.0.0 {
            self.aabb.0.0 = p.0;
        }
        if p.1 < self.aabb.0.1 {
            self.aabb.0.1 = p.1;
        }
        if p.0 > self.aabb.1.0 {
            self.aabb.1.0 = p.0;
        }
        if p.1 > self.aabb.1.1 {
            self.aabb.1.1 = p.1;
        }
    }

    fn draw(self: &Self, data: Data) -> Data {
        let mut d = data;
        let l = self.points.len();
        let first = self.points[l - 1];
        d = d.move_to(first);
        for i in 0..l - 1 {
            let p = self.points[l - i - 2];
            d = d.line_to(p);
        }
        return d;
    }

    fn collides(self: &Self, from: (f64, f64), to: (f64, f64)) -> Option<(f64, f64)> {
        if from.0.min(to.0) < self.aabb.0.0 {
            return None;
        }
        if from.1.min(to.1) < self.aabb.0.1 {
            return None;
        }
        if from.0.max(to.0) > self.aabb.1.0 {
            return None;
        }
        if from.1.max(to.1) > self.aabb.1.1 {
            return None;
        }
        let segment = LineInterval::line_segment(Line {
            start: to.into(),
            end: from.into(),
        });
        let mut last = self.points[0];
        for i in 1..self.points.len() {
            let p = self.points[i];
            let intersection = segment
                .relate(&LineInterval::line_segment(Line {
                    start: p.into(),
                    end: last.into(),
                }))
                .unique_intersection()
                .map(|p| p.x_y());
            if intersection.is_some() {
                return intersection;
            }
            last = p;
        }
        return None;
    }
}
