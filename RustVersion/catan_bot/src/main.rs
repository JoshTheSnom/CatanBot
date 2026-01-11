use eframe::egui;
use egui::{Color32, Pos2, Stroke};

#[derive(Clone, Copy)]
struct Hex {
    id: u8,
    q: i32,
    r: i32,
    res: Resource,
    num: Option<u8>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Resource {
    Wood,
    Brick,
    Sheep,
    Wheat,
    Ore,
    Desert,
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

fn pip_count(num: u8) -> usize {
    let n = num as i32;
    let pips = 6 - (n - 7).abs();
    pips.max(0) as usize
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

struct CatanApp {
    hexes: Vec<Hex>,
    selected_hex: usize,
}

impl CatanApp {
    fn new() -> Self {
        Self {
            hexes: DEFAULT_HEXES.to_vec(),
            selected_hex: 0,
        }
    }
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
