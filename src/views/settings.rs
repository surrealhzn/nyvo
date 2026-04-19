use gpui::*;

pub struct SettingsView;
impl Render for SettingsView {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div().child("Settings view")
    }
}
