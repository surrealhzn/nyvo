use gpui::*;
use gpui_component::{button::*, *};

use crate::views::{Navigation, SettingsView};

pub struct MainView;
impl Render for MainView {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .v_flex()
            .gap_16()
            .size_full()
            .items_center()
            .justify_center()
            .child(
                div()
                    .flex()
                    .gap_4()
                    .items_center()
                    .justify_center()
                    .text_size(px(40.))
                    .child(
                        img("icon.svg")
                            .w(relative(1.)) // Full width
                            .max_w(px(64.))
                            .h(px(64.))
                            .rounded_xl(),
                    )
                    .child(
                        div()
                            .flex()
                            .child(div().font_weight(FontWeight::BOLD).child("Surreal"))
                            .child(" Nyvo"),
                    ),
            )
            .child(
                Button::new("ok")
                    .primary()
                    .label("Extract")
                    .on_click(|_, _, _| println!("Clicked!"))
                    .child(Icon::new(Icon::empty()).path("icons/folder-down.svg")),
            )
            .child(
                div()
                    .child(
                        Button::new("version")
                            .ghost()
                            .small()
                            .label(format!("v{}", env!("CARGO_PKG_VERSION")))
                            .on_click(|_, _, _| {
                                open::that(format!(
                                    "https://github.com/surrealhzn/nyvo/releases/tag/v{}",
                                    env!("CARGO_PKG_VERSION")
                                ))
                                .expect("failed to open URL")
                            }),
                    )
                    .child(
                        Button::new("settings")
                            .ghost()
                            .small()
                            .label("Settings")
                            .on_click(|_, _, cx| {
                                Navigation::navigate(cx, SettingsView);
                            }),
                    ),
            )
    }
}
