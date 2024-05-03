use itertools::Itertools;
use xcap::{image::Rgba, Window};

#[derive(Debug, Clone)]
pub struct ScreenCapture {
    samples: Vec<(SampleColor, (u32, u32))>,
}

impl ScreenCapture {
    pub fn new(window: &Window) -> Self {
        let image = window
            .capture_image()
            .expect("Couldn't capture Minecraft window.");

        let mut samples = vec![];

        let good_rows = image.enumerate_rows().step_by(10);
        for row in good_rows {
            let pixels = row.1.collect_vec();
            for chunk in pixels.chunks_exact(50) {
                let sample_color = chunk
                    .iter()
                    .map(|(_, _, pixel)| SampleColor::from_rgba(**pixel))
                    .all_equal_value();

                if let Some(sample_color) = sample_color.ok().flatten() {
                    let (x, y, _) = chunk[0];
                    samples.push((sample_color, (x, y)));
                }
            }
        }

        Self { samples }
    }

    pub fn average_of_color(&self, wanted_color: SampleColor) -> Option<(u32, u32)> {
        let good_samples = self
            .samples
            .iter()
            .cloned()
            .filter(|(color, _)| *color == wanted_color);

        let len = good_samples.clone().count() as u32;

        good_samples
            .map(|(_, coords)| coords)
            .reduce(|(x1, y1), (x2, y2)| (x1 + x2, y1 + y2))
            .map(|(x, y)| (x / len, y / len))
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum SampleColor {
    Green,
    White,
}

impl SampleColor {
    fn from_rgba(rgba: Rgba<u8>) -> Option<Self> {
        let [r, g, b, _] = rgba.0;
        if g > 120 && g > r.saturating_add(r & b) {
            Some(Self::Green)
        } else if r > 150 && r.abs_diff(g) < 10 && r.abs_diff(b) < 10 {
            Some(Self::White)
        } else {
            None
        }
    }
}
