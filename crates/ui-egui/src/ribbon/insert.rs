use super::common::{mi, mi_pic, stack};
use super::language::mi_check;
use super::{Ui, WordApp, big, group, json, menu_button, small};

/// A tile of Insert › Shapes: its name (tooltip, assistive technology), the `insert.shape`
/// kind, the picture drawn on it and, for lines and connectors, the arrowheads.
pub(super) struct ShapeTile {
    pub label: &'static str,
    pub kind: &'static str,
    pub icon: &'static str,
    pub arrow: Option<&'static str>,
}

const fn tile(label: &'static str, kind: &'static str, icon: &'static str) -> ShapeTile {
    ShapeTile { label, kind, icon, arrow: None }
}

const fn line(label: &'static str, kind: &'static str, icon: &'static str, arrow: &'static str) -> ShapeTile {
    ShapeTile { label, kind, icon, arrow: Some(arrow) }
}

/// Insert › Shapes, by category as Word groups them: every shape WordCraft draws. Names and
/// headings are looked up in the `shapes` context, so a language whose plain word for "Lines"
/// means lines of text, or "Diamond" the gem, says what the gallery shows.
pub(super) const SHAPE_CATEGORIES: [(&str, &[ShapeTile]); 6] = [
    (
        "Lines",
        &[
            tile("Line", "line", "shape.line"),
            line("Straight Connector", "straightConnector", "shape.straightConnector", "none"),
            line("Straight Arrow Connector", "straightConnector", "shape.straightArrowConnector", "end"),
            line("Elbow Connector", "elbowConnector", "shape.elbowConnector", "none"),
            line("Elbow Arrow Connector", "elbowConnector", "shape.elbowArrowConnector", "end"),
            line("Curved Connector", "curvedConnector", "shape.curvedConnector", "none"),
            line("Curved Arrow Connector", "curvedConnector", "shape.curvedArrowConnector", "end"),
        ],
    ),
    ("Rectangles", &[tile("Rectangle", "rectangle", "shape.rectangle"), tile("Rounded Rectangle", "roundedRectangle", "shape.roundedRectangle")]),
    (
        "Basic Shapes",
        &[
            tile("Oval", "ellipse", "shape.ellipse"),
            tile("Triangle", "triangle", "shape.triangle"),
            tile("Right Triangle", "rightTriangle", "shape.rightTriangle"),
            tile("Diamond", "diamond", "shape.diamond"),
            tile("Heart", "heart", "shape.heart"),
        ],
    ),
    (
        "Block Arrows",
        &[
            tile("Arrow: Right", "arrow", "shape.blockArrowRight"),
            tile("Arrow: Left", "arrowLeft", "shape.arrowLeft"),
            tile("Arrow: Up", "arrowUp", "shape.arrowUp"),
            tile("Arrow: Down", "arrowDown", "shape.arrowDown"),
        ],
    ),
    ("Flowchart", &[tile("Flowchart: Or", "flowchartOr", "shape.flowchartOr")]),
    ("Stars and Banners", &[tile("Star", "star", "shape.star")]),
];

/// Shapes per row of the Insert › Shapes grid.
const SHAPE_TILES_PER_ROW: usize = 8;

/// Insert › Shapes: each category's name over a grid of drawn tiles (#4349), each named by its
/// tooltip; a click inserts that shape.
pub(super) fn shape_gallery(ui: &mut Ui, app: &mut WordApp) {
    use crate::{icons, theme::Tokens};
    use egui::{Rect, Sense, vec2};
    let t = Tokens::get(ui.ctx());
    let enabled = crate::widgets::enabled(app, "insert.shape");
    ui.add_enabled_ui(enabled, |ui| {
        ui.spacing_mut().item_spacing = vec2(2.0, 2.0);
        for (category, tiles) in SHAPE_CATEGORIES {
            ui.label(egui::RichText::new(crate::i18n::tc("shapes", category)).small().strong());
            for row in tiles.chunks(SHAPE_TILES_PER_ROW) {
                ui.horizontal(|ui| {
                    for tile in row {
                        let (r, resp) = ui.allocate_exact_size(vec2(30.0, 30.0), Sense::click());
                        let name = crate::i18n::tc("shapes", tile.label);
                        resp.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, ui.is_enabled(), name));
                        crate::widgets::bg(ui, r, &resp, false, &t);
                        icons::paint(ui.painter(), Rect::from_center_size(r.center(), vec2(22.0, 22.0)), tile.icon, t.icon, t.accent);
                        if resp.on_hover_text(name).clicked() {
                            let params = match tile.arrow {
                                Some(arrow) => json!({"kind": tile.kind, "arrow": arrow}),
                                None => json!({"kind": tile.kind}),
                            };
                            crate::widgets::run_from_ribbon(app, "insert.shape", params);
                            ui.close();
                        }
                    }
                });
            }
            ui.add_space(2.0);
        }
    });
}

pub(super) fn insert(app: &mut WordApp, ui: &mut Ui) {
    group(ui, "Pages", None, app, |ui, app| {
        stack(ui, |ui| {
            menu_button(
                ui,
                app,
                crate::widgets::MenuButtonOptions::new("coverPage", Some("Cover Page"), "Cover Page", false, "insert.coverPage"),
                |ui, app| {
                    mi_pic(ui, app, "menu.coverPage", "Studio cover", None, "insert.coverPage", json!({}));
                },
            );
            small(ui, app, "blankPage", Some("Blank Page"), "Blank Page", crate::widgets::ButtonAction::new("insert.blankPage", json!({}), false));
            small(ui, app, "pageBreak", Some("Page Break"), "Page Break", crate::widgets::ButtonAction::new("insert.pageBreak", json!({}), false));
        });
    });
    group(ui, "Tables", None, app, |ui, app| {
        menu_button(ui, app, crate::widgets::MenuButtonOptions::new("table", Some("Table"), "Add a Table", true, "insert.table"), |ui, app| {
            crate::dialogs::table_grid_picker(ui, app);
            ui.separator();
            mi(ui, app, "Insert Table…", "ui.dialog", json!({"name": "insertTable"}));
            let drawing = app.canvas.table_tool == Some(crate::table_pen::TableTool::Draw);
            mi_check(ui, app, "Draw Table", drawing, "table.draw", json!({}));
            mi(ui, app, "Convert Text to Table…", "table.fromText", json!({}));
            ui.menu_button(tl!("Quick Tables"), |ui| {
                mi(ui, app, "Tabular List", "table.quick", json!({"kind": "tabular"}));
                mi(ui, app, "Matrix", "table.quick", json!({"kind": "matrix"}));
                mi(ui, app, "Calendar", "table.quick", json!({"kind": "calendar"}));
            });
        });
    });
    group(ui, "Illustrations", None, app, |ui, app| {
        big(ui, app, "picture", "Pictures", "insert.picture", json!({}), false);
        menu_button(ui, app, crate::widgets::MenuButtonOptions::new("shapes", Some("Shapes"), "Shapes", true, "insert.shape"), |ui, app| {
            shape_gallery(ui, app);
            ui.separator();
            // Connectors (under Lines) glue their ends to shapes by dragging them (shape.connect).
            // A menu button like the rest of the menu: it takes the menu's width, never widens it.
            let entry = super::common::PicEntry {
                icon: "canvas",
                label: tl!("New Drawing Canvas").into(),
                checked: None,
                size: super::common::SMALL_PICTURE,
            };
            super::common::mi_pic_sized(ui, app, entry, "insert.canvas", json!({}));
        });
        stack(ui, |ui| {
            small(ui, app, "icons", Some("Icons"), "Icons", crate::widgets::ButtonAction::new("insert.icon", json!({}), false));
            small(ui, app, "smartArt", Some("SmartArt"), "SmartArt", crate::widgets::ButtonAction::new("insert.smartArt", json!({}), false));
            small(ui, app, "chart", Some("Chart"), "Chart", crate::widgets::ButtonAction::new("insert.chart", json!({}), false));
        });
    });
    group(ui, "Links", None, app, |ui, app| {
        stack(ui, |ui| {
            small(ui, app, "link", Some("Link"), "Link", crate::widgets::ButtonAction::new("ui.dialog", json!({"name": "link"}), false));
            small(
                ui,
                app,
                "bookmark",
                Some("Bookmark"),
                "Bookmark",
                crate::widgets::ButtonAction::new("ui.dialog", json!({"name": "bookmark"}), false),
            );
            small(
                ui,
                app,
                "crossRef",
                Some("Cross-reference"),
                "Cross-reference",
                crate::widgets::ButtonAction::new("insert.crossReference", json!({}), false),
            );
        });
    });
    group(ui, "Comments", None, app, |ui, app| {
        big(ui, app, "newComment", "Comment", "review.newComment", json!({}), false);
    });
    group(ui, "Header & Footer", None, app, |ui, app| {
        menu_button(ui, app, crate::widgets::MenuButtonOptions::new("header", Some("Header"), "Header", true, "insert.header"), |ui, app| {
            for (l, k) in [("Blank", "blank"), ("Blank (Three Columns)", "blankThree"), ("Document Title", "title")] {
                mi_pic(ui, app, &format!("menu.header.{k}"), l, None, "insert.header", json!({"preset": k}));
            }
            ui.separator();
            mi(ui, app, "Edit Header", "insert.editHeader", json!({}));
            mi(ui, app, "Remove Header", "insert.removeHeader", json!({}));
        });
        menu_button(ui, app, crate::widgets::MenuButtonOptions::new("footer", Some("Footer"), "Footer", true, "insert.footer"), |ui, app| {
            for (l, k) in [("Blank", "blank"), ("Blank (Three Columns)", "blankThree"), ("Page Number", "pageNumber")] {
                mi_pic(ui, app, &format!("menu.footer.{k}"), l, None, "insert.footer", json!({"preset": k}));
            }
            ui.separator();
            mi(ui, app, "Edit Footer", "insert.editFooter", json!({}));
            mi(ui, app, "Remove Footer", "insert.removeFooter", json!({}));
        });
        menu_button(
            ui,
            app,
            crate::widgets::MenuButtonOptions::new("pageNumber", Some("Page\nNumber"), "Page Number", true, "insert.pageNumber"),
            |ui, app| {
                mi_pic(ui, app, "menu.pageNumber.top", "Top of Page", None, "insert.pageNumber", json!({"position": "top", "align": "right"}));
                mi_pic(
                    ui,
                    app,
                    "menu.pageNumber.bottom",
                    "Bottom of Page",
                    None,
                    "insert.pageNumber",
                    json!({"position": "bottom", "align": "center"}),
                );
                mi_pic(ui, app, "menu.pageNumber.xOfY", "Page X of Y", None, "insert.pageNumber", json!({"position": "bottom", "format": "x of y"}));
                mi_pic(ui, app, "menu.pageNumber.current", "Current Position", None, "insert.pageNumber", json!({"position": "current"}));
                mi(ui, app, "Format Page Numbers…", "ui.dialog", json!({"name": "pageNumbers"}));
            },
        );
    });
    group(ui, "Text", None, app, |ui, app| {
        big(ui, app, "textBox", "Text\nBox", "insert.textBox", json!({}), false);
        stack(ui, |ui| {
            menu_button(
                ui,
                app,
                crate::widgets::MenuButtonOptions::new("quickParts", Some("Quick Parts"), "Quick Parts", false, "insert.quickParts"),
                |ui, app| {
                    // This session's building blocks, then the document's own Quick Parts.
                    let mut parts: Vec<String> = app.session.building_blocks.keys().cloned().collect();
                    let mut seen: std::collections::HashSet<String> = parts.iter().cloned().collect();
                    let glossary = app.session.doc.glossary.iter().filter(|e| e.gallery == wordcraft_doc::QUICK_PARTS_GALLERY);
                    parts.extend(glossary.filter(|e| seen.insert(e.name.clone())).map(|e| e.name.clone()));
                    for name in parts {
                        if ui.button(&name).clicked() {
                            let _ = app.run("insert.quickParts", json!({"insert": name}));
                            ui.close();
                        }
                    }
                    mi(ui, app, "Field…", "ui.dialog", json!({"name": "field"}));
                },
            );
            menu_button(ui, app, crate::widgets::MenuButtonOptions::new("wordArt", Some("WordArt"), "WordArt", false, ""), |ui, app| {
                crate::wordart_menus::style_gallery(ui, app, "insert.wordArt")
            });
            menu_button(
                ui,
                app,
                crate::widgets::MenuButtonOptions::new("dropCap", Some("Drop Cap"), "Drop Cap", false, "insert.dropCap"),
                drop_cap_items,
            );
        });
        stack(ui, |ui| {
            small(ui, app, "signature", None, "Signature Line", crate::widgets::ButtonAction::new("insert.signatureLine", json!({}), false));
            small(ui, app, "dateTime", None, "Date & Time", crate::widgets::ButtonAction::new("insert.dateTime", json!({}), false));
            small(ui, app, "object", None, "Object", crate::widgets::ButtonAction::new("insert.object", json!({}), false));
        });
    });
    group(ui, "Symbols", None, app, |ui, app| {
        crate::equation_tab::insert_button(ui, app);
        menu_button(ui, app, crate::widgets::MenuButtonOptions::new("symbol", Some("Symbol"), "Symbol", true, "insert.symbol"), |ui, app| {
            // Recently used symbols first (their font on hover), then common ones.
            let mut items: Vec<(String, String)> = app.ui.recent_symbols.iter().map(|r| (r.ch.clone(), r.font.clone())).collect();
            for c in [
                "©", "®", "™", "§", "¶", "€", "£", "¥", "°", "±", "≠", "≤", "≥", "÷", "×", "∞", "µ", "α", "β", "π", "Ω", "∑", "√", "→", "←", "✓",
                "★", "♥", "—", "…",
            ] {
                if !items.iter().any(|(ch, f)| ch == c && f.is_empty()) {
                    items.push((c.to_string(), String::new()));
                }
            }
            items.truncate(30);
            egui::Grid::new("syms").show(ui, |ui| {
                for (i, (c, font)) in items.iter().enumerate() {
                    let b = ui.button(egui::RichText::new(c).size(16.0));
                    let b = if font.is_empty() { b } else { b.on_hover_text(font) };
                    if b.clicked() {
                        let params = if font.is_empty() { json!({"char": c}) } else { json!({"char": c, "font": font}) };
                        if app.run("insert.symbol", params).is_ok() {
                            crate::dialogs_insert::remember_symbol(app, c, font);
                        }
                        ui.close();
                    }
                    if i % 6 == 5 {
                        ui.end_row();
                    }
                }
            });
            ui.separator();
            mi(ui, app, "More Symbols…", "ui.dialog", json!({"name": "symbol"}));
        });
    });
}

/// Insert › Drop Cap's menu (#2601): None, Dropped and In margin (the current one checked),
/// then the Drop Cap dialog.
pub(super) fn drop_cap_items(ui: &mut Ui, app: &mut WordApp) {
    let current = wordcraft_engine::cmd::insert::drop_cap_state(&app.session).position;
    for (label, v, icon) in
        [("None", "none", "menu.dropCap.none"), ("Dropped", "dropped", "menu.dropCap.dropped"), ("In margin", "margin", "menu.dropCap.margin")]
    {
        mi_pic(ui, app, icon, label, Some(current == v), "insert.dropCap", json!({"position": v}));
    }
    ui.separator();
    mi(ui, app, "Drop Cap Options…", "ui.dialog", json!({"name": "dropCap"}));
}
