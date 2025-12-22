use leptos::prelude::*;
use crate::models::Player;
use crate::domain::skill::components::SkillWindow as DomainSkillWindow;

#[component]
pub fn SkillWindow(
    player: ReadSignal<Player>,
    on_close: impl Fn(web_sys::MouseEvent) + 'static + Clone,
) -> impl IntoView {
    view! {
        <DomainSkillWindow 
            player_class=Signal::derive(move || player.get().class.name().to_string())
            player_level=Signal::derive(move || player.get().level)
            on_close=on_close
        />
    }
}
