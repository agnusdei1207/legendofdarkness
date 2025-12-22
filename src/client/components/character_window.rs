use leptos::prelude::*;
use crate::models::*;

#[component]
pub fn CharacterWindow(
    player: ReadSignal<Player>,
    set_player: WriteSignal<Player>,
    on_close: impl Fn(web_sys::MouseEvent) + 'static,
) -> impl IntoView {
    let add_str = move |_| {
        set_player.update(|p: &mut Player| p.add_stat(StatType::Strength, 1));
    };
    let add_dex = move |_| {
        set_player.update(|p: &mut Player| p.add_stat(StatType::Dexterity, 1));
    };
    let add_int = move |_| {
        set_player.update(|p: &mut Player| p.add_stat(StatType::Intelligence, 1));
    };
    let add_vit = move |_| {
        set_player.update(|p: &mut Player| p.add_stat(StatType::Vitality, 1));
    };
    let add_luk = move |_| {
        set_player.update(|p: &mut Player| p.add_stat(StatType::Luck, 1));
    };
    
    view! {
        <div class="modal-overlay">
            <div class="window character-window">
                <div class="window-header">
                    <h2>"캐릭터 정보"</h2>
                    <button class="close-btn" on:click=on_close>"✕"</button>
                </div>
                
                <div class="window-content">
                    <div class="character-stats">
                        <h3>"기본 스탯"</h3>
                        <div class="stat-list">
                            <div class="stat-item">
                                <span>"힘 (STR)"</span>
                                <span>{move || player.get().stats.strength}</span>
                                <button on:click=add_str>"+"</button>
                            </div>
                            <div class="stat-item">
                                <span>"민첩 (DEX)"</span>
                                <span>{move || player.get().stats.dexterity}</span>
                                <button on:click=add_dex>"+"</button>
                            </div>
                            <div class="stat-item">
                                <span>"지능 (INT)"</span>
                                <span>{move || player.get().stats.intelligence}</span>
                                <button on:click=add_int>"+"</button>
                            </div>
                            <div class="stat-item">
                                <span>"체력 (VIT)"</span>
                                <span>{move || player.get().stats.vitality}</span>
                                <button on:click=add_vit>"+"</button>
                            </div>
                            <div class="stat-item">
                                <span>"행운 (LUK)"</span>
                                <span>{move || player.get().stats.luck}</span>
                                <button on:click=add_luk>"+"</button>
                            </div>
                        </div>
                        <div class="stat-points">
                            "스탯 포인트: " {move || player.get().stat_points}
                        </div>
                    </div>
                    
                    <div class="combat-stats">
                        <h3>"전투 스탯"</h3>
                        <div class="stat-list">
                            <div class="stat-item">
                                <span>"공격력"</span>
                                <span>{move || {
                                    let p = player.get();
                                    format!("{}-{}", p.combat_stats.attack_min, p.combat_stats.attack_max)
                                }}</span>
                            </div>
                            <div class="stat-item">
                                <span>"방어력"</span>
                                <span>{move || player.get().combat_stats.defense}</span>
                            </div>
                            <div class="stat-item">
                                <span>"명중률"</span>
                                <span>{move || format!("{}%", player.get().combat_stats.hit_rate)}</span>
                            </div>
                            <div class="stat-item">
                                <span>"회피율"</span>
                                <span>{move || format!("{}%", player.get().combat_stats.avoid_rate)}</span>
                            </div>
                            <div class="stat-item">
                                <span>"크리티컬"</span>
                                <span>{move || format!("{}%", player.get().combat_stats.critical_rate)}</span>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
