use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::winit::WinitWindows;
use std::io::Cursor;
use winit::window::Icon;

// Sets the icon on windows and X11
pub(super) fn set_window_icon(
    windows: NonSend<WinitWindows>,
    primary_window: Query<Entity, With<PrimaryWindow>>,
) {
    let primary_entity = primary_window.single();
    let primary = windows.get_window(primary_entity).unwrap();
    let icon_buf = Cursor::new(include_bytes!(
        "../../build/macos/AppIcon.iconset/icon_256x256.png"
    ));

    if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        let icon = Icon::from_rgba(rgba, width, height).unwrap();
        primary.set_window_icon(Some(icon));
    };
}
