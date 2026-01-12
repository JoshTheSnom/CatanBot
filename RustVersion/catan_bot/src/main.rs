use eframe::egui;
use egui::{Color32, Pos2, Stroke};
use std::collections::BTreeSet;
use thiserror::Error;

// const HEX_SIZE: f32 = 50.0;
// const INTERSECTION_RADIUS: f32 = HEX_SIZE * 0.12;
// const INTERSECTION_RADIUS_HIGHLIGHT: f32 = HEX_SIZE * 0.18;
// const NUMBER_FONT_SIZE: f32 = HEX_SIZE * 0.28;
// const INTERSECTION_FONT_SIZE: f32 = HEX_SIZE * 0.22;

#[derive(Default, Clone, Copy, Debug, PartialEq)]
enum Resource {
    Wood,
    Brick,
    Sheep,
    Wheat,
    Ore,
    #[default]
    Desert,
}
#[derive(Clone, Copy, Default)]
struct Hex {
    id: u8,
    q: i32,
    r: i32,
    res: Resource,
    num: Option<u8>,
}

struct Intersection {
    pos: egui::Pos2,
    value: u8, //pip sum
}

struct CatanApp {
    hexes: Vec<Hex>,
    selected_hex: usize,
    import_text: String,
    import_error: Option<String>,
}

impl CatanApp {
    fn new() -> Self {
        Self {
            hexes: DEFAULT_HEXES.to_vec(),
            selected_hex: 0,
            import_text: String::new(),
            import_error: None,
        }
    }
}

#[derive(Debug, Error)]
enum InputHexParseError {
    #[error("resource with name '{0}' doesn't exist.")]
    InvalidResource(String),
    #[error("number '{0}' is invalid.")]
    InvalidNumber(u8),
    #[error("invalid hex format at tile {0}: '{1}'")]
    InvalidFormat(usize, String),
    #[error("board must have exactly 19 tiles.")]
    InvalidInputLength,
}

impl TryInto<Resource> for &str {
    type Error = InputHexParseError;

    fn try_into(self) -> Result<Resource, Self::Error> {
        match self.to_lowercase().as_str() {
            "wood" => Ok(Resource::Wood),
            "brick" => Ok(Resource::Brick),
            "sheep" => Ok(Resource::Sheep),
            "wheat" => Ok(Resource::Wheat),
            "ore" => Ok(Resource::Ore),
            "desert" => Ok(Resource::Desert),
            other => Err(InputHexParseError::InvalidResource(other.to_string())),
        }
    }
}

const DEFAULT_HEXES: [Hex; 19] = [
    Hex {
        id: 1,
        res: Resource::Ore,
        num: Some(10),
        q: 0,
        r: -2,
    },
    Hex {
        id: 2,
        res: Resource::Sheep,
        num: Some(2),
        q: 1,
        r: -2,
    },
    Hex {
        id: 3,
        res: Resource::Wood,
        num: Some(9),
        q: 2,
        r: -2,
    },
    Hex {
        id: 4,
        res: Resource::Wheat,
        num: Some(12),
        q: -1,
        r: -1,
    },
    Hex {
        id: 5,
        res: Resource::Brick,
        num: Some(6),
        q: 0,
        r: -1,
    },
    Hex {
        id: 6,
        res: Resource::Sheep,
        num: Some(4),
        q: 1,
        r: -1,
    },
    Hex {
        id: 7,
        res: Resource::Brick,
        num: Some(10),
        q: 2,
        r: -1,
    },
    Hex {
        id: 8,
        res: Resource::Wheat,
        num: Some(9),
        q: -2,
        r: 0,
    },
    Hex {
        id: 9,
        res: Resource::Wood,
        num: Some(11),
        q: -1,
        r: 0,
    },
    Hex {
        id: 10,
        res: Resource::Desert,
        num: None,
        q: 0,
        r: 0,
    },
    Hex {
        id: 11,
        res: Resource::Wood,
        num: Some(3),
        q: 1,
        r: 0,
    },
    Hex {
        id: 12,
        res: Resource::Ore,
        num: Some(8),
        q: 2,
        r: 0,
    },
    Hex {
        id: 13,
        res: Resource::Wood,
        num: Some(8),
        q: -2,
        r: 1,
    },
    Hex {
        id: 14,
        res: Resource::Ore,
        num: Some(3),
        q: -1,
        r: 1,
    },
    Hex {
        id: 15,
        res: Resource::Wheat,
        num: Some(4),
        q: 0,
        r: 1,
    },
    Hex {
        id: 16,
        res: Resource::Sheep,
        num: Some(5),
        q: 1,
        r: 1,
    },
    Hex {
        id: 17,
        res: Resource::Brick,
        num: Some(5),
        q: -2,
        r: 2,
    },
    Hex {
        id: 18,
        res: Resource::Wheat,
        num: Some(6),
        q: -1,
        r: 2,
    },
    Hex {
        id: 19,
        res: Resource::Sheep,
        num: Some(11),
        q: 0,
        r: 2,
    },
];

fn axial_to_pixel(q: i32, r: i32, size: f32) -> Pos2 {
    let x = size * (3.0_f32.sqrt() * (q as f32 + r as f32 / 2.0));
    let y = size * (3.0 / 2.0 * r as f32);
    Pos2::new(x, y)
}

fn hex_corners(center: Pos2, size: f32) -> Vec<Pos2> {
    (0..6)
        .map(|i| {
            let angle = (60.0 * i as f32 + 30.0).to_radians();
            Pos2::new(center.x + size * angle.cos(), center.y + size * angle.sin())
        })
        .collect()
}
fn resource_colors(res: Resource) -> Color32 {
    match res {
        Resource::Wood => Color32::from_hex("#2D8C24").unwrap(),
        Resource::Brick => Color32::from_hex("#D9532B").unwrap(),
        Resource::Sheep => Color32::from_hex("#78B800").unwrap(),
        Resource::Wheat => Color32::from_hex("#F2CB30").unwrap(),
        Resource::Ore => Color32::from_hex("#A9ADAE").unwrap(),
        Resource::Desert => Color32::from_hex("#DFD8B1").unwrap(),
    }
}

fn number_color(num: u8) -> Color32 {
    if num == 6 || num == 8 {
        Color32::WHITE
    } else {
        Color32::BLACK
    }
}

fn value_color(value: u8, top: &[u8]) -> Option<Color32> {
    match top.iter().position(|&v| v == value) {
        Some(0) => Some(Color32::from_hex("#ffd700").unwrap()),
        Some(1) => Some(Color32::from_hex("#c0c0c0").unwrap()),
        Some(2) => Some(Color32::from_hex("#cd7f32").unwrap()),
        _ => None,
    }
}

fn pip_count(num: u8) -> u8 {
    let n = num as i32;
    (6 - (n - 7).abs()) as u8
}

fn draw_pips(painter: &egui::Painter, center: Pos2, num: u8, size: f32, color: Color32) {
    let count = pip_count(num);
    let radius = size * 0.03;
    let spacing = radius * 2.8;
    let y_offset = size * 0.35;

    let total_width = (count as f32 - 1.0) * spacing;
    let start_x = center.x - total_width / 2.0;

    for i in 0..count {
        let x = start_x + i as f32 * spacing;
        let y = center.y + y_offset;

        painter.circle_filled(Pos2::new(x, y), radius, color);
    }
}

fn parse_import(text: &str) -> Result<Vec<Hex>, InputHexParseError> {
    let entries: Vec<&str> = text.split(',').map(|s| s.trim()).collect();
    if entries.len() != 19 {
        return Err(InputHexParseError::InvalidInputLength); // must have exactly 19 tiles... may do an alternative later
    }

    let mut result = Vec::with_capacity(19);

    for (i, entry) in entries.iter().enumerate() {
        let parts: Vec<&str> = entry.split_whitespace().collect();
        let hex = match parts.len() {
            1 if parts[0].eq_ignore_ascii_case("desert") => Hex {
                id: (i + 1) as u8,
                res: Resource::Desert,
                num: None,
                q: DEFAULT_HEXES[i].q,
                r: DEFAULT_HEXES[i].r,
            },
            2 => {
                let res: Resource = parts[0].try_into()?;
                let num: u8 = parts[1]
                    .parse()
                    .map_err(|_| InputHexParseError::InvalidFormat(i + 1, entry.to_string()))?;

                if num < 2 || num > 12 || num == 7 {
                    return Err(InputHexParseError::InvalidNumber(num));
                }

                Hex {
                    id: (i + 1) as u8,
                    res,
                    num: Some(num),
                    q: DEFAULT_HEXES[i].q,
                    r: DEFAULT_HEXES[i].r,
                }
            }
            _ => return Err(InputHexParseError::InvalidFormat(i + 1, entry.to_string())),
        };
        result.push(hex);
    }
    Ok(result)
}

fn compute_intersections(hexes: &[Hex], hex_size: f32) -> Vec<Intersection> {
    let mut raw_points: Vec<(Pos2, u8)> = Vec::new();

    for h in hexes {
        let center = axial_to_pixel(h.q, h.r, hex_size);

        let pips = h.num.map(pip_count).unwrap_or(0);

        for corner in hex_corners(center, hex_size) {
            raw_points.push((corner, pips));
        }
    }

    let mut intersections: Vec<(Pos2, u8)> = Vec::new();
    let merge_dist = hex_size * 0.2;

    'outer: for (pos, pips) in raw_points {
        for (existing_pos, value) in intersections.iter_mut() {
            if existing_pos.distance(pos) < merge_dist {
                *value += pips;
                continue 'outer;
            }
        }
        intersections.push((pos, pips));
    }

    intersections
        .into_iter()
        .map(|(pos, value)| Intersection { pos, value })
        .collect()
}

impl eframe::App for CatanApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::SidePanel::right("editor").show(ctx, |ui| {
            ui.heading("Board editor");

            egui::ComboBox::from_label("Select Hex")
                .selected_text(format!("Hex {}", self.hexes[self.selected_hex].id))
                .show_ui(ui, |ui| {
                    for (i, h) in self.hexes.iter().enumerate() {
                        ui.selectable_value(&mut self.selected_hex, i, format!("Hex {}", h.id));
                    }
                });

            let hex = &mut self.hexes[self.selected_hex];

            ui.separator();
            ui.label("Resource");

            egui::ComboBox::from_id_salt("resource")
                .selected_text(format!("{:?}", hex.res))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut hex.res, Resource::Wood, "Wood");
                    ui.selectable_value(&mut hex.res, Resource::Brick, "Brick");
                    ui.selectable_value(&mut hex.res, Resource::Sheep, "Sheep");
                    ui.selectable_value(&mut hex.res, Resource::Wheat, "Wheat");
                    ui.selectable_value(&mut hex.res, Resource::Ore, "Ore");
                    ui.selectable_value(&mut hex.res, Resource::Desert, "Desert");
                });

            ui.separator();
            ui.label("Number");

            if hex.res == Resource::Desert {
                hex.num = None;
                ui.label("Desert has no number");
            } else {
                let mut num = hex.num.unwrap_or(6);
                ui.add(egui::Slider::new(&mut num, 2..=12).clamping(egui::SliderClamping::Always));
                if num == 7 {
                    num = 6;
                }
                hex.num = Some(num);
            }

            ui.separator();
            ui.label("Import Board (comma-separated)");
            ui.text_edit_multiline(&mut self.import_text);

            if ui.button("Apply Import").clicked() {
                match parse_import(&self.import_text) {
                    Ok(new_board) => {
                        self.hexes = new_board;
                        self.selected_hex = 0;
                        self.import_error = None;
                    }
                    Err(e) => {
                        self.import_error = Some(e.to_string());
                    }
                }
            }

            if let Some(err) = &self.import_error {
                ui.colored_label(Color32::RED, err);
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let painter = ui.painter();
            let hex_size = 60.0;

            let center = ui.available_rect_before_wrap().center();

            for h in &self.hexes {
                let pos = axial_to_pixel(h.q, h.r, hex_size) + center.to_vec2();

                let corners = hex_corners(pos, hex_size);

                painter.add(egui::Shape::convex_polygon(
                    corners,
                    resource_colors(h.res),
                    Stroke::new(2.0, Color32::BLACK),
                ));

                if let Some(num) = h.num {
                    let font_size = 28.0 - ((num as f32 - 7.0).abs() * 2.5);
                    let color = number_color(num);

                    //TODO: make font bold
                    painter.text(
                        pos,
                        egui::Align2::CENTER_CENTER,
                        num.to_string(),
                        egui::FontId::new(font_size, egui::FontFamily::Proportional),
                        color,
                    );

                    draw_pips(painter, pos, num, hex_size, color);
                }
            }

            let intersections = compute_intersections(&self.hexes, hex_size);

            let mut values = BTreeSet::new();
            for i in &intersections {
                values.insert(i.value);
            }

            let top_values: Vec<u8> = values.iter().rev().take(3).cloned().collect();

            for inter in intersections {
                let pos = inter.pos + center.to_vec2();

                if let Some(color) = value_color(inter.value, &top_values) {
                    painter.circle_filled(pos, 7.0, color);
                    painter.circle_stroke(pos, 7.0, Stroke::new(2.0, Color32::BLACK));

                    painter.text(
                        pos,
                        egui::Align2::CENTER_CENTER,
                        inter.value.to_string(),
                        egui::FontId::proportional(12.0),
                        Color32::BLACK,
                    );
                } else {
                    painter.circle_filled(pos, 7.0, Color32::BLACK);

                    painter.text(
                        pos,
                        egui::Align2::CENTER_CENTER,
                        inter.value.to_string(),
                        egui::FontId::proportional(12.0),
                        Color32::WHITE,
                    );
                }
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Catan Board",
        options,
        Box::new(|_cc| Ok(Box::new(CatanApp::new()))),
    )
}

// TODO:
// - Make numbers bold (probably have to import a ttf)
// - Allow importing using text (like "ore 10, sheep 2, wood 9")
// - Calculate and display the "value" of each intersection (based on pips)
// - Random board generator, do the spiral thing (A-R counterclockwise) (5 2 6 3 8 10 9 12 11 4 8 10 9 4 5 6 3 11)
// - Actual bot stuff: Set player count and colors, place buildings and roads
