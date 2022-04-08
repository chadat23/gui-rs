use crate::guiproperties::guiposition::{GUILength, GUIPosition};

pub mod arcs {
    use crate::guiproperties::guiposition::{GUILength, GUIPosition};

    const PI: f64 = 3.141592653589793238;

    /// Creates a set of vertices to approximate the top right quadrent of a circle.
    /// The center of the arc has the coordinates 0, 0.
    /// fascets is the number of fascets in the arc, so one quater of the
    /// number of fascets that'd be in a similarly detialed circle.
    /// The bottom right most vertice is the first with them in counter clockwise order.
    pub fn make_top_right_arc(radius: GUILength, fascets: usize) -> Vec<GUIPosition> {
        let vertices = (fascets + 1) as usize;
        let mut positions = Vec::with_capacity(vertices);

        for i in 0..vertices {
            let angle = 2. * PI * i as f64 / (fascets * 4) as f64;

            positions.push(GUIPosition {
                x: GUILength {
                    length: radius.length * angle.cos(),
                },
                y: GUILength {
                    length: -radius.length * angle.sin(),
                },
            });
        }

        positions
    }

    /// Creates a set of vertices to approximate the top left quadrent of a circle.
    /// The center of the arc has the coordinates 0, 0.
    /// fascets is the number of fascets in the arc, so one quater of the
    /// number of fascets that'd be in a similarly detialed circle.
    /// The top right most vertice is the first with them in counter clockwise order.
    pub fn make_top_left_arc(radius: GUILength, fascets: usize) -> Vec<GUIPosition> {
        let mut top_right = make_top_right_arc(radius, fascets);
        top_right.reverse();
        top_right
            .iter()
            .map(|position| GUIPosition {
                x: GUILength {
                    length: position.x.length * -1.,
                },
                y: position.y,
            })
            .collect()
    }

    /// Creates a set of vertices to approximate the bottom right quadrent of a circle.
    /// The center of the arc has the coordinates 0, 0.
    /// fascets is the number of fascets in the arc, so one quater of the
    /// number of fascets that'd be in a similarly detialed circle.
    /// The bottom left most vertice is the first with them in counter clockwise order.
    pub fn make_bottom_right_arc(radius: GUILength, fascets: usize) -> Vec<GUIPosition> {
        let mut top_right = make_top_right_arc(radius, fascets);
        top_right.reverse();
        top_right
            .iter()
            .map(|position| GUIPosition {
                x: position.x,
                y: GUILength {
                    length: position.y.length * -1.,
                },
            })
            .collect()
    }

    /// Creates a set of vertices to approximate the bottom left quadrent of a circle.
    /// The center of the arc has the coordinates 0, 0.
    /// fascets is the number of fascets in the arc, so one quater of the
    /// number of fascets that'd be in a similarly detialed circle.
    /// The top left most vertice is the first with them in counter clockwise order.
    pub fn make_bottom_left_arc(radius: GUILength, fascets: usize) -> Vec<GUIPosition> {
        let top_right = make_top_right_arc(radius, fascets);
        top_right
            .iter()
            .map(|position| GUIPosition {
                x: GUILength {
                    length: position.x.length * -1.,
                },
                y: GUILength {
                    length: position.y.length * -1.,
                },
            })
            .collect()
    }
}

pub fn translate(
    mut vertices: Vec<GUIPosition>,
    dx: &GUILength,
    dy: &GUILength,
) -> Vec<GUIPosition> {
    for vertice in vertices.iter_mut() {
        vertice.x = vertice.x.add(dx);
        vertice.y = vertice.y.add(dy);
    }

    vertices
}

#[cfg(test)]
mod tests {
    use crate::guiproperties::guiposition::{GUILength, GUIPosition};

    use crate::guiwidgets::widget_utils::arcs::*;

    #[test]
    fn make_top_right_arc_good() {
        let actual = make_top_right_arc(GUILength::from_pixels(5.), 4);
        let expected = Vec::from([
            GUIPosition::from_pixels(5., 0.),
            GUIPosition::from_pixels(4.6193976625564339, -1.913417161825449),
            GUIPosition::from_pixels(3.5355339059327378, -3.5355339059327373),
            GUIPosition::from_pixels(1.9134171618254492, -4.6193976625564339),
            GUIPosition::from_pixels(0., -5.),
        ]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn make_top_left_arc_good() {
        let actual = make_top_left_arc(GUILength::from_pixels(5.), 4);
        let expected = Vec::from([
            GUIPosition::from_pixels(0., -5.),
            GUIPosition::from_pixels(-1.9134171618254492, -4.6193976625564339),
            GUIPosition::from_pixels(-3.5355339059327378, -3.5355339059327373),
            GUIPosition::from_pixels(-4.6193976625564339, -1.913417161825449),
            GUIPosition::from_pixels(-5., 0.),
        ]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn make_bottom_right_arc_good() {
        let actual = make_bottom_right_arc(GUILength::from_pixels(5.), 4);
        let expected = Vec::from([
            GUIPosition::from_pixels(0., 5.),
            GUIPosition::from_pixels(1.9134171618254492, 4.6193976625564339),
            GUIPosition::from_pixels(3.5355339059327378, 3.5355339059327373),
            GUIPosition::from_pixels(4.6193976625564339, 1.913417161825449),
            GUIPosition::from_pixels(5., 0.),
        ]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn make_bottom_left_arc_good() {
        let actual = make_bottom_left_arc(GUILength::from_pixels(5.), 4);
        let expected = Vec::from([
            GUIPosition::from_pixels(-5., 0.),
            GUIPosition::from_pixels(-4.6193976625564339, 1.913417161825449),
            GUIPosition::from_pixels(-3.5355339059327378, 3.5355339059327373),
            GUIPosition::from_pixels(-1.9134171618254492, 4.6193976625564339),
            GUIPosition::from_pixels(0., 5.),
        ]);
        assert_eq!(actual, expected);
    }
}
