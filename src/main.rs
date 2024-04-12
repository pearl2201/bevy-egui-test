// disable console on windows for release builds
#![allow(clippy::type_complexity)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::window::PrimaryWindow;
use bevy::winit::WinitWindows;
use bevy::DefaultPlugins;
use bevy::{asset::AssetMetaCheck, window::PresentMode};
use bevy::{prelude::*, ui};
use bevy_egui::{egui, EguiContext, EguiContexts, EguiPlugin, EguiSettings};
use std::io::Cursor;
use wasm_bindgen::prelude::*;
use winit::window::Icon;
mod allocate;
use allocate::Grid;
fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .insert_resource(AssetMetaCheck::Never)
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy game".to_string(), // ToDo
                // Bind to canvas included in `index.html`
                canvas: Some("#bevy".to_owned()),
                // Tells wasm not to override default event handling, like F5 and Ctrl+R
                prevent_default_event_handling: false,
                present_mode: PresentMode::AutoNoVsync, // Reduces input lag.
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin)
        .add_systems(Startup, (set_window_icon, configure_visuals))
        .add_systems(Update, ui_example)
        .run();
}

// Sets the icon on windows and X11
fn set_window_icon(
    windows: NonSend<WinitWindows>,
    primary_window: Query<Entity, With<PrimaryWindow>>,
) {
    let primary_entity = primary_window.single();
    let Some(primary) = windows.get_window(primary_entity) else {
        return;
    };
    let icon_buf = Cursor::new(include_bytes!(
        "../build/macos/AppIcon.iconset/icon_256x256.png"
    ));
    if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        let icon = Icon::from_rgba(rgba, width, height).unwrap();
        primary.set_window_icon(Some(icon));
    };
}

fn configure_visuals(mut egui_ctx: EguiContexts) {
    egui_ctx.ctx_mut().set_visuals(egui::Visuals {
        window_rounding: 0.0.into(),
        ..Default::default()
    });
}

fn ui_example(mut egui_ctx: EguiContexts) {
    egui::TopBottomPanel::top("top_panel").show(egui_ctx.ctx_mut(), |ui| {
        // The top panel is often a good place for a menu bar:
        egui::menu::bar(ui, |ui| {
            egui::menu::menu_button(ui, "File", |ui| {
                if ui.button("Quit").clicked() {
                    std::process::exit(0);
                }
            });
        });
    });

    egui::CentralPanel::default().show(egui_ctx.ctx_mut(), |ui| {
        let griding = Grid::new(ui, 10, 3);

        // Place a widget on [4,1] cell (index starts from 1)
        griding.place(ui, 4, 1, |ui| {
            ui.centered_and_justified(|ui| {
                ui.label("row4, col1");
            });
        });

        // Place a widget on [3,2] cell
        griding.place(ui, 3, 2, |ui| {
            ui.centered_and_justified(|ui| {
                ui.label("row3, col2");
            });
        });

        // Place a widget on [5,3] cell (index starts from 1)
        griding.place(ui, 5, 3, |ui| {
            ui.centered_and_justified(|ui| {
                ui.label("row5, col3");
            });
        });
        let mut my_f32: f32 = 0.0;
        let mut my_string = "abc";
        let mut my_boolean: bool = false;
        let mut my_enum: Enum = Enum::First;
        ui.label("This is a label");
        ui.hyperlink("https://github.com/emilk/egui");
        ui.text_edit_singleline(&mut my_string);
        if ui.button("Click me").clicked() {}
        ui.add(egui::Slider::new(&mut my_f32, 0.0..=100.0));
        ui.add(egui::DragValue::new(&mut my_f32));

        ui.checkbox(&mut my_boolean, "Checkbox");

        #[derive(PartialEq)]
        enum Enum {
            First,
            Second,
            Third,
        }
        ui.horizontal(|ui| {
            ui.radio_value(&mut my_enum, Enum::First, "First");
            ui.radio_value(&mut my_enum, Enum::Second, "Second");
            ui.radio_value(&mut my_enum, Enum::Third, "Third");
        });

        ui.separator();

        //ui.image(my_image, [640.0, 480.0]);

        ui.collapsing("Click to see what is hidden!", |ui| {
            ui.label("Not much, as it turns out");
        });
    });
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // app.init_state::<GameState>().add_plugins((
        //     LoadingPlugin,
        //     MenuPlugin,
        //     ActionsPlugin,
        //     InternalAudioPlugin,
        //     PlayerPlugin,
        // ));

        // #[cfg(debug_assertions)]
        // {
        //     app.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()));
        // }
    }
}
