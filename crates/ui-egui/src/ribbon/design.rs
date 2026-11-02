use super::common::{mi, mi_pic, mi_pic_text, stack, tile_rows};
use super::{CONTENT_H, Rect, Sense, Target, Ui, WordApp, big, group, json, menu_button, small, vec2};

/// The Design tab's style set gallery in a box of its own, sized from its thumbnails and clipped
/// to it, with a gap on each side: the Colors and Fonts column can't be drawn over, or cut by,
/// the thumbnails (#638). Returns the box and the space the thumbnails used.
pub(super) fn style_set_box(app: &mut WordApp, ui: &mut Ui) -> (Rect, Rect) {
    ui.add_space(4.0);
    let (gallery, _) = ui.allocate_exact_size(vec2(crate::previews::STYLE_SET_GALLERY_W, CONTENT_H), Sense::hover());
    let mut gui = ui.new_child(egui::UiBuilder::new().max_rect(gallery).layout(egui::Layout::left_to_right(egui::Align::Min)));
    gui.set_clip_rect(gallery.intersect(ui.clip_rect()));
    gui.spacing_mut().item_spacing = vec2(1.0, 1.0);
    crate::previews::style_set_gallery(app, &mut gui);
    ui.add_space(8.0);
    (gallery, gui.min_rect())
}

pub(super) fn design(app: &mut WordApp, ui: &mut Ui) {
    group(ui, "Themes", None, app, |ui, app| {
        menu_button(ui, app, crate::widgets::MenuButtonOptions::new("themes", Some("Themes"), "Themes", true, "design.themes"), themes_gallery);
    });
    group(ui, "Document Formatting", None, app, |ui, app| {
        style_set_box(app, ui);
        stack(ui, |ui| {
            menu_button(
                ui,
                app,
                crate::widgets::MenuButtonOptions::new("colors", Some("Colors"), "Theme Colors", false, "design.themeColors"),
                theme_colors_items,
            );
            menu_button(
                ui,
                app,
                crate::widgets::MenuButtonOptions::new("fonts", Some("Fonts"), "Theme Fonts", false, "design.themeFonts"),
                theme_fonts_items,
            );
        });
        stack(ui, |ui| {
            menu_button(
                ui,
                app,
                crate::widgets::MenuButtonOptions::new(
                    "paraSpacing",
                    Some("Paragraph Spacing"),
                    "Paragraph Spacing",
                    false,
                    "design.paragraphSpacing",
                ),
                |ui, app| {
                    for (l, v) in [
                        ("Default", "default"),
                        ("No Paragraph Space", "none"),
                        ("Compact", "compact"),
                        ("Tight", "tight"),
                        ("Open", "open"),
                        ("Relaxed", "relaxed"),
                        ("Double", "double"),
                    ] {
                        // "Open" here is roomy spacing, not the file verb: only a `spacing` row
                        // translates it, else it stays English.
                        let icon = format!("menu.paraSpacing.{v}");
                        if l == "Open" {
                            let text = crate::i18n::get("spacing", l).unwrap_or(l);
                            mi_pic_text(ui, app, &icon, text, None, "design.paragraphSpacing", json!({"value": v}));
                            continue;
                        }
                        mi_pic(ui, app, &icon, l, None, "design.paragraphSpacing", json!({"value": v}));
                    }
                },
            );
            small(ui, app, "effectsDesign", Some("Effects"), "Effects", crate::widgets::ButtonAction::new("design.effects", json!({}), false));
            small(
                ui,
                app,
                "setDefault",
                Some("Set as Default"),
                "Set as Default",
                crate::widgets::ButtonAction::new("design.setDefault", json!({}), false),
            );
        });
    });
    group(ui, "Page Background", None, app, |ui, app| {
        menu_button(
            ui,
            app,
            crate::widgets::MenuButtonOptions::new("watermark", Some("Watermark"), "Watermark", true, "design.watermark"),
            |ui, app| {
                watermark_gallery(ui, app);
                ui.separator();
                mi(ui, app, "Custom Watermark…", "ui.dialog", json!({"name": "watermark"}));
                mi(ui, app, "Remove Watermark", "design.watermark", json!({"remove": true}));
            },
        );
        menu_button(
            ui,
            app,
            crate::widgets::MenuButtonOptions::new("pageColor", Some("Page\nColor"), "Page Color", true, "design.pageColor"),
            |ui, app| {
                mi(ui, app, "No Color", "design.pageColor", json!({"color": null}));
                let theme = app.session.doc.settings.theme_colors.clone();
                crate::colors::menu(ui, app, &theme, Target::cmd("design.pageColor"));
            },
        );
        big(ui, app, "pageBorders", "Page\nBorders", "design.pageBorders", json!({}), false);
    });
}

/// Design › Themes: a tile per theme, "Aa" in its heading font over its colours; the
/// document's theme is selected.
fn themes_gallery(ui: &mut Ui, app: &mut WordApp) {
    let current = app.session.doc.settings.theme_name.clone();
    let enabled = crate::widgets::enabled(app, "design.theme");
    tile_rows(ui, wordcraft_engine::cmd::design::THEMES, 3, |ui, (name, heading, _, accents)| {
        let colors: Vec<egui::Color32> = accents.iter().map(|c| crate::theme::c32(*c)).collect();
        let on = current.eq_ignore_ascii_case(name);
        let resp = ui.add_enabled_ui(enabled, |ui| crate::menu_visuals::theme_tile(ui, &mut app.previews, tl!(name), heading, &colors, on)).inner;
        let clicked = resp.clicked();
        if clicked {
            crate::widgets::run_from_ribbon(app, "design.theme", json!({"name": name}));
        }
        clicked
    });
}

/// Design › Colors: each palette as a row of swatches and its name; the palette whose first
/// accent the document uses is selected.
fn theme_colors_items(ui: &mut Ui, app: &mut WordApp) {
    let accent = app.session.doc.settings.theme_colors.get(4).copied();
    let enabled = crate::widgets::enabled(app, "design.themeColors");
    for (name, _, _, accents) in wordcraft_engine::cmd::design::THEMES {
        let colors: Vec<egui::Color32> = accents.iter().map(|c| crate::theme::c32(*c)).collect();
        let on = accent == accents.first().copied();
        let resp = ui.add_enabled_ui(enabled, |ui| crate::menu_visuals::theme_colors_item(ui, tl!(name), &colors, on)).inner;
        if resp.clicked() {
            crate::widgets::run_from_ribbon(app, "design.themeColors", json!({"name": name}));
            ui.close();
        }
    }
}

/// Design › Fonts: each pair as "Aa" in its heading font and the two fonts named in their own
/// faces; the document's pair is selected. Each entry reads as `Name: Heading / Body`.
fn theme_fonts_items(ui: &mut Ui, app: &mut WordApp) {
    let settings = &app.session.doc.settings;
    let (major, minor) = (settings.major_font.clone(), settings.minor_font.clone());
    let enabled = crate::widgets::enabled(app, "design.themeFonts");
    for (name, h, b, _) in wordcraft_engine::cmd::design::THEMES {
        let label = format!("{}: {h} / {b}", tl!(name));
        let on = major.eq_ignore_ascii_case(h) && minor.eq_ignore_ascii_case(b);
        let resp = ui.add_enabled_ui(enabled, |ui| crate::menu_visuals::theme_fonts_item(ui, &mut app.previews, tl!(name), h, b, &label, on)).inner;
        if resp.clicked() {
            crate::widgets::run_from_ribbon(app, "design.themeFonts", json!({"heading": h, "body": b}));
            ui.close();
        }
    }
}

/// The watermarks Design › Watermark offers ready-made.
pub(super) const WATERMARKS: [&str; 6] = ["DRAFT", "CONFIDENTIAL", "DO NOT COPY", "SAMPLE", "ASAP", "URGENT"];

/// Design › Watermark's gallery: a page with each watermark laid across it.
fn watermark_gallery(ui: &mut Ui, app: &mut WordApp) {
    let enabled = crate::widgets::enabled(app, "design.watermark");
    tile_rows(ui, &WATERMARKS, 3, |ui, w| {
        let text = tl!(w);
        let resp = ui
            .add_enabled_ui(enabled, |ui| {
                crate::menu_visuals::tile(ui, vec2(72.0, 92.0), text, false, |ui, r, pal| crate::menu_visuals::paint_watermark(ui, r, pal, text))
            })
            .inner;
        let clicked = resp.clicked();
        if clicked {
            crate::widgets::run_from_ribbon(app, "design.watermark", json!({"text": w}));
        }
        clicked
    });
}
