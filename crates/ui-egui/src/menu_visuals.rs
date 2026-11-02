//! Drop-down menus that show what their entries do, as Word's do: a picture beside each entry
//! (margins, columns, wrapping, borders…), galleries of tiles (numbering and multilevel list
//! libraries, themes, watermarks, picture styles) and entries drawn in what they apply (theme
//! fonts in their own faces, theme colours as swatches, line weights as lines).
//!
//! Built to be shared by every Craft app (WordCraft, DeckCraft, GridCraft…): nothing here knows
//! about the document, the session or the app. The widgets take only egui, a [`MenuPalette`]
//! (made from the app's theme tokens in one place, [`MenuPalette::get`]) and, for entries drawn in
//! a font, a [`FontSamples`] the app implements. Running commands, translating labels and
//! knowing which entry is current stay with the caller (`ribbon::common::mi_pic` in WordCraft).
//!
//! Every picture is our own drawing ([`glyphs`]); fonts are rendered with the installed faces or
//! their open substitutes. Each entry keeps the name it had as a text entry for assistive
//! technology, tooltips and tests, and a selected entry reports itself as checked.

use egui::{Align2, Color32, Rect, Response, Sense, Stroke, TextureHandle, Ui, pos2, vec2};

use crate::theme::{regular, semibold};

pub mod glyphs;
#[cfg(test)]
mod tests;

/// The side of an entry's picture in a picture menu.
pub const PICTURE: f32 = 32.0;

/// Ink on a preview page: dark on the light page in either theme.
pub const PAGE_INK: Color32 = Color32::from_rgb(0x40, 0x40, 0x40);
/// Text lines on a preview page.
pub const PAGE_LINE: Color32 = Color32::from_rgb(0xA6, 0xA6, 0xA6);

/// Whether `r` can be drawn in: finite, with some width and height. Hostile sizes (a NaN from
/// a layout, an empty rectangle) draw nothing rather than reaching text layout with a NaN size.
pub fn drawable(r: Rect) -> bool {
    r.is_finite() && r.width() > 0.0 && r.height() > 0.0
}

/// `text` laid out on one line no wider than `max_w`: at `size` points when it fits, else smaller
/// down to `min_size`, else cut short with "…". Translations run longer than English (German,
/// Russian, Greek…), and a tile or swatch row has a fixed width, so a label shrinks or shortens
/// instead of spilling over its neighbours. The full text stays in the tooltip.
pub fn fitted(p: &egui::Painter, text: &str, max_w: f32, size: f32, min_size: f32, color: Color32) -> std::sync::Arc<egui::Galley> {
    let font = |pt: f32| regular(pt);
    let g = p.layout_no_wrap(text.to_string(), font(size), color);
    if g.size().x <= max_w || !max_w.is_finite() || max_w <= 0.0 {
        return g;
    }
    let pt = (size * max_w / g.size().x.max(1.0)).max(min_size);
    let g = p.layout_no_wrap(text.to_string(), font(pt), color);
    if g.size().x <= max_w {
        return g;
    }
    // Still too wide at the smallest size: drop characters from the end (whole characters,
    // never inside one) until it fits with the ellipsis.
    let chars: Vec<char> = text.chars().collect();
    let mut n = chars.len();
    while n > 1 {
        n -= 1;
        let cut: String = chars.iter().take(n).collect::<String>().trim_end().to_string() + "…";
        let g = p.layout_no_wrap(cut, font(pt), color);
        if g.size().x <= max_w {
            return g;
        }
    }
    p.layout_no_wrap("…".to_string(), font(pt), color)
}

/// The colours the menus are drawn in, taken from the app's theme.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MenuPalette {
    pub text: Color32,
    pub text_dim: Color32,
    pub text_disabled: Color32,
    /// Line work of pictures.
    pub icon: Color32,
    /// The accent of pictures, and the outline of the hovered or selected tile.
    pub accent: Color32,
    pub hover: Color32,
    pub pressed: Color32,
    /// The background of the selected entry or tile.
    pub checked: Color32,
    pub border: Color32,
    /// The paper previews are drawn on (dimmed in a dark theme).
    pub page: Color32,
}

impl From<&crate::theme::Tokens> for MenuPalette {
    fn from(t: &crate::theme::Tokens) -> Self {
        MenuPalette {
            text: t.text,
            text_dim: t.text_dim,
            text_disabled: t.text_disabled,
            icon: t.icon,
            accent: t.accent,
            hover: t.hover,
            pressed: t.pressed,
            checked: t.checked,
            border: t.border,
            page: crate::previews::preview_tint(t, false),
        }
    }
}

impl MenuPalette {
    /// The palette of the app's current theme.
    pub fn get(ctx: &egui::Context) -> Self {
        Self::from(&crate::theme::Tokens::get(ctx))
    }
}

/// Text rendered in a given font, for entries that show fonts in their own faces.
pub trait FontSamples {
    /// `text` in font `family` at `pt` points in a `w`×`h` point raster, as a white ink mask to
    /// tint; `None` while it isn't ready (the caller asks again next frame).
    fn font_sample(&mut self, ctx: &egui::Context, family: &str, text: &str, pt: f32, w: f32, h: f32) -> Option<TextureHandle>;
}

/// A menu entry with a check column (when `checked` is `Some`), a `size`-point picture drawn by
/// [`glyphs::paint`] or the app's icon set (`icons::paint(icon)`) and `label` (already in the
/// interface language), with `shortcut` at the right. Reports `label` to assistive technology
/// and, when checked, the toggled state.
pub fn picture_item(ui: &mut Ui, icon: &str, label: impl Into<egui::WidgetText>, checked: Option<bool>, size: f32, shortcut: &str) -> Response {
    let on = checked.unwrap_or(false);
    let mark = egui::Atom::paint(vec2(if checked.is_some() { 14.0 } else { 0.0 }, 14.0), move |ui, args| {
        if on {
            let r = args.rect;
            let points = vec![r.min + r.size() * vec2(0.15, 0.5), r.min + r.size() * vec2(0.4, 0.75), r.min + r.size() * vec2(0.85, 0.2)];
            ui.painter().add(egui::Shape::line(points, Stroke::new(1.5, args.fallback_text_color)));
        }
    });
    let enabled = ui.is_enabled();
    let picture = egui::Atom::paint(vec2(size, size), move |ui, args| {
        let pal = MenuPalette::get(ui.ctx());
        let (line, accent) = if enabled { (pal.icon, pal.accent) } else { (pal.text_disabled, pal.text_disabled) };
        crate::icons::paint(ui.painter(), args.rect, icon, line, accent);
    });
    let button = egui::Button::new((mark, picture, label.into())).selected(on).min_size(vec2(200.0, size + 4.0));
    let button = if shortcut.is_empty() { button } else { button.shortcut_text(shortcut) };
    ui.add(button)
}

/// An entry's label on two lines, as Word's picture menus set them: `title` in bold over
/// `detail` (sizes, margins), each in the entry's own text colour (hover and selection
/// included). The entry reads as both lines to assistive technology.
pub fn two_line(ui: &Ui, title: &str, detail: &str) -> egui::text::LayoutJob {
    let size = egui::TextStyle::Button.resolve(ui.style()).size;
    let mut job = egui::text::LayoutJob::default();
    let format = |font| egui::TextFormat { font_id: font, color: Color32::PLACEHOLDER, ..Default::default() };
    // The title in the interface's semibold face; where none is installed (a bare harness),
    // the regular one, rather than laying text out in a family with no fonts.
    let bold = semibold(size);
    let has_bold = ui.ctx().fonts(|f| f.definitions().families.get(&bold.family).is_some_and(|faces| !faces.is_empty()));
    job.append(title, 0.0, format(if has_bold { bold } else { regular(size) }));
    if !detail.is_empty() {
        job.append(&format!("\n{detail}"), 0.0, format(regular(size - 0.5)));
    }
    job
}

/// A menu entry showing a line `width` points thick (Weight, Thickness, Picture Border) before
/// `label`, checked when `checked`.
pub fn line_item(ui: &mut Ui, width: f32, label: &str, checked: bool) -> Response {
    let enabled = ui.is_enabled();
    let sample = egui::Atom::paint(vec2(96.0, 16.0), move |ui, args| {
        let pal = MenuPalette::get(ui.ctx());
        let c = if enabled { pal.text } else { pal.text_disabled };
        let y = args.rect.center().y;
        // At least a hairline on screen; the thickest stays inside the entry.
        let w = if width.is_finite() { width.clamp(0.5, 12.0) } else { 1.0 };
        ui.painter().line_segment([pos2(args.rect.left() + 2.0, y), pos2(args.rect.right() - 2.0, y)], Stroke::new(w, c));
    });
    ui.add(egui::Button::new((sample, label)).selected(checked).min_size(vec2(200.0, 22.0)))
}

/// A gallery tile `size` points big, named `label` (tooltip and assistive technology), drawn by
/// `paint` into its inner rectangle; `selected` marks the current choice.
pub fn tile(ui: &mut Ui, size: egui::Vec2, label: &str, selected: bool, paint: impl FnOnce(&Ui, Rect, &MenuPalette)) -> Response {
    let (r, resp) = ui.allocate_exact_size(size, Sense::click());
    let enabled = ui.is_enabled();
    resp.widget_info(|| egui::WidgetInfo::selected(egui::Role::Button, enabled, selected, label));
    let pal = MenuPalette::get(ui.ctx());
    if let Some(fill) = highlight(&resp, selected, &pal) {
        ui.painter().rect_filled(r, 4.0, fill);
    }
    let stroke = if selected || resp.hovered() { Stroke::new(1.0, pal.accent) } else { Stroke::new(1.0, pal.border) };
    ui.painter().rect_stroke(r.shrink(1.0), 3.0, stroke, egui::StrokeKind::Inside);
    paint(ui, r.shrink(4.0), &pal);
    if !enabled {
        ui.painter().rect_filled(r, 4.0, ui.visuals().window_fill.gamma_multiply(0.5));
    }
    resp.on_hover_text(label)
}

/// The fill behind a pressed, selected or hovered entry or tile.
fn highlight(resp: &Response, selected: bool, pal: &MenuPalette) -> Option<Color32> {
    if resp.is_pointer_button_down_on() {
        Some(pal.pressed)
    } else if resp.hovered() {
        Some(pal.hover)
    } else if selected {
        Some(pal.checked)
    } else {
        None
    }
}

/// The paper a tile previews on.
pub fn page(ui: &Ui, r: Rect, pal: &MenuPalette) {
    ui.painter().rect(r, 1.0, pal.page, Stroke::new(1.0, pal.border), egui::StrokeKind::Inside);
}

/// A numbering library tile: one list item per marker (already in the interface language), its
/// number and a line of text.
pub fn paint_numbering(ui: &Ui, r: Rect, pal: &MenuPalette, markers: &[String]) {
    if !drawable(r) {
        return;
    }
    page(ui, r, pal);
    let p = ui.painter();
    let n = markers.len().max(1) as f32;
    let pitch = r.height() / (n + 1.0);
    let x_num = r.left() + r.width() * 0.36;
    for (i, m) in markers.iter().enumerate() {
        let y = r.top() + pitch * (i as f32 + 1.0);
        p.text(pos2(x_num, y), Align2::RIGHT_CENTER, m, regular(10.0), PAGE_INK);
        p.line_segment([pos2(x_num + 4.0, y), pos2(r.right() - 6.0, y)], Stroke::new(1.5, PAGE_LINE));
    }
}

/// A multilevel list library tile: one row per level, each indented further, with its number
/// and either a line of text or (for lists linked to headings) the heading's name.
pub fn paint_multilevel(ui: &Ui, r: Rect, pal: &MenuPalette, levels: &[(String, Option<String>)]) {
    if !drawable(r) {
        return;
    }
    page(ui, r, pal);
    let p = ui.painter();
    let pitch = r.height() / (levels.len().max(1) as f32 + 1.0);
    let clip = p.with_clip_rect(r.shrink(1.0));
    for (i, (marker, text)) in levels.iter().enumerate() {
        let y = r.top() + pitch * (i as f32 + 1.0);
        let x = r.left() + 5.0 + i as f32 * 7.0;
        // A long number (`Article I.` in a longer language) shrinks to leave room for its text.
        let room = r.right() - 4.0 - x;
        let number = fitted(p, marker, room * if text.is_some() { 0.6 } else { 0.75 }, 8.5, 6.0, PAGE_INK);
        let w = number.size().x;
        clip.galley(pos2(x, y - number.size().y / 2.0), number, PAGE_INK);
        let x_text = x + w + 3.0;
        match text {
            Some(text) => {
                let g = fitted(p, text, (r.right() - 3.0 - x_text).max(1.0), 8.0, 6.0, PAGE_LINE);
                clip.galley(pos2(x_text, y - g.size().y / 2.0), g, PAGE_LINE);
            }
            None if x_text < r.right() - 8.0 => {
                p.line_segment([pos2(x_text, y), pos2(r.right() - 5.0, y)], Stroke::new(1.5, PAGE_LINE));
            }
            None => {}
        }
    }
}

/// A watermark tile: a page with `text` across it, diagonally in light grey, as the watermark is
/// laid on the page.
pub fn paint_watermark(ui: &Ui, r: Rect, pal: &MenuPalette, text: &str) {
    if !drawable(r) {
        return;
    }
    page(ui, r, pal);
    let p = ui.painter();
    let grey = Color32::from_rgb(0xB4, 0xB4, 0xB4);
    let diagonal = (r.width().powi(2) + r.height().powi(2)).sqrt();
    // The text as laid on the page: one line, or for several words two lines split at the space
    // nearest the middle, whichever reads larger along the diagonal (`NO COPIAR`, a long
    // `CONFIDENCIAL` in a longer language).
    let one = text.to_string();
    let two = text
        .char_indices()
        .filter(|(_, c)| *c == ' ')
        .min_by_key(|(i, _)| (2 * *i).abs_diff(text.len()))
        .map(|(i, _)| format!("{}\n{}", &text[..i], text[i..].trim_start()));
    let fit = |t: &str| {
        let probe = p.layout_no_wrap(t.to_string(), semibold(10.0), grey);
        let by_width = 10.0 * diagonal * 0.8 / probe.size().x.max(1.0);
        // Two lines also have to fit across the page's narrow way.
        let by_height = 10.0 * r.width().min(r.height()) * 0.55 / probe.size().y.max(1.0);
        let size = by_width.min(by_height);
        if size.is_finite() { size.clamp(5.0, 18.0) } else { 10.0 }
    };
    let (text, size) = match two {
        Some(two) if fit(&two) > fit(&one) * 1.15 => {
            let size = fit(&two);
            (two, size)
        }
        _ => {
            let size = fit(&one);
            (one, size)
        }
    };
    let mut job = egui::text::LayoutJob::simple(text, semibold(size), grey, f32::INFINITY);
    job.halign = egui::Align::Center;
    let galley = p.layout_job(job);
    let angle = -(r.height() / r.width().max(1.0)).atan();
    // Turn about the text's middle, which sits on the page's middle.
    let half = galley.size() / 2.0;
    let (s, c) = angle.sin_cos();
    let turned = vec2(half.x * c - half.y * s, half.x * s + half.y * c);
    // A centred job's galley starts left of its origin by half its width.
    let origin = r.center() - turned + vec2(half.x * c, half.x * s);
    p.with_clip_rect(r).add(egui::epaint::TextShape::new(origin, galley, grey).with_angle(angle));
}

/// A palette's colours as a row of `side`-point swatches from the left of `r`, centred in it.
pub fn paint_swatches(ui: &Ui, r: Rect, colors: &[Color32], side: f32, pal: &MenuPalette) {
    if !drawable(r) {
        return;
    }
    let p = ui.painter();
    for (i, c) in colors.iter().enumerate() {
        let sw = Rect::from_min_size(pos2(r.left() + i as f32 * (side + 2.0), r.center().y - side / 2.0), vec2(side, side));
        p.rect(sw, 1.0, *c, Stroke::new(1.0, pal.border), egui::StrokeKind::Inside);
    }
}

/// Draws a [`FontSamples`] texture with its top-left corner at `at`, tinted `color`; returns its
/// size in points.
pub fn draw_sample(ui: &Ui, tex: &TextureHandle, at: egui::Pos2, color: Color32) -> egui::Vec2 {
    let sz = tex.size_vec2() / ui.ctx().pixels_per_point();
    ui.painter().image(tex.id(), Rect::from_min_size(at, sz), Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)), color);
    sz
}

/// Theme Fonts entry: an "Aa" in the heading font, then the set's `name` over its heading and
/// body fonts, each named in its own face. A menu button like the others, so it takes the
/// menu's width without widening it; `label` (name and fonts) is what assistive technology reads.
pub fn theme_fonts_item(ui: &mut Ui, fonts: &mut dyn FontSamples, name: &str, heading: &str, body: &str, label: &str, selected: bool) -> Response {
    const LINE_W: f32 = 190.0;
    let ctx = ui.ctx().clone();
    let aa = fonts.font_sample(&ctx, heading, "Aa", 20.0, 36.0, 30.0);
    let samples = [heading, body].map(|family| fonts.font_sample(&ctx, family, family, 12.5, LINE_W, 16.0));
    let enabled = ui.is_enabled();
    let name_w = ui.painter().layout_no_wrap(name.to_string(), regular(10.5), Color32::PLACEHOLDER).size().x;
    let name = name.to_string();
    let aa_atom = egui::Atom::paint(vec2(40.0, 40.0), move |ui, args| {
        let pal = MenuPalette::get(ui.ctx());
        let ink = if enabled { pal.text } else { pal.text_disabled };
        ui.painter().rect_stroke(args.rect, 2.0, Stroke::new(1.0, pal.border), egui::StrokeKind::Inside);
        if let Some(tex) = &aa {
            let sz = tex.size_vec2() / ui.ctx().pixels_per_point();
            draw_sample(ui, tex, args.rect.center() - sz / 2.0 + vec2(4.0, 0.0), ink);
        }
    });
    let info = egui::Atom::paint(vec2(LINE_W.max(name_w), 46.0), move |ui, args| {
        let pal = MenuPalette::get(ui.ctx());
        let (ink, dim) = if enabled { (pal.text, pal.text_dim) } else { (pal.text_disabled, pal.text_disabled) };
        let r = args.rect;
        ui.painter().text(r.min, Align2::LEFT_TOP, &name, regular(10.5), dim);
        for (i, tex) in samples.iter().enumerate() {
            if let Some(tex) = tex {
                draw_sample(ui, tex, pos2(r.left(), r.top() + 14.0 + i as f32 * 16.0), ink);
            }
        }
    });
    let resp = ui.add(egui::Button::new((aa_atom, info)).selected(selected).min_size(vec2(200.0, 50.0)));
    resp.widget_info(|| egui::WidgetInfo::selected(egui::Role::Button, enabled, selected, label));
    resp
}

/// Theme Colors entry: the palette's swatches, then its `name`; a menu button like the others,
/// so it takes the menu's width without widening it.
pub fn theme_colors_item(ui: &mut Ui, name: &str, colors: &[Color32], selected: bool) -> Response {
    let colors = colors.to_vec();
    let swatches = egui::Atom::paint(vec2(colors.len() as f32 * 18.0, 16.0), move |ui, args| {
        let pal = MenuPalette::get(ui.ctx());
        paint_swatches(ui, args.rect, &colors, 16.0, &pal);
    });
    ui.add(egui::Button::new((swatches, name)).selected(selected).min_size(vec2(200.0, 26.0)))
}

/// A Themes tile: "Aa" in the theme's heading font over a bar of its accent colours, its `name`
/// below.
pub fn theme_tile(ui: &mut Ui, fonts: &mut dyn FontSamples, name: &str, heading: &str, colors: &[Color32], selected: bool) -> Response {
    let ctx = ui.ctx().clone();
    let sample = fonts.font_sample(&ctx, heading, "Aa", 22.0, 44.0, 30.0);
    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing = vec2(0.0, 2.0);
        let resp = tile(ui, vec2(84.0, 64.0), name, selected, |ui, r, pal| {
            page(ui, r, pal);
            if let Some(tex) = &sample {
                let sz = tex.size_vec2() / ctx.pixels_per_point();
                draw_sample(ui, tex, pos2(r.center().x - sz.x / 2.0 + 4.0, r.top() + 4.0), PAGE_INK);
            }
            let bars = Rect::from_min_max(pos2(r.left() + 4.0, r.bottom() - 14.0), pos2(r.right() - 4.0, r.bottom() - 4.0));
            let w = bars.width() / colors.len().max(1) as f32;
            for (i, c) in colors.iter().enumerate() {
                let b = Rect::from_min_size(pos2(bars.left() + i as f32 * w, bars.top()), vec2(w, bars.height()));
                ui.painter().rect_filled(b, 0.0, *c);
            }
        });
        // The name under the tile, shrunk or shortened to the tile's width (the tooltip has it whole).
        let pal = MenuPalette::get(ui.ctx());
        let (r, _) = ui.allocate_exact_size(vec2(84.0, 14.0), Sense::hover());
        let g = fitted(ui.painter(), name, r.width(), 10.5, 8.0, pal.text_dim);
        ui.painter().galley(pos2(r.center().x - g.size().x / 2.0, r.center().y - g.size().y / 2.0), g, pal.text_dim);
        resp
    })
    .inner
}

/// A picture style tile: a small landscape (sky, sun and hills, drawn here) in the style's frame
/// or effect: `simpleFrame`, `thickFrame`, `rounded`, `softEdge` or `shadow`.
pub fn paint_picture_style(ui: &Ui, r: Rect, style: &str) {
    if !drawable(r) {
        return;
    }
    let p = ui.painter();
    let inner = r.shrink(4.0);
    let picture = |p: &egui::Painter, r: Rect, radius: f32| {
        p.rect_filled(r, radius, Color32::from_rgb(0x9C, 0xC9, 0xEB));
        p.circle_filled(pos2(r.right() - r.width() * 0.25, r.top() + r.height() * 0.3), r.height() * 0.12, Color32::from_rgb(0xF6, 0xD3, 0x5B));
        // Two convex hills, overlapping.
        let green = Color32::from_rgb(0x5A, 0x9A, 0x55);
        let (l, rt, b) = (r.left() + radius * 0.3, r.right() - radius * 0.3, r.bottom());
        let peak1 = pos2(r.left() + r.width() * 0.35, r.top() + r.height() * 0.45);
        let peak2 = pos2(r.left() + r.width() * 0.72, r.top() + r.height() * 0.58);
        p.add(egui::Shape::convex_polygon(
            vec![pos2(l, b), pos2(l, r.top() + r.height() * 0.72), peak1, pos2(r.left() + r.width() * 0.62, b)],
            green,
            Stroke::NONE,
        ));
        p.add(egui::Shape::convex_polygon(
            vec![pos2(r.left() + r.width() * 0.4, b), peak2, pos2(rt, r.top() + r.height() * 0.66), pos2(rt, b)],
            green.gamma_multiply(0.85),
            Stroke::NONE,
        ));
    };
    match style {
        "simpleFrame" => {
            p.rect_filled(inner, 0.0, Color32::WHITE);
            p.rect_stroke(inner, 0.0, Stroke::new(1.0, Color32::from_gray(0xC0)), egui::StrokeKind::Inside);
            picture(p, inner.shrink(3.0), 0.0);
        }
        "thickFrame" => {
            p.rect_filled(inner.translate(vec2(1.5, 1.5)), 0.0, Color32::from_black_alpha(60));
            p.rect_filled(inner, 0.0, Color32::from_gray(0x30));
            picture(p, inner.shrink(5.0), 0.0);
        }
        "rounded" => picture(p, inner, inner.height() * 0.25),
        "softEdge" => {
            picture(p, inner, 0.0);
            // A fade to the menu's colour at every edge.
            let bg = ui.visuals().window_fill;
            for i in 0..5 {
                let a = 1.0 - i as f32 / 5.0;
                p.rect_stroke(inner.shrink(i as f32), 0.0, Stroke::new(1.2, bg.gamma_multiply(a)), egui::StrokeKind::Inside);
            }
        }
        "shadow" => {
            p.rect_filled(inner.translate(vec2(3.0, 3.0)), 1.0, Color32::from_black_alpha(90));
            picture(p, inner, 0.0);
        }
        _ => picture(p, inner, 0.0),
    }
}
