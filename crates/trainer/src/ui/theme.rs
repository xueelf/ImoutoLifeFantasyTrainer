//! 手绘风格的基础绘制组件：纸张、墨线、胶带、网点和按钮。

use eframe::egui::{
    self, Align2, Color32, CornerRadius, FontFamily, FontId, Mesh, Painter, Pos2, Rect, Shape,
    Stroke, StrokeKind, TextureId, Vec2,
};

pub const INK: Color32 = Color32::from_rgb(27, 27, 27);
pub const PAPER: Color32 = Color32::from_rgb(253, 252, 248);
pub const MENU_PAPER: Color32 = Color32::from_rgb(255, 253, 248);
pub const MUTED: Color32 = Color32::from_rgb(111, 106, 96);
pub const DISABLED: Color32 = Color32::from_rgb(196, 191, 179);
pub const LINE: Color32 = Color32::from_rgb(215, 211, 200);
pub const SHADOW: Color32 = Color32::from_rgba_unmultiplied_const(27, 27, 27, 45);
pub const SCREEN_TONE: Color32 = Color32::from_rgba_unmultiplied_const(27, 27, 27, 28);

pub fn list_separator(painter: &Painter, rect: Rect) {
    painter.line_segment(
        [
            Pos2::new(rect.left() + 6.0, rect.bottom()),
            Pos2::new(rect.right() - 6.0, rect.bottom()),
        ],
        Stroke::new(1.0, Color32::from_rgba_unmultiplied(215, 211, 200, 160)),
    );
}

pub fn font(size: f32) -> FontId {
    FontId::new(size, FontFamily::Proportional)
}

pub fn mono(size: f32) -> FontId {
    FontId::new(size, FontFamily::Monospace)
}

pub fn measure(ui: &egui::Ui, text: &str, font: FontId) -> Vec2 {
    ui.painter()
        .layout_no_wrap(text.to_owned(), font, INK)
        .size()
}

pub fn panel(painter: &Painter, rect: Rect, radius: f32, offset: Vec2) {
    if offset != Vec2::ZERO {
        painter.rect_filled(rect.translate(offset), CornerRadius::from(radius), SHADOW);
    }
    painter.rect(
        rect,
        CornerRadius::from(radius),
        MENU_PAPER,
        Stroke::new(2.0, INK),
        StrokeKind::Inside,
    );
}

pub fn wobbly_oval(painter: &Painter, rect: Rect, stroke: Stroke, seed: f32) {
    let center = rect.center();
    let rx = rect.width() * 0.5;
    let ry = rect.height() * 0.5;
    let points: Vec<Pos2> = (0..40)
        .map(|index| {
            let t = index as f32 / 40.0 * std::f32::consts::TAU;
            let wobble =
                1.0 + 0.018 * (5.0 * t + seed).sin() + 0.012 * (9.0 * t + seed * 1.7).sin();
            center + Vec2::new(t.cos() * rx * wobble, t.sin() * ry * wobble)
        })
        .collect();
    painter.add(Shape::closed_line(points, stroke));
}

pub fn screentone(painter: &Painter, rect: Rect) {
    // 用规则网点模拟漫画网点纸。
    const SPACING: f32 = 6.0;
    const RADIUS: f32 = 0.7;
    let mut y = rect.top() + SPACING * 0.5;
    while y < rect.bottom() {
        let mut x = rect.left() + SPACING * 0.5;
        while x < rect.right() {
            painter.circle_filled(Pos2::new(x, y), RADIUS, SCREEN_TONE);
            x += SPACING;
        }
        y += SPACING;
    }
}

pub fn wavy_line(painter: &Painter, x0: f32, x1: f32, y: f32, color: Color32) {
    let width = (x1 - x0).max(1.0);
    let steps = (width / 4.0).clamp(4.0, 48.0) as usize;
    let points: Vec<Pos2> = (0..=steps)
        .map(|index| {
            let t = index as f32 / steps as f32;
            let x = x0 + width * t;
            let wave = (t * std::f32::consts::TAU * 4.0).sin() * 0.7;
            Pos2::new(x, y + wave)
        })
        .collect();
    painter.add(Shape::line(points, Stroke::new(1.0, color)));
}

pub fn tape(painter: &Painter, center: Pos2, size: Vec2, angle: f32) {
    // 胶带上下边平直，左右两端画成锯齿切口。
    let half = size * 0.5;
    let notch = half.x - 4.0;
    let notch_y = half.y * 0.5;
    let local = [
        (-half.x, -half.y),
        (half.x, -half.y),
        (notch, -notch_y),
        (half.x, 0.0),
        (notch, notch_y),
        (half.x, half.y),
        (-half.x, half.y),
        (-notch, notch_y),
        (-half.x, 0.0),
        (-notch, -notch_y),
    ];

    let (sin, cos) = angle.sin_cos();
    let rotate = |point: (f32, f32)| {
        let offset = Vec2::new(point.0, point.1);
        center
            + Vec2::new(
                offset.x * cos - offset.y * sin,
                offset.x * sin + offset.y * cos,
            )
    };

    let fill = Color32::from_rgba_unmultiplied(242, 240, 235, 178);
    let mut mesh = Mesh::default();
    mesh.vertices.push(egui::epaint::Vertex {
        pos: rotate((0.0, 0.0)),
        uv: Pos2::ZERO,
        color: fill,
    });
    for point in local {
        mesh.vertices.push(egui::epaint::Vertex {
            pos: rotate(point),
            uv: Pos2::ZERO,
            color: fill,
        });
    }
    let count = local.len() as u32;
    for index in 0..count {
        mesh.indices
            .extend_from_slice(&[0, index + 1, (index + 1) % count + 1]);
    }
    painter.add(Shape::mesh(mesh));

    painter.line_segment(
        [
            rotate((-half.x + 6.0, -half.y + 4.0)),
            rotate((half.x - 6.0, -half.y + 4.0)),
        ],
        Stroke::new(1.0, Color32::from_rgba_unmultiplied(255, 255, 255, 80)),
    );

    painter.add(Shape::closed_line(
        local.map(rotate).to_vec(),
        Stroke::new(1.0, Color32::from_rgba_unmultiplied(27, 27, 27, 32)),
    ));
}

pub fn rotated_rect(
    painter: &Painter,
    center: Pos2,
    size: Vec2,
    angle: f32,
    fill: Color32,
    stroke: Stroke,
) {
    let points = rotated_points(center, size, angle);
    painter.add(Shape::convex_polygon(points.to_vec(), fill, stroke));
}

pub fn rotated_points(center: Pos2, size: Vec2, angle: f32) -> [Pos2; 4] {
    let (sin, cos) = angle.sin_cos();
    let half = size * 0.5;
    [
        Vec2::new(-half.x, -half.y),
        Vec2::new(half.x, -half.y),
        Vec2::new(half.x, half.y),
        Vec2::new(-half.x, half.y),
    ]
    .map(|offset| {
        center
            + Vec2::new(
                offset.x * cos - offset.y * sin,
                offset.x * sin + offset.y * cos,
            )
    })
}

pub fn rotated_texture(
    painter: &Painter,
    texture: TextureId,
    center: Pos2,
    size: Vec2,
    angle: f32,
) {
    let points = rotated_points(center, size, angle);
    let mut mesh = Mesh::with_texture(texture);
    let uvs = [
        Pos2::new(0.0, 0.0),
        Pos2::new(1.0, 0.0),
        Pos2::new(1.0, 1.0),
        Pos2::new(0.0, 1.0),
    ];
    for (position, uv) in points.into_iter().zip(uvs) {
        mesh.vertices.push(egui::epaint::Vertex {
            pos: position,
            uv,
            color: Color32::WHITE,
        });
    }
    mesh.indices.extend_from_slice(&[0, 1, 2, 0, 2, 3]);
    painter.add(Shape::mesh(mesh));
}

pub fn keycap(painter: &Painter, rect: Rect, label: &str, active: bool, font: FontId) {
    if !active {
        painter.rect_filled(
            rect.translate(Vec2::new(1.5, 1.5)),
            CornerRadius::from(2.0),
            Color32::from_rgba_unmultiplied(27, 27, 27, 70),
        );
    }
    let (fill, text_color) = if active {
        (INK, PAPER)
    } else {
        (Color32::WHITE, MUTED)
    };
    painter.rect(
        rect,
        CornerRadius::from(4.0),
        fill,
        Stroke::new(1.5, INK),
        StrokeKind::Inside,
    );
    painter.text(
        rect.center(),
        Align2::CENTER_CENTER,
        label,
        font,
        text_color,
    );
}

pub fn button(
    ui: &mut egui::Ui,
    rect: Rect,
    label: &str,
    primary: bool,
    id: egui::Id,
) -> egui::Response {
    let response = ui.interact(rect, id, egui::Sense::click());
    let fill = if primary { INK } else { Color32::WHITE };
    let text_color = if primary { PAPER } else { INK };
    ui.painter().rect(
        rect,
        CornerRadius::from(6.0),
        fill,
        Stroke::new(1.5, INK),
        StrokeKind::Inside,
    );
    ui.painter().text(
        rect.center(),
        Align2::CENTER_CENTER,
        label,
        font(12.0),
        text_color,
    );
    if response.hovered() {
        ui.painter().rect_stroke(
            rect.expand(2.0),
            CornerRadius::from(6.0),
            Stroke::new(1.0, Color32::from_rgba_unmultiplied(27, 27, 27, 80)),
            StrokeKind::Outside,
        );
    }
    response.on_hover_cursor(egui::CursorIcon::PointingHand)
}
