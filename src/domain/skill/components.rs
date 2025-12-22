use leptos::prelude::*;
use crate::domain::skill::models::Skill;
use crate::domain::skill::server::get_skills;

#[component]
pub fn SkillWindow(
    player_class: ReadSignal<String>, // Passed as signal or string? Dynamic means signal.
    player_level: ReadSignal<i32>,
    on_close: impl Fn(web_sys::MouseEvent) + 'static + Clone,
) -> impl IntoView {
    // Resource to fetch skills
    let skills_resource = Resource::new(
        move || (player_class.get(), player_level.get()),
        move |(class, level)| async move {
            get_skills(class, level).await
        }
    );

    view! {
        <div class="modal-overlay">
            <div class="window skill-window">
                <div class="window-header">
                    <h2>"스킬"</h2>
                    <button class="close-btn" on:click=on_close.clone()>"✕"</button>
                </div>
                
                <div class="window-content">
                    <div class="skill-info">
                        <p>"직업: " {move || player_class.get()}</p>
                        <p>"레벨 " {move || player_level.get()} " 스킬 목록"</p>
                    </div>
                    
                    <div class="skill-grid-container">
                        <Suspense fallback=move || view! { <div class="loading">"스킬 불러오는 중..."</div> }>
                            {move || {
                                skills_resource.get().map(|result| match result {
                                    Ok(skills) => {
                                        if skills.is_empty() {
                                            view! { <div class="empty-message">"배울 수 있는 스킬이 없습니다."</div> }.into_view()
                                        } else {
                                            view! {
                                                <div class="skill-grid">
                                                    {skills.into_iter().map(|skill| {
                                                        view! {
                                                            <div class="skill-slot" title={skill.description.unwrap_or_default()}>
                                                                <div class="skill-icon-wrapper">
                                                                   {match skill.icon_path {
                                                                       Some(path) => view! { <img src=path alt=skill.name class="skill-icon-img" /> }.into_view(),
                                                                       None => view! { <div class="skill-icon-placeholder">"⚔️"</div> }.into_view()
                                                                   }}
                                                                </div>
                                                                <div class="skill-details">
                                                                    <div class="skill-name">{skill.name}</div>
                                                                    <div class="skill-meta">
                                                                        <span class="skill-mp">"MP " {skill.mp_cost}</span>
                                                                        <span class="skill-dmg">"Dmg " {skill.damage_min}"~"{skill.damage_max}</span>
                                                                    </div>
                                                                </div>
                                                            </div>
                                                        }
                                                    }).collect_view()}
                                                </div>
                                            }.into_view()
                                        }
                                    },
                                    Err(e) => view! { <div class="error-message">"오류 발생: " {e.to_string()}</div> }.into_view()
                                })
                            }}
                        </Suspense>
                    </div>
                </div>
            </div>
        </div>
    }
}
