use gpui::*;

mod main;
mod settings;

pub use main::MainView;
pub use settings::SettingsView;

pub struct Interface;

impl Interface {
    pub fn new(cx: &mut Context<Self>) -> Self {
        cx.observe_global::<Navigation>(|_, cx| cx.notify())
            .detach();

        Self
    }
}

impl Render for Interface {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .child(cx.global::<Navigation>().current.clone())
    }
}

pub struct Navigation {
    pub current: AnyView,
}

impl Global for Navigation {}

impl Navigation {
    pub fn init(cx: &mut App) {
        let current = cx.new(|_| MainView).into();
        cx.set_global(Navigation { current });
    }

    pub fn navigate<T: Render>(cx: &mut App, view: T) {
        let view = cx.new(|_| view).into();
        cx.update_global::<Navigation, _>(|nav, _| nav.current = view);
    }
}
