use image::io::Reader as ImageReader;
use image::GenericImageView;
use svg::node::element::*;
use svg::Document;

pub mod line_intersection;

// usual scale is 1.0 for A4
pub fn signature(scale: f64, translation: (f64, f64), color: &str) -> Path {
    return Path::new().
        set("d", "m 15.815664,12.893319 c -1.445284,-3.0999497 -5.555449,-0.3575 -5.08537,2.32203 1.697826,2.92736 4.504013,-3.54417 4.40859,-2.58178 -1.548999,2.22986 0.741131,6.08701 3.012419,3.25791 2.532153,-2.82358 0.259001,-7.8326797 -3.488671,-7.9011197 -3.217272,0.056 -5.8863857,2.4603197 -7.9308737,4.7238797 -2.354585,2.46752 0.0048,5.887 2.757763,6.62143 3.2195457,1.10867 6.8759417,1.30834 9.9459317,-0.36585 2.270396,-1.12373 5.025949,-2.62031 5.680576,-5.20027 -2.108811,-3.66096 -6.038415,1.28356 -3.842036,3.67822 1.07278,0.89979 4.586982,-2.27037 3.201668,-2.73503 0.03094,3.24357 1.226854,6.37852 1.337023,9.60311 -0.672198,3.54892 -7.469251,0.32096 -4.637082,-2.5164 2.158436,-2.4193 5.610472,-2.84094 8.202925,-4.57369 0.993877,-1.40371 0.353413,-5.25046 3.182464,-3.48957 2.142923,1.43516 -2.250898,5.7532 1.723416,5.02339 1.661189,-0.71663 6.494946,-1.40457 4.601401,-3.95236 -4.205319,-0.68052 -1.190571,5.86505 1.665411,3.46881 1.929752,-0.9247 2.778055,-4.05119 1.423645,-5.35034 0.479155,1.8589 3.849911,7.52574 4.880369,3.32696 0.21201,-1.28088 0.40468,-3.80204 1.01246,-1.23041 0.5858,2.6865 3.83412,4.91909 4.56937,1.07383 0.65272,-1.00894 -0.2696,-4.02739 0.99929,-1.35746 1.10974,2.31613 6.32001,1.46113 6.147,-1.13059 -1.98394,-2.13868 -5.3717,1.45205 -3.78252,3.73454 2.57741,0.96208 6.69797,-0.21041 7.06275,-3.33983 0.41287,-2.63769 0.26643,-5.3430297 -0.11178,-7.9756197 0.67418,3.94149 1.24889,7.9380497 2.39963,11.7713397 2.10586,1.67977 5.7434,1.65022 7.74596,-0.23639 3.03149,-1.85431 -0.26637,-4.76925 -2.71777,-4.54025 -2.11577,0.0793 -5.36257,2.40772 -5.16868,3.85604 2.08262,-2.38818 5.55759,-1.22628 8.30726,-1.6832 3.182,-0.26596 6.46546,-0.57372 9.54494,-1.18158 0.24171,0.4199 -0.27752,0.54338 -0.43067,0.17453")
        .set("fill","none")
        .set("stroke", color)
        .set("stroke-width", 1)
        .set("transform", format!("translate({},{}) scale({})", translation.0, translation.1, 0.3 * scale));
}

pub fn grayscale((r, g, b): (f64, f64, f64)) -> f64 {
    return 0.299 * r + 0.587 * g + 0.114 * b;
}

pub fn smoothstep(a: f64, b: f64, x: f64) -> f64 {
    let k = ((x - a) / (b - a)).max(0.0).min(1.0);
    return k * k * (3.0 - 2.0 * k);
}

// see also https://en.wikipedia.org/wiki/CMYK_color_model
pub fn rgb_to_cmyk((r, g, b): (f64, f64, f64)) -> (f64, f64, f64, f64) {
    let k = 1.0 - r.max(g).max(b);
    let c = (1.0 - r - k) / (1.0 - k);
    let m = (1.0 - g - k) / (1.0 - k);
    let y = (1.0 - b - k) / (1.0 - k);
    return (c, m, y, k);
}

// point is normalized in 0..1
// returned value is a rgb tuple in 0..1 range
pub fn image_get_color(
    path: &str,
) -> Result<impl Fn((f64, f64)) -> (f64, f64, f64), image::ImageError> {
    let img = ImageReader::open(path)?.decode()?;
    let (width, height) = img.dimensions();
    return Ok(move |(x, y)| {
        let xi = (x * (width as f64)) as u32;
        let yi = (y * (height as f64)) as u32;
        let pixel = img.get_pixel(xi, yi);
        let r = (pixel[0] as f64) / 255.0;
        let g = (pixel[1] as f64) / 255.0;
        let b = (pixel[2] as f64) / 255.0;
        return (r, g, b);
    });
}

pub fn layer(id: &str) -> Group {
    return Group::new()
        .set("inkscape:groupmode", "layer")
        .set("inkscape:label", id);
}

pub fn base_a4_portrait(bg: &str) -> Document {
    Document::new()
        .set(
            "xmlns:inkscape",
            "http://www.inkscape.org/namespaces/inkscape",
        )
        .set("viewBox", (0, 0, 210, 297))
        .set("width", "210mm")
        .set("height", "297mm")
        .set("style", format!("background:{}", bg))
}

pub fn base_a4_landscape(bg: &str) -> Document {
    Document::new()
        .set(
            "xmlns:inkscape",
            "http://www.inkscape.org/namespaces/inkscape",
        )
        .set("viewBox", (0, 0, 297, 210))
        .set("width", "297mm")
        .set("height", "210mm")
        .set("style", format!("background:{}", bg))
}