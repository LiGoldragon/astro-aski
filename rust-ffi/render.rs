// Thin FFI adapter — bevy rendering primitives.
// Buffers draw commands from aski, replays as bevy entities on app startup.
// All public functions accept f64 (matching aski's F64); cast to f32 for bevy.

use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_prototype_lyon::prelude::*;
use std::sync::Mutex;

static DRAW_CMDS: Mutex<Vec<DrawCmd>> = Mutex::new(Vec::new());

enum DrawCmd {
    Camera,
    RingSector {
        outer_r: f32, inner_r: f32,
        start_angle: f32, sweep_angle: f32,
        r: f32, g: f32, b: f32, a: f32, z: f32,
    },
    Line {
        x1: f32, y1: f32, x2: f32, y2: f32,
        r: f32, g: f32, b: f32, a: f32, width: f32, z: f32,
    },
    CircleOutline {
        radius: f32,
        r: f32, g: f32, b: f32, a: f32, width: f32, z: f32,
    },
    Label {
        text: String, size: f32,
        r: f32, g: f32, b: f32, a: f32,
        x: f32, y: f32, z: f32,
    },
}

fn push(cmd: DrawCmd) -> i32 {
    DRAW_CMDS.lock().unwrap().push(cmd);
    0
}

pub fn spawn_camera() -> i32 { push(DrawCmd::Camera) }

pub fn fill_ring_sector(
    outer_r: f64, inner_r: f64,
    start_angle: f64, sweep_angle: f64,
    r: f64, g: f64, b: f64, a: f64, z: f64,
) -> i32 {
    push(DrawCmd::RingSector {
        outer_r: outer_r as f32, inner_r: inner_r as f32,
        start_angle: start_angle as f32, sweep_angle: sweep_angle as f32,
        r: r as f32, g: g as f32, b: b as f32, a: a as f32, z: z as f32,
    })
}

pub fn stroke_line(
    x1: f64, y1: f64, x2: f64, y2: f64,
    r: f64, g: f64, b: f64, a: f64, width: f64, z: f64,
) -> i32 {
    push(DrawCmd::Line {
        x1: x1 as f32, y1: y1 as f32, x2: x2 as f32, y2: y2 as f32,
        r: r as f32, g: g as f32, b: b as f32, a: a as f32,
        width: width as f32, z: z as f32,
    })
}

pub fn stroke_circle(
    radius: f64, r: f64, g: f64, b: f64, a: f64, width: f64, z: f64,
) -> i32 {
    push(DrawCmd::CircleOutline {
        radius: radius as f32,
        r: r as f32, g: g as f32, b: b as f32, a: a as f32,
        width: width as f32, z: z as f32,
    })
}

pub fn label(
    text: String, size: f64,
    r: f64, g: f64, b: f64, a: f64,
    x: f64, y: f64, z: f64,
) -> i32 {
    push(DrawCmd::Label {
        text, size: size as f32,
        r: r as f32, g: g as f32, b: b as f32, a: a as f32,
        x: x as f32, y: y as f32, z: z as f32,
    })
}

pub fn app_run(
    title: String, width: i32, height: i32,
    bg_r: f64, bg_g: f64, bg_b: f64,
) -> i32 {
    let cmds = std::mem::take(&mut *DRAW_CMDS.lock().unwrap());

    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title,
                    resolution: WindowResolution::new(width as u32, height as u32),
                    ..default()
                }),
                ..default()
            }),
            ShapePlugin,
        ))
        .insert_resource(ClearColor(Color::srgb(bg_r as f32, bg_g as f32, bg_b as f32)))
        .insert_resource(DeferredDraw(cmds))
        .add_systems(Startup, replay_draw)
        .run();

    0
}

pub fn f64_cos(angle: f64) -> f64 { angle.cos() }
pub fn f64_sin(angle: f64) -> f64 { angle.sin() }

#[derive(Resource)]
struct DeferredDraw(Vec<DrawCmd>);

fn replay_draw(mut commands: Commands, deferred: Res<DeferredDraw>, asset_server: Res<AssetServer>) {
    let font: Handle<Font> = asset_server.load("fonts/DejaVuSans.ttf");
    for cmd in &deferred.0 {
        match cmd {
            DrawCmd::Camera => {
                commands.spawn(Camera2d);
            }
            DrawCmd::RingSector { outer_r, inner_r, start_angle, sweep_angle, r, g, b, a, z } => {
                let steps = 30;
                let mut path = ShapePath::new()
                    .move_to(Vec2::new(
                        outer_r * start_angle.cos(),
                        outer_r * start_angle.sin(),
                    ));
                for s in 1..=steps {
                    let t = s as f32 / steps as f32;
                    let angle = start_angle + t * sweep_angle;
                    path = path.line_to(Vec2::new(outer_r * angle.cos(), outer_r * angle.sin()));
                }
                for s in (0..=steps).rev() {
                    let t = s as f32 / steps as f32;
                    let angle = start_angle + t * sweep_angle;
                    path = path.line_to(Vec2::new(inner_r * angle.cos(), inner_r * angle.sin()));
                }
                path = path.close();
                let shape = ShapeBuilder::with(&path)
                    .fill(Fill::color(Color::srgba(*r, *g, *b, *a)))
                    .build();
                commands.spawn((shape, Transform::from_xyz(0.0, 0.0, *z)));
            }
            DrawCmd::Line { x1, y1, x2, y2, r, g, b, a, width, z } => {
                let line = shapes::Line(Vec2::new(*x1, *y1), Vec2::new(*x2, *y2));
                let shape = ShapeBuilder::with(&line)
                    .stroke(Stroke::new(Color::srgba(*r, *g, *b, *a), *width))
                    .build();
                commands.spawn((shape, Transform::from_xyz(0.0, 0.0, *z)));
            }
            DrawCmd::CircleOutline { radius, r, g, b, a, width, z } => {
                let circle = shapes::Circle { radius: *radius, center: Vec2::ZERO };
                let shape = ShapeBuilder::with(&circle)
                    .stroke(Stroke::new(Color::srgba(*r, *g, *b, *a), *width))
                    .build();
                commands.spawn((shape, Transform::from_xyz(0.0, 0.0, *z)));
            }
            DrawCmd::Label { text, size, r, g, b, a, x, y, z } => {
                commands.spawn((
                    Text2d::new(text.clone()),
                    TextFont { font: font.clone(), font_size: *size, ..default() },
                    TextColor(Color::srgba(*r, *g, *b, *a)),
                    Transform::from_xyz(*x, *y, *z),
                ));
            }
        }
    }
}
