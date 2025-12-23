//! 게임 모듈
//!
//! hydrate feature가 활성화된 경우에만 컴파일



use leptos::prelude::*;
use leptos::ev;

use crate::shared::domain::*;

mod canvas;
mod systems;

pub use canvas::*;
pub use systems::*;

/// Fetch monster data from API
async fn fetch_monsters() -> Vec<MonsterDataDto> {
    match gloo_net::http::Request::get("/api/monsters")
        .send()
        .await
    {
        Ok(response) => {
            if response.ok() {
                response.json().await.unwrap_or_default()
            } else {
                log::warn!("Failed to fetch monsters: {}", response.status());
                Vec::new()
            }
        }
        Err(e) => {
            log::warn!("Network error fetching monsters: {:?}", e);
            Vec::new()
        }
    }
}

#[component]
pub fn GameView() -> impl IntoView {
    // 게임 상태
    let (player, set_player) = signal(Player::new(
        "Hero".to_string(),
        PlayerClass::Warrior,
    ));
    let (monsters, set_monsters) = signal(Vec::<Monster>::new());
    let (show_character, set_show_character) = signal(false);
    let (show_inventory, set_show_inventory) = signal(false);
    let (show_skills, set_show_skills) = signal(false);
    
    // 키보드 입력
    let (keys_pressed, set_keys_pressed) = signal(std::collections::HashSet::<String>::new());
    
    // 게임 초기화 - 한 번만 실행
    let initialized = std::rc::Rc::new(std::cell::Cell::new(false));
    let initialized_clone = initialized.clone();
    
    Effect::new(move |_| {
        if initialized_clone.get() {
            return;
        }
        initialized_clone.set(true);
        
        // Fetch monsters from API
        leptos::task::spawn_local(async move {
            log::info!("🎮 Fetching monsters from API...");
            let monster_dtos = fetch_monsters().await;
            
            if monster_dtos.is_empty() {
                log::warn!("No monsters received from API, using fallback");
                // Fallback: Create a simple slime if API fails
                let fallback_monster = Monster::new(
                    &MonsterData {
                        id: 1,
                        name: "슬라임".to_string(),
                        level: 1,
                        max_hp: 30,
                        attack_min: 2,
                        attack_max: 4,
                        defense: 1,
                        exp_reward: 5,
                        gold_min: 2,
                        gold_max: 5,
                        ai_type: MonsterAIType::Passive,
                        detection_range: 150.0,
                        attack_range: 40.0,
                        move_speed: 80.0,
                        sprite_path: "/assets/monsters/slime/spritesheet.png".to_string(),
                        sprite_type: "slime".to_string(),
                        sprite_size: SpriteSize::Small,
                    },
                    Position::new(300.0, 200.0),
                );
                set_monsters.set(vec![fallback_monster]);
            } else {
                log::info!("✅ Loaded {} monsters from database", monster_dtos.len());
                
                // Convert DTOs to MonsterData and spawn monsters at random positions
                let mut rng_seed = 42u32;
                let initial_monsters: Vec<Monster> = monster_dtos
                    .into_iter()
                    .take(5) // Limit to 5 monsters for initial spawn
                    .map(|dto| {
                        // Simple pseudo-random position generation
                        rng_seed = rng_seed.wrapping_mul(1103515245).wrapping_add(12345);
                        let x = 200.0 + (rng_seed % 400) as f64;
                        rng_seed = rng_seed.wrapping_mul(1103515245).wrapping_add(12345);
                        let y = 150.0 + (rng_seed % 300) as f64;
                        
                        let monster_data = dto.into_monster_data();
                        Monster::new(&monster_data, Position::new(x, y))
                    })
                    .collect();
                
                set_monsters.set(initial_monsters);
            }
        });
        
        window_event_listener(ev::keydown, move |e: web_sys::KeyboardEvent| {
            let key = e.key();
            set_keys_pressed.update(|keys| {
                keys.insert(key);
            });
        });
        
        window_event_listener(ev::keyup, move |e: web_sys::KeyboardEvent| {
            let key = e.key();
            set_keys_pressed.update(|keys| {
                keys.remove(&key);
            });
        });
    });

    view! {
        <div class="game-container">
            <crate::client::components::HUD
                player=player
                on_character_click=move |_| set_show_character.set(!show_character.get())
                on_inventory_click=move |_| set_show_inventory.set(!show_inventory.get())
                on_skills_click=move |_| set_show_skills.set(!show_skills.get())
            />
            
            <GameCanvas
                player=player
                set_player=set_player
                monsters=monsters
                keys_pressed=keys_pressed
            />
            
            {move || {
                if show_character.get() {
                    Some(view! {
                        <crate::client::components::CharacterWindow
                            player=player
                            set_player=set_player
                            on_close=move |_| set_show_character.set(false)
                        />
                    })
                } else {
                    None
                }
            }}
            
            {move || {
                if show_inventory.get() {
                    Some(view! {
                        <crate::client::components::InventoryWindow on_close=move |_| set_show_inventory.set(false) />
                    })
                } else {
                    None
                }
            }}
            
            {move || {
                if show_skills.get() {
                    Some(view! {
                        <crate::client::components::SkillWindow player=player on_close=move |_| set_show_skills.set(false) />
                    })
                } else {
                    None
                }
            }}
            
            <div class="game-info">
                <p>"WASD - 이동 | C - 캐릭터 | I - 인벤토리 | K - 스킬"</p>
                <p>"현재 맵: " {move || player.get().current_map.clone()}</p>
            </div>
        </div>
    }
}
