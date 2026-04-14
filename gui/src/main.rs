use gpui::*;
use gpui_component::*;
use gpui_platform::application;

mod assets;
mod views;

fn main() {
    let app = application().with_assets(assets::Assets);

    app.run(move |cx| {
        gpui_component::init(cx);

        cx.spawn(async move |cx| {
            cx.update(views::Navigation::init);
            cx.open_window(
                WindowOptions {
                    titlebar: Some(TitlebarOptions {
                        title: Some("Nyvo".into()),
                        ..Default::default()
                    }),
                    app_id: Some("com.surrealhorizon.nyvo".into()),
                    ..Default::default()
                },
                |window, cx| {
                    cx.text_system()
                        .add_fonts(vec![
                            include_bytes!("../assets/fonts/Inter_24pt-Regular.ttf").into(),
                            include_bytes!("../assets/fonts/Inter_24pt-Bold.ttf").into(),
                        ])
                        .expect("failed to load fonts");
                    let view = cx.new(views::Interface::new);
                    cx.new(|cx| Root::new(view, window, cx).font_family("Inter 24pt"))
                },
            )
            .expect("Failed to open window");
        })
        .detach();
    });
}
