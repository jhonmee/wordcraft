use super::*;

/// The shapes painting `paint` produces in a 32-point square.
fn painted(mut paint: impl FnMut(&egui::Painter, Rect)) -> Vec<egui::epaint::ClippedShape> {
    let ctx = egui::Context::default();
    let out = ctx.run_ui(egui::RawInput::default(), |ui| {
        paint(ui.painter(), Rect::from_min_size(egui::Pos2::ZERO, vec2(32.0, 32.0)));
    });
    let shapes = out.shapes.clone();
    out.drop_without_applying_deltas();
    shapes
}

/// The menu pictures the ribbon's menus name.
const MENU_PICTURES: &[&str] = &[
    "margins.normal",
    "margins.narrow",
    "margins.moderate",
    "margins.wide",
    "margins.mirrored",
    "margins.office2003",
    "orientation.portrait",
    "orientation.landscape",
    "columns.one",
    "columns.two",
    "columns.three",
    "columns.left",
    "columns.right",
    "break.page",
    "break.column",
    "break.textWrapping",
    "break.nextPage",
    "break.continuous",
    "break.evenPage",
    "break.oddPage",
    "textDirection.horizontal",
    "textDirection.down",
    "textDirection.up",
    "lineNumbers.none",
    "lineNumbers.continuous",
    "lineNumbers.restartPage",
    "lineNumbers.restartSection",
    "wrap.inline",
    "wrap.square",
    "wrap.tight",
    "wrap.through",
    "wrap.topAndBottom",
    "wrap.behindText",
    "wrap.inFrontOfText",
    "rotate.right",
    "rotate.left",
    "rotate.flipVertical",
    "rotate.flipHorizontal",
    "align.left",
    "align.center",
    "align.right",
    "dropCap.none",
    "dropCap.dropped",
    "dropCap.margin",
    "border.bottom",
    "border.top",
    "border.left",
    "border.right",
    "border.none",
    "border.all",
    "border.outside",
    "border.inside",
    "border.horizontalLine",
    "paraSpacing.default",
    "paraSpacing.none",
    "paraSpacing.compact",
    "paraSpacing.tight",
    "paraSpacing.open",
    "paraSpacing.relaxed",
    "paraSpacing.double",
    "space.addBefore",
    "space.removeAfter",
    "header.blank",
    "header.blankThree",
    "header.title",
    "footer.blank",
    "footer.blankThree",
    "footer.pageNumber",
    "pageNumber.top",
    "pageNumber.bottom",
    "pageNumber.xOfY",
    "pageNumber.current",
    "toc.3",
    "toc.2",
    "coverPage",
    "page.612x792",
    "page.297x420",
    "find.find",
    "find.advanced",
    "find.goto",
    "select.all",
    "select.paragraph",
    "select.sentence",
    "select.similar",
];

/// A paper picture's size comes from its name: names that aren't two positive, finite numbers
/// draw nothing instead of a page of a NaN or negative size.
#[test]
fn paper_pictures_refuse_hostile_sizes() {
    for name in ["page.", "page.x", "page.612", "page.NaNx792", "page.-5x7", "page.0x0", "page.infx3", "page.1e40x2"] {
        let mut known = true;
        let shapes = painted(|p, r| known = glyphs::paint(p, r, name, Color32::BLACK, Color32::BLUE));
        assert!(!known && shapes.is_empty(), "{name} draws nothing");
    }
}

/// Every menu picture the menus name has a drawing of its own, and an unknown name has none
/// (it falls back to the icon set's lettered tile instead of drawing nothing).
#[test]
fn every_menu_picture_is_drawn() {
    for name in MENU_PICTURES {
        let mut known = false;
        let shapes = painted(|p, r| known = glyphs::paint(p, r, name, Color32::BLACK, Color32::BLUE));
        assert!(known, "{name} is a menu picture");
        assert!(!shapes.is_empty(), "{name} draws something");
    }
    let mut known = true;
    let shapes = painted(|p, r| known = glyphs::paint(p, r, "margins.unknown", Color32::BLACK, Color32::BLUE));
    assert!(!known && shapes.is_empty(), "an unknown picture draws nothing here");
}

/// The shapes Insert › Shapes added (#4349 follow-up) are drawn, not the lettered fallback tile.
#[test]
fn the_new_shape_tiles_have_drawings() {
    for kind in ["rightTriangle", "flowchartOr", "blockArrowRight", "arrowLeft", "arrowUp", "arrowDown"] {
        let shapes = painted(|p, r| crate::icons::paint(p, r, &format!("shape.{kind}"), Color32::BLACK, Color32::BLUE));
        assert!(!shapes.is_empty(), "{kind} draws something");
        assert!(!shapes.iter().any(|s| matches!(s.shape, egui::Shape::Text(_))), "{kind} is a drawing, not a lettered tile");
    }
}

/// Hostile sizes (zero, negative, NaN rectangles; NaN line widths) never panic.
#[test]
fn pictures_survive_degenerate_rectangles() {
    let ctx = egui::Context::default();
    let out = ctx.run_ui(egui::RawInput::default(), |ui| {
        for r in [Rect::NOTHING, Rect::from_min_size(egui::Pos2::ZERO, vec2(0.0, 0.0)), Rect::from_min_size(egui::Pos2::ZERO, vec2(f32::NAN, 4.0))] {
            for name in MENU_PICTURES {
                glyphs::paint(ui.painter(), r, name, Color32::BLACK, Color32::BLUE);
            }
            let pal = MenuPalette::get(ui.ctx());
            paint_watermark(ui, r, &pal, "DRAFT");
            paint_numbering(ui, r, &pal, &["1.".into(), "2.".into()]);
            paint_multilevel(ui, r, &pal, &[("1".into(), None), ("1.1".into(), Some("Heading 2".into()))]);
            paint_picture_style(ui, r, "softEdge");
        }
        line_item(ui, f32::NAN, "NaN", false);
    });
    out.drop_without_applying_deltas();
}

/// A picture entry carries its label and checked state for assistive technology, and a tile
/// its name and selection.
#[test]
fn entries_and_tiles_report_their_name_and_state() {
    use egui_kittest::kittest::{NodeT, Queryable};
    let mut h = egui_kittest::Harness::new_ui(|ui| {
        picture_item(ui, "menu.margins.narrow", "Narrow", Some(true), PICTURE, "");
        picture_item(ui, "menu.margins.wide", "Wide", Some(false), PICTURE, "");
        tile(ui, vec2(60.0, 60.0), "1. 2. 3.", true, |ui, r, pal| paint_numbering(ui, r, pal, &["1.".into()]));
        line_item(ui, 2.25, "2.25 pt", true);
    });
    h.run();
    let toggled = |label: &str| h.get_by_label(label).accesskit_node().toggled();
    assert_eq!(toggled("Narrow"), Some(egui::accesskit::Toggled::True));
    assert_eq!(toggled("Wide"), Some(egui::accesskit::Toggled::False));
    assert_eq!(toggled("1. 2. 3."), Some(egui::accesskit::Toggled::True));
    assert_eq!(toggled("2.25 pt"), Some(egui::accesskit::Toggled::True));
}
