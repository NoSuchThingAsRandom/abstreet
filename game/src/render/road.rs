use crate::helpers::{ColorScheme, ID};
use crate::render::{dashed_lines, DrawCtx, DrawOptions, Renderable, OUTLINE_THICKNESS};
use ezgui::{Color, Drawable, GeomBatch, GfxCtx, Line, Prerender, Text};
use geom::{Distance, Polygon, Pt2D};
use map_model::{Map, Road, RoadID};

pub struct DrawRoad {
    pub id: RoadID,
    zorder: isize,

    draw_center_line: Drawable,
    label: Text,
    label_pos: Pt2D,
}

impl DrawRoad {
    pub fn new(r: &Road, cs: &ColorScheme, prerender: &Prerender) -> DrawRoad {
        let mut draw = GeomBatch::new();
        draw.extend(
            cs.get_def("road center line", Color::YELLOW),
            dashed_lines(&r.center_pts, Distance::meters(2.0), Distance::meters(1.0)),
        );

        let mut label = Text::new();
        label.add(Line(r.get_name()).size(50));

        DrawRoad {
            id: r.id,
            zorder: r.get_zorder(),
            draw_center_line: prerender.upload(draw),
            label,
            label_pos: r.center_pts.middle(),
        }
    }
}

impl Renderable for DrawRoad {
    fn get_id(&self) -> ID {
        ID::Road(self.id)
    }

    fn draw(&self, g: &mut GfxCtx, opts: &DrawOptions, _: &DrawCtx) {
        g.redraw(&self.draw_center_line);
        if opts.label_roads {
            g.draw_text_at_mapspace(&self.label, self.label_pos);
        }
    }

    fn get_outline(&self, map: &Map) -> Polygon {
        let (pl, width) = map.get_r(self.id).get_thick_polyline().unwrap();
        pl.to_thick_boundary(width, OUTLINE_THICKNESS)
            .unwrap_or_else(|| map.get_r(self.id).get_thick_polygon().unwrap())
    }

    fn contains_pt(&self, pt: Pt2D, map: &Map) -> bool {
        map.get_r(self.id)
            .get_thick_polygon()
            .unwrap()
            .contains_pt(pt)
    }

    fn get_zorder(&self) -> isize {
        self.zorder
    }
}