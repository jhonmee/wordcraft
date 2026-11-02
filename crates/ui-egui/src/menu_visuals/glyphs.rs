//! Pictures for drop-down menu entries: page setup (margins, orientation, columns, breaks, text
//! direction, line numbers), object layout (wrapping, rotation, alignment), drop caps, borders,
//! paragraph spacing, headers and footers, page numbers and tables of contents. Original
//! drawings on a 20×20 grid in two colours (line work and an accent), the same scheme as the
//! app's icon set; menus draw them larger than toolbar icons, so the line is thinner.
//!
//! Only egui: no application types, so every Craft app's menus can share them.

use egui::{Color32, Painter, Pos2, Rect, Shape, Stroke, pos2, vec2};

/// Draws on the 20×20 grid scaled into `r`.
struct Pen<'a> {
    p: &'a Painter,
    r: Rect,
    c: Color32,
    a: Color32,
    w: f32,
}

impl Pen<'_> {
    fn s(&self) -> f32 {
        self.r.width().min(self.r.height()) / 20.0
    }
    fn pt(&self, x: f32, y: f32) -> Pos2 {
        let s = self.s();
        let o = self.r.center() - vec2(10.0 * s, 10.0 * s);
        pos2(o.x + x * s, o.y + y * s)
    }
    fn stroke(&self, c: Color32) -> Stroke {
        Stroke::new(self.w * self.s(), c)
    }
    fn line(&self, pts: &[(f32, f32)]) {
        self.line_c(pts, self.c);
    }
    fn line_c(&self, pts: &[(f32, f32)], c: Color32) {
        let v: Vec<Pos2> = pts.iter().map(|&(x, y)| self.pt(x, y)).collect();
        self.p.add(Shape::line(v, self.stroke(c)));
    }
    fn closed(&self, pts: &[(f32, f32)], c: Color32) {
        let v: Vec<Pos2> = pts.iter().map(|&(x, y)| self.pt(x, y)).collect();
        self.p.add(Shape::closed_line(v, self.stroke(c)));
    }
    /// Fills a convex polygon.
    fn fill(&self, pts: &[(f32, f32)], c: Color32) {
        let v: Vec<Pos2> = pts.iter().map(|&(x, y)| self.pt(x, y)).collect();
        self.p.add(Shape::convex_polygon(v, c, Stroke::NONE));
    }
    fn rect(&self, x0: f32, y0: f32, x1: f32, y1: f32, c: Color32) {
        self.p.rect_stroke(Rect::from_min_max(self.pt(x0, y0), self.pt(x1, y1)), 0.5, self.stroke(c), egui::StrokeKind::Middle);
    }
    fn frect(&self, x0: f32, y0: f32, x1: f32, y1: f32, c: Color32) {
        self.p.rect_filled(Rect::from_min_max(self.pt(x0, y0), self.pt(x1, y1)), 0.5, c);
    }
    fn fcircle(&self, x: f32, y: f32, r: f32, c: Color32) {
        self.p.circle_filled(self.pt(x, y), r * self.s(), c);
    }
    /// `t` centred on (`x`, `y`), `size` grid units tall. Digits and single letters only, so the
    /// pictures need no translation.
    fn text(&self, x: f32, y: f32, size: f32, t: &str, c: Color32, bold: bool) {
        let px = size * self.s();
        if !px.is_finite() || px <= 0.0 {
            return;
        }
        let font = if bold { crate::theme::semibold(size * self.s()) } else { egui::FontId::proportional(size * self.s()) };
        self.p.text(self.pt(x, y), egui::Align2::CENTER_CENTER, t, font, c);
    }
    fn arrow_down(&self, x: f32, y: f32, c: Color32) {
        self.fill(&[(x - 2.5, y - 1.2), (x + 2.5, y - 1.2), (x, y + 1.5)], c);
    }
    /// A rectangle outlined with a whole number of screen pixels whose edges sit on the pixel
    /// grid, optionally filled: nested rectangles (a page and its text area) stay crisp and
    /// evenly spaced at any interface size instead of blurring a half pixel to one side.
    fn crisp_rect(&self, x0: f32, y0: f32, x1: f32, y1: f32, stroke: Color32, fill: Option<Color32>) {
        let ppp = self.p.pixels_per_point().max(0.5);
        let px = (self.w * self.s() * ppp).round().max(1.0);
        let width = px / ppp;
        // An odd number of pixels is centred on a pixel's middle, an even one on its edge.
        let snap = |v: f32| {
            let v = v * ppp;
            (if px % 2.0 == 1.0 { v.floor() + 0.5 } else { v.round() }) / ppp
        };
        let (a, b) = (self.pt(x0, y0), self.pt(x1, y1));
        let r = Rect::from_min_max(pos2(snap(a.x), snap(a.y)), pos2(snap(b.x), snap(b.y)));
        if let Some(fill) = fill {
            self.p.rect_filled(r, 0.0, fill);
        }
        self.p.rect_stroke(r, 0.0, Stroke::new(width, stroke), egui::StrokeKind::Middle);
    }
}

/// Paints menu picture `name` into `r` in line colour `c` and accent `a`; false when there is
/// no picture of that name.
pub fn paint(p: &Painter, r: Rect, name: &str, c: Color32, a: Color32) -> bool {
    // A rectangle nothing can be drawn in still answers whether the picture exists.
    let r = if super::drawable(r) { r } else { Rect::from_min_size(Pos2::ZERO, vec2(0.0, 0.0)) };
    let pen = Pen { p, r, c, a, w: 1.0 };
    if let Some(k) = name.strip_prefix("margins.") {
        return margins(&pen, k);
    }
    if let Some(k) = name.strip_prefix("orientation.") {
        return orientation(&pen, k);
    }
    if let Some(k) = name.strip_prefix("columns.") {
        return columns(&pen, k);
    }
    if let Some(k) = name.strip_prefix("break.") {
        return breaks(&pen, k);
    }
    if let Some(k) = name.strip_prefix("textDirection.") {
        return text_direction(&pen, k);
    }
    if let Some(k) = name.strip_prefix("lineNumbers.") {
        return line_numbers(&pen, k);
    }
    if let Some(k) = name.strip_prefix("wrap.") {
        return wrap(&pen, k);
    }
    if let Some(k) = name.strip_prefix("rotate.") {
        return rotate(&pen, k);
    }
    if let Some(k) = name.strip_prefix("align.") {
        return align(&pen, k);
    }
    if let Some(k) = name.strip_prefix("dropCap.") {
        return drop_cap(&pen, k);
    }
    if let Some(k) = name.strip_prefix("border.") {
        return border(&pen, k);
    }
    if let Some(k) = name.strip_prefix("paraSpacing.") {
        return para_spacing(&pen, k);
    }
    if let Some(k) = name.strip_prefix("space.") {
        return space(&pen, k);
    }
    if let Some(k) = name.strip_prefix("header.") {
        return header_footer(&pen, k, true);
    }
    if let Some(k) = name.strip_prefix("footer.") {
        return header_footer(&pen, k, false);
    }
    if let Some(k) = name.strip_prefix("pageNumber.") {
        return page_number(&pen, k);
    }
    if let Some(k) = name.strip_prefix("toc.") {
        return toc(&pen, k);
    }
    if name == "coverPage" {
        return cover_page(&pen);
    }
    if let Some(k) = name.strip_prefix("page.") {
        return paper(&pen, k);
    }
    if let Some(k) = name.strip_prefix("find.").or_else(|| name.strip_prefix("select.")) {
        return find_select(&pen, k);
    }
    false
}

/// The colour of text lines: the line colour, lighter.
fn ink(pen: &Pen) -> Color32 {
    pen.c.gamma_multiply(0.55)
}

/// The colour of a picture or shape in the text: the accent, light.
fn tint(pen: &Pen) -> Color32 {
    pen.a.linear_multiply(0.3)
}

/// Text lines from `x0` to `x1` at each of `ys`, in the text colour.
fn text_lines(pen: &Pen, x0: f32, x1: f32, ys: &[f32]) {
    for &y in ys {
        if x1 > x0 {
            pen.line_c(&[(x0, y), (x1, y)], ink(pen));
        }
    }
}

/// A plain page outline.
fn sheet(pen: &Pen, x0: f32, y0: f32, x1: f32, y1: f32) {
    pen.rect(x0, y0, x1, y1, pen.c);
}

/// A dashed line from (`x0`, `y`) to (`x1`, `y`).
fn dashed(pen: &Pen, x0: f32, x1: f32, y: f32, c: Color32) {
    let mut x = x0;
    while x < x1 {
        pen.line_c(&[(x, y), ((x + 1.2).min(x1), y)], c);
        x += 2.0;
    }
}

/// A dashed line from (`x`, `y0`) to (`x`, `y1`).
fn dashed_v(pen: &Pen, x: f32, y0: f32, y1: f32, c: Color32) {
    let mut y = y0;
    while y < y1 {
        pen.line_c(&[(x, y), (x, (y + 1.2).min(y1))], c);
        y += 2.0;
    }
}

/// A page with its text area (inside the margins) tinted and outlined in the accent.
fn margined_page(pen: &Pen, page: (f32, f32, f32, f32), m: (f32, f32, f32, f32)) {
    let (x0, y0, x1, y1) = page;
    let (left, top, right, bottom) = m;
    pen.crisp_rect(x0, y0, x1, y1, pen.c, None);
    pen.crisp_rect(x0 + left, y0 + top, x1 - right, y1 - bottom, pen.a, Some(tint(pen)));
}

/// Layout › Margins: the page and its text area, the margins in proportion.
fn margins(pen: &Pen, k: &str) -> bool {
    let page = (4.5, 1.5, 15.5, 18.5);
    match k {
        "normal" => margined_page(pen, page, (2.4, 2.4, 2.4, 2.4)),
        "narrow" => margined_page(pen, page, (1.2, 1.2, 1.2, 1.2)),
        "moderate" => margined_page(pen, page, (1.8, 2.4, 1.8, 2.4)),
        "wide" => margined_page(pen, page, (4.0, 2.4, 4.0, 2.4)),
        "office2003" => margined_page(pen, page, (3.0, 2.4, 3.0, 2.4)),
        // Facing pages: the inside margins are the wider ones.
        "mirrored" => {
            margined_page(pen, (1.0, 3.0, 9.6, 17.0), (1.4, 1.8, 2.6, 1.8));
            margined_page(pen, (10.4, 3.0, 19.0, 17.0), (2.6, 1.8, 1.4, 1.8));
        }
        _ => return false,
    }
    true
}

/// Layout › Orientation: an upright or a sideways page with lines of text.
fn orientation(pen: &Pen, k: &str) -> bool {
    match k {
        "portrait" => {
            sheet(pen, 5.0, 1.5, 15.0, 18.5);
            text_lines(pen, 7.0, 13.0, &[4.5, 6.5, 8.5, 10.5, 12.5, 14.5]);
        }
        "landscape" => {
            sheet(pen, 1.5, 5.0, 18.5, 15.0);
            text_lines(pen, 3.5, 16.5, &[7.5, 9.5, 11.5, 13.0]);
        }
        _ => return false,
    }
    true
}

/// Layout › Columns: a page whose text runs in the preset's columns.
fn columns(pen: &Pen, k: &str) -> bool {
    let cols: &[(f32, f32)] = match k {
        "one" => &[(5.5, 14.5)],
        "two" => &[(5.5, 9.5), (10.5, 14.5)],
        "three" => &[(5.5, 8.0), (8.8, 11.2), (12.0, 14.5)],
        "left" => &[(5.5, 7.8), (8.8, 14.5)],
        "right" => &[(5.5, 11.2), (12.2, 14.5)],
        _ => return false,
    };
    sheet(pen, 3.5, 1.5, 16.5, 18.5);
    for &(x0, x1) in cols {
        text_lines(pen, x0, x1, &[4.0, 6.0, 8.0, 10.0, 12.0, 14.0, 16.0]);
    }
    true
}

/// Layout › Breaks: where the text stops and where it picks up again.
fn breaks(pen: &Pen, k: &str) -> bool {
    let a = pen.a;
    match k {
        // The end of one page and the top of the next.
        "page" | "nextPage" => {
            pen.line(&[(4.0, 1.5), (4.0, 8.0), (16.0, 8.0), (16.0, 1.5)]);
            text_lines(pen, 6.0, 14.0, &[3.0, 5.5]);
            pen.line(&[(4.0, 18.5), (4.0, 12.0), (16.0, 12.0), (16.0, 18.5)]);
            text_lines(pen, 6.0, 14.0, &[14.5, 17.0]);
            if k == "nextPage" {
                dashed(pen, 1.0, 19.0, 10.0, a);
            } else {
                pen.arrow_down(10.0, 10.0, a);
            }
        }
        // The text leaves the first column for the top of the second.
        "column" => {
            sheet(pen, 3.0, 1.5, 17.0, 18.5);
            text_lines(pen, 5.0, 9.0, &[4.0, 6.0, 8.0]);
            text_lines(pen, 11.0, 15.0, &[4.0, 6.0, 8.0, 10.0, 12.0, 14.0, 16.0]);
            pen.line_c(&[(7.0, 10.5), (7.0, 16.0), (10.0, 16.0)], a);
            pen.line_c(&[(10.0, 16.0), (10.0, 3.0), (10.8, 3.0)], a);
        }
        // A line ends early beside a picture and the text continues below it.
        "textWrapping" => {
            text_lines(pen, 2.0, 18.0, &[3.0, 5.5]);
            text_lines(pen, 2.0, 8.5, &[8.0]);
            pen.frect(11.0, 7.0, 18.0, 12.5, tint(pen));
            pen.rect(11.0, 7.0, 18.0, 12.5, a);
            pen.line_c(&[(9.5, 8.0), (9.5, 10.0), (3.5, 10.0)], a);
            pen.fill(&[(2.2, 10.0), (3.8, 9.0), (3.8, 11.0)], a);
            text_lines(pen, 2.0, 18.0, &[15.0, 17.5]);
        }
        // The new section starts on the same page.
        "continuous" => {
            sheet(pen, 3.5, 1.5, 16.5, 18.5);
            text_lines(pen, 5.5, 14.5, &[4.0, 6.5]);
            dashed(pen, 3.5, 16.5, 10.0, a);
            text_lines(pen, 5.5, 14.5, &[13.0, 15.5]);
        }
        // The new section starts on the next even (or odd) page.
        "evenPage" | "oddPage" => {
            let (first, second) = if k == "evenPage" { ("2", "4") } else { ("1", "3") };
            sheet(pen, 1.5, 3.0, 9.5, 17.0);
            sheet(pen, 10.5, 3.0, 18.5, 17.0);
            pen.text(5.5, 10.0, 7.0, first, pen.c, true);
            pen.text(14.5, 10.0, 7.0, second, a, true);
        }
        _ => return false,
    }
    true
}

/// Layout › Text Direction: lines of text running across, down or up, with an arrow.
fn text_direction(pen: &Pen, k: &str) -> bool {
    let a = pen.a;
    match k {
        "horizontal" => {
            text_lines(pen, 2.5, 13.0, &[5.0, 8.5, 12.0]);
            pen.line_c(&[(2.5, 16.0), (16.0, 16.0)], a);
            pen.fill(&[(18.0, 16.0), (15.5, 14.5), (15.5, 17.5)], a);
        }
        "down" => {
            for x in [15.0, 11.5, 8.0] {
                pen.line_c(&[(x, 2.5), (x, 13.0)], ink(pen));
            }
            pen.line_c(&[(4.0, 2.5), (4.0, 16.0)], a);
            pen.fill(&[(4.0, 18.0), (2.5, 15.5), (5.5, 15.5)], a);
        }
        "up" => {
            for x in [5.0, 8.5, 12.0] {
                pen.line_c(&[(x, 7.0), (x, 17.5)], ink(pen));
            }
            pen.line_c(&[(16.0, 17.5), (16.0, 4.0)], a);
            pen.fill(&[(16.0, 2.0), (14.5, 4.5), (17.5, 4.5)], a);
        }
        _ => return false,
    }
    true
}

/// Layout › Line Numbers: lines of text with (or without) numbers in the margin.
fn line_numbers(pen: &Pen, k: &str) -> bool {
    let ys = [3.5, 7.5, 11.5, 15.5];
    let numbers: &[&str] = match k {
        "none" => &[],
        "continuous" => &["1", "2", "3", "4"],
        // A page or section boundary between the second and third lines starts again at 1.
        "restartPage" | "restartSection" => &["1", "2", "1", "2"],
        _ => return false,
    };
    text_lines(pen, 8.0, 18.0, &ys);
    for (n, y) in numbers.iter().zip(ys) {
        pen.text(4.0, y, 5.5, n, pen.a, true);
    }
    match k {
        "restartPage" => pen.line(&[(1.0, 9.5), (19.0, 9.5)]),
        "restartSection" => dashed(pen, 1.0, 19.0, 9.5, pen.c),
        _ => {}
    }
    true
}

/// Wrap Text: how the lines of text run around a picture.
fn wrap(pen: &Pen, k: &str) -> bool {
    let a = pen.a;
    let picture = |x0: f32, y0: f32, x1: f32, y1: f32| {
        pen.frect(x0, y0, x1, y1, tint(pen));
        pen.rect(x0, y0, x1, y1, a);
    };
    let ys = [3.0, 5.5, 8.0, 10.5, 13.0, 15.5, 18.0];
    match k {
        // The picture sits on a line, like a big letter.
        "inLine" | "inline" => {
            text_lines(pen, 2.0, 18.0, &[3.0, 5.5, 15.5, 18.0]);
            text_lines(pen, 2.0, 6.0, &[12.5]);
            picture(7.0, 7.5, 13.0, 13.0);
            text_lines(pen, 14.0, 18.0, &[12.5]);
        }
        // Lines stop at the picture's frame on both sides.
        "square" => {
            text_lines(pen, 2.0, 18.0, &[3.0, 18.0]);
            for y in [5.5, 8.0, 10.5, 13.0, 15.5] {
                text_lines(pen, 2.0, 6.5, &[y]);
                text_lines(pen, 13.5, 18.0, &[y]);
            }
            picture(7.5, 4.5, 12.5, 16.5);
        }
        // Lines follow the picture's own outline.
        "tight" | "through" => {
            let tri = [(10.0, 4.5), (14.5, 16.5), (5.5, 16.5)];
            pen.fill(&tri, tint(pen));
            pen.closed(&tri, a);
            for &y in &ys {
                // The outline's left and right edges at this height (a triangle).
                let (l, r) = if (4.5..=16.5).contains(&y) {
                    let half = (y - 4.5) / 12.0 * 4.5;
                    (10.0 - half - 1.0, 10.0 + half + 1.0)
                } else {
                    (20.0, 0.0)
                };
                if l > r {
                    text_lines(pen, 2.0, 18.0, &[y]);
                } else {
                    text_lines(pen, 2.0, l, &[y]);
                    text_lines(pen, r, 18.0, &[y]);
                }
            }
            if k == "through" {
                // Text runs into the picture's open middle too.
                text_lines(pen, 9.0, 11.0, &[13.0]);
            }
        }
        // Text above and below, nothing beside.
        "topAndBottom" => {
            text_lines(pen, 2.0, 18.0, &[2.5, 5.0, 15.0, 17.5]);
            picture(6.0, 6.8, 14.0, 13.2);
        }
        // The picture under the text: the lines are drawn over it.
        "behindText" => {
            picture(5.0, 4.0, 15.0, 16.0);
            text_lines(pen, 2.0, 18.0, &ys);
        }
        // The picture over the text: it hides the lines under it.
        "inFrontOfText" => {
            text_lines(pen, 2.0, 18.0, &ys);
            pen.frect(5.0, 4.0, 15.0, 16.0, pen.a.linear_multiply(0.55));
            pen.rect(5.0, 4.0, 15.0, 16.0, a);
        }
        _ => return false,
    }
    true
}

/// Rotate: a shape, then where it ends up (a curved arrow, or a mirror line).
fn rotate(pen: &Pen, k: &str) -> bool {
    let a = pen.a;
    let tri = |pts: [(f32, f32); 3], fill: bool| {
        if fill {
            pen.fill(&pts, tint(pen));
        }
        pen.closed(&pts, if fill { a } else { pen.c });
    };
    match k {
        "right" | "left" => {
            tri([(4.0, 16.0), (9.0, 16.0), (4.0, 8.0)], true);
            let (start, end): (f32, f32) = if k == "right" { (200.0, 330.0) } else { (340.0, 210.0) };
            let pts: Vec<(f32, f32)> = (0..=12)
                .map(|i| {
                    let d = (start + (end - start) * i as f32 / 12.0).to_radians();
                    (12.0 + 5.5 * d.cos(), 10.0 + 5.5 * d.sin())
                })
                .collect();
            pen.line(&pts);
            // The arrowhead points the way the arc runs at its end.
            if let [.., (px, py), (x, y)] = pts[..] {
                let (dx, dy) = (x - px, y - py);
                let len = (dx * dx + dy * dy).sqrt().max(0.01);
                let (tx, ty) = (dx / len, dy / len);
                pen.fill(&[(x + tx * 2.2, y + ty * 2.2), (x - ty * 1.7, y + tx * 1.7), (x + ty * 1.7, y - tx * 1.7)], pen.c);
            }
        }
        "flipVertical" => {
            tri([(5.0, 8.5), (15.0, 8.5), (5.0, 2.5)], false);
            dashed(pen, 2.0, 18.0, 10.0, pen.c);
            tri([(5.0, 11.5), (15.0, 11.5), (5.0, 17.5)], true);
        }
        "flipHorizontal" => {
            tri([(8.5, 5.0), (8.5, 15.0), (2.5, 15.0)], false);
            dashed_v(pen, 10.0, 2.0, 18.0, pen.c);
            tri([(11.5, 5.0), (11.5, 15.0), (17.5, 15.0)], true);
        }
        _ => return false,
    }
    true
}

/// Align: an object against the left edge, the middle or the right edge.
fn align(pen: &Pen, k: &str) -> bool {
    let a = pen.a;
    let bars = |x: f32, widths: [f32; 2], anchor: f32| {
        // Two bars lined up on `x`: `anchor` 0 left, 0.5 centre, 1 right.
        for (w, (y0, y1)) in widths.iter().zip([(4.0, 8.5), (11.5, 16.0)]) {
            let x0 = x - w * anchor;
            pen.frect(x0, y0, x0 + w, y1, tint(pen));
            pen.rect(x0, y0, x0 + w, y1, a);
        }
    };
    match k {
        "left" => {
            pen.line(&[(3.0, 2.0), (3.0, 18.0)]);
            bars(4.5, [12.0, 8.0], 0.0);
        }
        "center" => {
            pen.line(&[(10.0, 2.0), (10.0, 18.0)]);
            bars(10.0, [12.0, 7.0], 0.5);
        }
        "right" => {
            pen.line(&[(17.0, 2.0), (17.0, 18.0)]);
            bars(15.5, [12.0, 8.0], 1.0);
        }
        _ => return false,
    }
    true
}

/// Insert › Drop Cap: a paragraph whose first letter is ordinary, dropped into the text, or in the margin.
fn drop_cap(pen: &Pen, k: &str) -> bool {
    let ys = [3.5, 7.0, 10.5, 14.0, 17.5];
    match k {
        "none" => {
            pen.text(4.0, 3.5, 5.0, "A", pen.a, true);
            text_lines(pen, 6.5, 18.0, &ys[..1]);
            text_lines(pen, 2.0, 18.0, &ys[1..]);
        }
        "dropped" => {
            pen.text(4.6, 7.0, 13.0, "A", pen.a, true);
            text_lines(pen, 9.0, 18.0, &ys[..3]);
            text_lines(pen, 2.0, 18.0, &ys[3..]);
        }
        "margin" => {
            pen.line_c(&[(6.5, 1.5), (6.5, 18.5)], ink(pen));
            pen.text(3.2, 7.0, 10.0, "A", pen.a, true);
            text_lines(pen, 8.0, 18.0, &ys);
        }
        _ => return false,
    }
    true
}

/// Borders: a two-by-two grid, dotted, with the sides the entry sets drawn solid.
fn border(pen: &Pen, k: &str) -> bool {
    let (x0, y0, x1, y1, m) = (3.0, 3.0, 17.0, 17.0, 10.0);
    let faint = pen.c.gamma_multiply(0.45);
    let dots = |pts: [(f32, f32); 2]| {
        let [(ax, ay), (bx, by)] = pts;
        let n = 14;
        for i in 0..=n {
            if i % 2 == 0 {
                let s = i as f32 / n as f32;
                pen.fcircle(ax + (bx - ax) * s, ay + (by - ay) * s, 0.45, faint);
            }
        }
    };
    let top = [(x0, y0), (x1, y0)];
    let bottom = [(x0, y1), (x1, y1)];
    let left = [(x0, y0), (x0, y1)];
    let right = [(x1, y0), (x1, y1)];
    let mid_h = [(x0, m), (x1, m)];
    let mid_v = [(m, y0), (m, y1)];
    let all = [top, bottom, left, right, mid_h, mid_v];
    let solid: Vec<[(f32, f32); 2]> = match k {
        "bottom" => vec![bottom],
        "top" => vec![top],
        "left" => vec![left],
        "right" => vec![right],
        "none" => vec![],
        "all" => all.to_vec(),
        "outside" => vec![top, bottom, left, right],
        "inside" => vec![mid_h, mid_v],
        "insideHorizontal" => vec![mid_h],
        "insideVertical" => vec![mid_v],
        // A line across the text, with a paragraph above and below.
        "horizontalLine" => {
            text_lines(pen, 3.0, 17.0, &[4.0, 6.5]);
            pen.frect(2.0, 9.4, 18.0, 10.6, pen.c.gamma_multiply(0.7));
            text_lines(pen, 3.0, 17.0, &[13.5, 16.0]);
            return true;
        }
        _ => return false,
    };
    for side in all {
        if !solid.contains(&side) {
            dots(side);
        }
    }
    for side in solid {
        pen.p.line_segment([pen.pt(side[0].0, side[0].1), pen.pt(side[1].0, side[1].1)], Stroke::new(1.6 * pen.s(), pen.a));
    }
    true
}

/// Design › Paragraph Spacing: two paragraphs, their lines and the gap between them in the
/// preset's proportions (`none` packs everything; `double` spreads it out).
fn para_spacing(pen: &Pen, k: &str) -> bool {
    // (line pitch, space after a paragraph), in grid units: the preset's proportions.
    let (pitch, gap): (f32, f32) = match k {
        "default" => (1.7, 1.6),
        "none" => (1.6, 0.0),
        "compact" => (1.6, 0.8),
        "tight" => (1.5, 1.2),
        "open" => (1.9, 1.9),
        "relaxed" => (2.1, 1.9),
        "double" => (2.8, 1.9),
        _ => return false,
    };
    // A page: a heading in the accent, then paragraphs of three lines, the last one short, as far
    // as the page goes.
    let (x0, y0, x1, y1) = (3.5, 1.0, 16.5, 19.0);
    pen.crisp_rect(x0, y0, x1, y1, pen.c, None);
    pen.p.line_segment([pen.pt(5.5, 3.6), pen.pt(12.0, 3.6)], Stroke::new(1.4 * pen.s(), pen.a));
    let mut y = 3.6 + 1.6 + gap.max(0.6);
    'page: for _ in 0..6 {
        for line in 0..3 {
            if y > y1 - 1.2 {
                break 'page;
            }
            let end = if line == 2 { 10.5 } else { 14.5 };
            pen.line_c(&[(5.5, y), (end, y)], ink(pen));
            y += pitch;
        }
        y += gap;
    }
    true
}

/// Layout › Size: a page in the paper's proportions and at its size relative to the largest
/// paper the menu lists (`page.612x792`: width and height in points), as Word draws them, so A5
/// reads smaller than A4 and an envelope reads long and thin.
fn paper(pen: &Pen, k: &str) -> bool {
    let Some((w, h)) = k.split_once('x').and_then(|(w, h)| Some((w.parse::<f32>().ok()?, h.parse::<f32>().ok()?))) else {
        return false;
    };
    if !(w.is_finite() && h.is_finite() && w > 0.0 && h > 0.0) {
        return false;
    }
    // The largest listed paper (A3 and Tabloid are about 1,200 points tall) fills the grid.
    let scale = 18.0 / 1224.0;
    let (pw, ph) = ((w * scale).clamp(4.0, 18.0), (h * scale).clamp(4.0, 18.0));
    let (x0, y0) = (10.0 - pw / 2.0, 10.0 - ph / 2.0);
    pen.crisp_rect(x0, y0, x0 + pw, y0 + ph, pen.c, Some(pen.c.gamma_multiply(0.08)));
    true
}

/// Home › Find and Select: magnifier, magnifier with options, go to an arrow; the selections.
fn find_select(pen: &Pen, k: &str) -> bool {
    let a = pen.a;
    let magnifier = |cx: f32, cy: f32, r: f32| {
        pen.p.circle_stroke(pen.pt(cx, cy), r * pen.s(), pen.stroke(pen.c));
        let d = r * 0.72;
        pen.p.line_segment([pen.pt(cx + d, cy + d), pen.pt(cx + d + 4.0, cy + d + 4.0)], Stroke::new(2.0 * pen.s(), pen.c));
    };
    match k {
        "find" => magnifier(8.5, 8.5, 5.0),
        // Find with its options: the magnifier over a list of settings.
        "advanced" => {
            magnifier(7.0, 7.0, 4.2);
            for (i, y) in [12.5, 15.0, 17.5].into_iter().enumerate() {
                pen.fcircle(12.0, y, 0.8, a);
                pen.line_c(&[(13.8, y), (if i == 1 { 17.0 } else { 18.5 }, y)], ink(pen));
            }
        }
        // Go To: an arrow to a page.
        "goto" => {
            pen.crisp_rect(11.0, 3.0, 18.0, 17.0, pen.c, None);
            pen.line_c(&[(2.0, 10.0), (9.0, 10.0)], a);
            pen.fill(&[(10.5, 10.0), (7.5, 7.5), (7.5, 12.5)], a);
        }
        // Select All: every line highlighted.
        "all" => {
            for y in [4.0, 8.0, 12.0, 16.0] {
                pen.frect(2.5, y - 1.3, 17.5, y + 1.3, tint(pen));
                pen.line_c(&[(3.5, y), (if y == 16.0 { 11.0 } else { 16.5 }, y)], pen.c);
            }
        }
        // Select Paragraph: one paragraph of three highlighted, its mark at the end.
        "paragraph" => {
            text_lines(pen, 2.5, 17.5, &[3.0]);
            pen.frect(2.0, 5.6, 18.0, 13.6, tint(pen));
            text_lines(pen, 3.0, 17.0, &[7.0, 10.0]);
            pen.line_c(&[(3.0, 12.5), (10.0, 12.5)], pen.c);
            pen.text(12.0, 12.4, 3.6, "¶", a, true);
            text_lines(pen, 2.5, 17.5, &[16.5]);
        }
        // Select Sentence: part of a line highlighted, up to its full stop.
        "sentence" => {
            text_lines(pen, 2.5, 17.5, &[5.0, 15.0]);
            pen.frect(2.0, 8.6, 13.0, 11.4, tint(pen));
            pen.line_c(&[(2.5, 10.0), (12.0, 10.0)], pen.c);
            pen.fcircle(12.6, 10.6, 0.5, a);
            text_lines(pen, 14.0, 17.5, &[10.0]);
        }
        // Select Text with Similar Formatting: the bold words of three lines highlighted.
        "similar" => {
            for (y, x0, x1) in [(4.5, 2.5, 7.5), (10.0, 10.0, 15.0), (15.5, 5.0, 10.0)] {
                text_lines(pen, 2.5, 17.5, &[y]);
                pen.frect(x0 - 0.5, y - 1.5, x1 + 0.5, y + 1.5, tint(pen));
                pen.p.line_segment([pen.pt(x0, y), pen.pt(x1, y)], Stroke::new(2.0 * pen.s(), pen.a));
            }
        }
        _ => return false,
    }
    true
}

/// Home › Line and Paragraph Spacing: add space before, remove space after.
fn space(pen: &Pen, k: &str) -> bool {
    let a = pen.a;
    match k {
        "addBefore" => {
            pen.line_c(&[(4.0, 3.0), (4.0, 8.0)], a);
            pen.fill(&[(4.0, 2.0), (2.5, 4.2), (5.5, 4.2)], a);
            pen.fill(&[(4.0, 9.0), (2.5, 6.8), (5.5, 6.8)], a);
            text_lines(pen, 8.0, 18.0, &[11.0, 14.0, 17.0]);
        }
        "removeAfter" => {
            text_lines(pen, 8.0, 18.0, &[3.0, 6.0, 9.0]);
            pen.fill(&[(4.0, 14.5), (2.5, 12.0), (5.5, 12.0)], a);
            pen.fill(&[(4.0, 14.5), (2.5, 17.0), (5.5, 17.0)], a);
        }
        _ => return false,
    }
    true
}

/// A page's top (header) or bottom (footer) strip with the preset's text blocks in it: `blank`
/// one placeholder at the left, `blankThree` three across, `title` a centred title, `pageNumber`
/// a centred number.
fn header_footer(pen: &Pen, k: &str, top: bool) -> bool {
    let (x0, x1) = (2.0, 18.0);
    let (edge, rule, band) = if top { (3.0, 9.0, 6.0) } else { (17.0, 11.0, 14.0) };
    // The page edge, the page's sides and the line between the header or footer and the body.
    pen.line(&[(x0, if top { 18.0 } else { 2.0 }), (x0, edge), (x1, edge), (x1, if top { 18.0 } else { 2.0 })]);
    dashed(pen, x0, x1, rule, pen.c.gamma_multiply(0.5));
    text_lines(pen, 4.0, 16.0, if top { &[12.0, 14.5] } else { &[5.5, 8.0] });
    let block = |x0: f32, x1: f32| {
        pen.frect(x0, band - 1.0, x1, band + 1.0, tint(pen));
        pen.rect(x0, band - 1.0, x1, band + 1.0, pen.a);
    };
    match k {
        "blank" => block(3.5, 9.0),
        "blankThree" => {
            block(3.5, 7.0);
            block(8.3, 11.7);
            block(13.0, 16.5);
        }
        "title" => pen.line_c(&[(6.0, band), (14.0, band)], pen.a),
        "pageNumber" => pen.text(10.0, band, 4.5, "1", pen.a, true),
        _ => return false,
    }
    true
}

/// Insert › Page Number: where the number goes on the page.
fn page_number(pen: &Pen, k: &str) -> bool {
    sheet(pen, 4.0, 1.5, 16.0, 18.5);
    let a = pen.a;
    match k {
        "top" => {
            pen.text(13.5, 3.8, 3.5, "1", a, true);
            text_lines(pen, 6.0, 14.0, &[7.0, 9.0, 11.0, 13.0, 15.0]);
        }
        "bottom" => {
            text_lines(pen, 6.0, 14.0, &[4.0, 6.0, 8.0, 10.0, 12.0]);
            pen.text(10.0, 16.2, 3.5, "1", a, true);
        }
        // "1 of 3" at the bottom: the number, a short rule and the count.
        "xOfY" => {
            text_lines(pen, 6.0, 14.0, &[4.0, 6.0, 8.0, 10.0, 12.0]);
            pen.text(8.2, 16.2, 3.5, "1", a, true);
            pen.line_c(&[(9.4, 16.2), (10.6, 16.2)], a);
            pen.text(11.8, 16.2, 3.5, "3", a, true);
        }
        // At the caret, in the middle of the text.
        "current" => {
            text_lines(pen, 6.0, 14.0, &[4.0, 6.0, 14.0, 16.0]);
            text_lines(pen, 6.0, 8.5, &[10.0]);
            pen.text(10.0, 10.0, 3.5, "1", a, true);
            pen.line_c(&[(11.5, 8.5), (11.5, 11.5)], a);
            text_lines(pen, 12.5, 14.0, &[10.0]);
        }
        _ => return false,
    }
    true
}

/// References › Table of Contents: a title over entries indented by level, with dotted leaders
/// to their page numbers; `levels` is how many levels the table lists.
fn toc(pen: &Pen, k: &str) -> bool {
    let levels = match k {
        "3" => 3,
        "2" => 2,
        _ => return false,
    };
    sheet(pen, 2.5, 1.5, 17.5, 18.5);
    pen.line_c(&[(4.5, 4.0), (11.0, 4.0)], pen.a);
    let pattern: &[usize] = if levels == 3 { &[0, 1, 2, 1, 0, 1] } else { &[0, 1, 1, 0, 1, 0] };
    for (i, level) in pattern.iter().enumerate() {
        let y = 7.0 + i as f32 * 2.0;
        let x = 4.5 + *level as f32 * 1.6;
        let end = x + 4.0 - *level as f32 * 0.6;
        pen.line_c(&[(x, y), (end, y)], if *level == 0 { pen.c } else { ink(pen) });
        let mut d = end + 0.8;
        while d < 14.0 {
            pen.fcircle(d, y, 0.2, ink(pen));
            d += 0.9;
        }
        pen.line_c(&[(14.5, y), (15.5, y)], pen.c);
    }
    true
}

/// Insert › Cover Page: a page with a band of colour, a title and a subtitle.
fn cover_page(pen: &Pen) -> bool {
    sheet(pen, 4.0, 1.5, 16.0, 18.5);
    pen.frect(4.0, 6.0, 16.0, 11.0, tint(pen));
    pen.line_c(&[(5.5, 8.0), (13.5, 8.0)], pen.a);
    pen.line_c(&[(5.5, 9.6), (11.0, 9.6)], ink(pen));
    text_lines(pen, 5.5, 9.0, &[16.5]);
    true
}
