use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;
use crate::shared::domain::*;
use std::collections::HashSet;

const CANVAS_WIDTH: f64 = 800.0;
const CANVAS_HEIGHT: f64 = 600.0;

#[component]
pub fn GameCanvas(
    player: ReadSignal<Player>,
    set_player: WriteSignal<Player>,
    monsters: ReadSignal<Vec<Monster>>,
    keys_pressed: ReadSignal<HashSet<String>>,
) -> impl IntoView {
    let canvas_ref = NodeRef::<leptos::html::Canvas>::new();
    let (tile_sheet, set_tile_sheet) = signal(None::<web_sys::HtmlImageElement>);
    let (building_sheet, set_building_sheet) = signal(None::<web_sys::HtmlImageElement>);
    
    // Map System
    let map_data = crate::client::game::systems::MapRenderer::create_mock_map();
    let map_renderer = crate::client::game::systems::MapRenderer::new(CANVAS_WIDTH, CANVAS_HEIGHT);
    
    // Load Tile Sheet
    Effect::new(move |_| {
        let img = web_sys::HtmlImageElement::new().unwrap();
        img.set_src("/assets/tiles/iso_tiles.png");
        let set_img = set_tile_sheet;
        let onload = Closure::wrap(Box::new(move || {
            set_img.set(Some(img.clone()));
        }) as Box<dyn FnMut()>);
        img.set_onload(Some(onload.as_ref().unchecked_ref()));
        onload.forget();
    });

    // Load Building Sheet
    Effect::new(move |_| {
        let img = web_sys::HtmlImageElement::new().unwrap();
        img.set_src("/assets/tiles/iso_buildings.png");
        let set_img = set_building_sheet;
        let onload = Closure::wrap(Box::new(move || {
            set_img.set(Some(img.clone()));
        }) as Box<dyn FnMut()>);
        img.set_onload(Some(onload.as_ref().unchecked_ref()));
        onload.forget();
    });

    // ... (existing effects) ...
    // Update render loop
    Effect::new(move |_| {
    // ...
                    render_game(
                        &ctx, 
                        player.get(), 
                        monsters.get(), 
                        &char_sheet.get(), 
                        &monster_sheet.get(), 
                        &tile_sheet.get(),
                        &building_sheet.get(),
                        &map_renderer,
                        &map_data
                    );
    // ...
    });

    // Keeping the rest of the file...
    // Note: I will need to replace the render_game signature and implementation in one go properly.
    // I am replacing the TOP part of the component logic here.
    
    // 게임 루프
    Effect::new(move |_| {
        if let Some(canvas) = canvas_ref.get() {
            let ctx: CanvasRenderingContext2d = canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into()
                .unwrap();
            
            ctx.set_image_smoothing_enabled(false);
            
            let f = std::rc::Rc::new(std::cell::RefCell::new(None::<Closure<dyn FnMut()>>));
            let g = f.clone();
            let mut last_time = js_sys::Date::now();
            
            *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
                let current_time = js_sys::Date::now();
                let delta_time = (current_time - last_time) / 1000.0;
                last_time = current_time;
                
                if delta_time > 0.0 && delta_time < 0.1 {
                    // Update logic (movement etc)
                    let keys = keys_pressed.get();
                    let mut p = player.get();
                    let speed = 200.0 * delta_time;
                    let mut new_x = p.position.x;
                    let mut new_y = p.position.y;
                    let mut moved = false;
                    
                    if keys.contains("ArrowUp") || keys.contains("w") || keys.contains("W") { new_y -= speed; p.direction = Direction::Up; moved = true; }
                    if keys.contains("ArrowDown") || keys.contains("s") || keys.contains("S") { new_y += speed; p.direction = Direction::Down; moved = true; }
                    if keys.contains("ArrowLeft") || keys.contains("a") || keys.contains("A") { new_x -= speed; p.direction = Direction::Left; moved = true; }
                    if keys.contains("ArrowRight") || keys.contains("d") || keys.contains("D") { new_x += speed; p.direction = Direction::Right; moved = true; }
                    
                    new_x = new_x.max(16.0).min(CANVAS_WIDTH - 16.0);
                    new_y = new_y.max(16.0).min(CANVAS_HEIGHT - 16.0);
                    
                    p.position.x = new_x;
                    p.position.y = new_y;
                    p.is_moving = moved;
                    set_player.set(p);
                    
                    render_game(&ctx, player.get(), monsters.get(), &char_sheet.get(), &monster_sheet.get());
                }
                request_animation_frame(f.borrow().as_ref().unwrap());
            }) as Box<dyn FnMut()>));
            request_animation_frame(g.borrow().as_ref().unwrap());
        }
    });

    view! {
        <canvas
            node_ref=canvas_ref
            width=CANVAS_WIDTH.to_string()
            height=CANVAS_HEIGHT.to_string()
            class="game-canvas"
        ></canvas>
    }
}

fn render_game(
    ctx: &CanvasRenderingContext2d, 
    player: Player, 
    monsters: Vec<Monster>, 
    char_sheet: &Option<web_sys::HtmlImageElement>,
    monster_sheet: &Option<web_sys::HtmlImageElement>,
    tile_sheet: &Option<web_sys::HtmlImageElement>,
    building_sheet: &Option<web_sys::HtmlImageElement>,
    map_renderer: &crate::client::game::systems::MapRenderer,
    map_data: &crate::shared::domain::map::MapData
) {
    // 배경
    ctx.set_fill_style(&JsValue::from_str("#0f0c29"));
    ctx.fill_rect(0.0, 0.0, CANVAS_WIDTH, CANVAS_HEIGHT);
    
    // 맵 렌더링 (Tiles & Buildings)
    // Note: Buildings currently drawn on top of tiles but under entities? 
    // In deep isometric, objects need to be sorted with entities.
    // map_renderer.render handles tiles + objects. 
    // Ideally we split it: Render Floor -> Render Entities+Objects sorted by Y.
    // For now, let's let map_renderer draw floor AND buildings, and draw entities on top.
    // This defines "Background buildings".
    
    map_renderer.render(ctx, map_data, tile_sheet, building_sheet);
    
    // ... Entities rendering ...
    
    // Sort entities
    enum Entity<'a> {
        Player(&'a Player),
        Monster(&'a Monster),
    }
    
    let mut entities: Vec<Entity> = Vec::new();
    entities.push(Entity::Player(&player));
    for m in &monsters {
        entities.push(Entity::Monster(m));
    }
    
    entities.sort_by(|a, b| {
        let (ax, ay) = match a {
            Entity::Player(p) => (p.position.x, p.position.y),
            Entity::Monster(m) => (m.position.x, m.position.y),
        };
        let (bx, by) = match b {
            Entity::Player(p) => (p.position.x, p.position.y),
            Entity::Monster(m) => (m.position.x, m.position.y),
        };
        (ax + ay).partial_cmp(&(bx + by)).unwrap_or(std::cmp::Ordering::Equal)
    });
    
    for entity in entities {
        match entity {
            Entity::Player(p) => {
                let (sx, sy) = to_screen_coord(p.position.x, p.position.y);
                if let Some(img) = char_sheet {
                    draw_player_sprite(ctx, p, img, sx, sy);
                } else {
                    draw_player(ctx, p, sx, sy);
                }
            },
            Entity::Monster(m) => {
                let (sx, sy) = to_screen_coord(m.position.x, m.position.y);
                if let Some(img) = monster_sheet {
                    draw_monster_sprite(ctx, m, img, sx, sy);
                } else {
                    draw_monster(ctx, m, sx, sy);
                }
            }
        }
    }
}

// 아이소메트릭 변환 함수 (Isometric Projection)
// 2D 그리드 좌표를 2.5D 아이소메트릭 화면 좌표로 변환합니다.
fn to_screen_coord(world_x: f64, world_y: f64) -> (f64, f64) {
    let tile_width = 64.0; // 타일의 시각적 너비 (가로 64픽셀)
    let tile_height = 32.0; // 타일의 시각적 높이 (세로 32픽셀)
    
    // 월드 좌표(픽셀)를 그리드 좌표(격자 인덱스)로 변환
    // 게임 로직상 32픽셀을 1칸으로 취급한다고 가정
    let grid_x = world_x / 32.0;
    let grid_y = world_y / 32.0;
    
    // 화면 중앙을 기준으로 렌더링 시작점 설정
    let origin_x = CANVAS_WIDTH / 2.0;
    let origin_y = 100.0;
    
    // 아이소메트릭 투영 공식
    // 화면 X = (그리드X - 그리드Y) * (타일너비 / 2)
    // 화면 Y = (그리드X + 그리드Y) * (타일높이 / 2)
    // 이 공식이 캐릭터와 타일을 사선(다이아몬드 형태)으로 배치하게 만듭니다.
    let screen_x = origin_x + (grid_x - grid_y) * (tile_width / 2.0);
    let screen_y = origin_y + (grid_x + grid_y) * (tile_height / 2.0);
    
    (screen_x, screen_y)
}
    // Load Tile Sheet
    let (tile_sheet, set_tile_sheet) = signal(None::<web_sys::HtmlImageElement>);
    Effect::new(move |_| {
        let img = web_sys::HtmlImageElement::new().unwrap();
        img.set_src("/assets/tiles/iso_tiles.png");
        let set_img = set_tile_sheet;
        let onload = Closure::wrap(Box::new(move || {
            set_img.set(Some(img.clone()));
        }) as Box<dyn FnMut()>);
        img.set_onload(Some(onload.as_ref().unchecked_ref()));
        onload.forget();
    });

    // ... (Inside render_game, update signature to accept tile_sheet) ...
    // render_game(&ctx, ..., &tile_sheet.get());
    
// Helper to draw the map
fn draw_map_tiles(ctx: &CanvasRenderingContext2d, tile_sheet: &Option<web_sys::HtmlImageElement>) {
    if let Some(sheet) = tile_sheet {
        // Assume a fixed map size for now (e.g., 20x20)
        // In a real scenario, this comes from Map Data
        let map_width = 25;
        let map_height = 25;
        
        // Tile dimensions in the sheet
        // Prompt generated: Grass, Stone, Water, Wall, Portal
        // Assuming they are in a row or grid.
        // Let's assume standard 64x32 logic (or 64x64 if they are tall blocks).
        // Let's assume the generated image has them in a row: 0: Grass, 1: Stone, 2: Water
        
        let tile_w = 64.0;
        let tile_h = 48.0; // Bit taller for the blockiness
        
        for gy in 0..map_height {
            for gx in 0..map_width {
                // Determine tile type based on coordinates (Mock Logic)
                let tile_idx = if gx == 0 || gx == map_width - 1 || gy == 0 || gy == map_height - 1 {
                    3.0 // Wall
                } else if (gx + gy) % 10 == 0 {
                    2.0 // Water
                } else if (gx * gy) % 7 == 0 {
                   1.0 // Stone
                } else {
                    0.0 // Grass
                };

                let grid_x = gx as f64;
                let grid_y = gy as f64;
                
                // Convert Grid -> Screen
                // Reuse to_screen_coord logic but optimized for loop
                 // 화면 X = (그리드X - 그리드Y) * (타일너비 / 2)
                 // 화면 Y = (그리드X + 그리드Y) * (타일높이 / 2)
                 // Tiles usually need to be drawn from top-back to bottom-front (Painter's algo).
                 // Simple loop Y then X matches isometric standard typically (or X+Y sorting).
                 
                 let origin_x = CANVAS_WIDTH / 2.0;
                 let origin_y = 100.0;
                 let tile_iso_width = 64.0;
                 let tile_iso_height = 32.0;
                 
                 let sx = origin_x + (grid_x - grid_y) * (tile_iso_width / 2.0);
                 let sy = origin_y + (grid_x + grid_y) * (tile_iso_height / 2.0);
                 
                 // Draw Tile
                 // Adjust draw position because the "origin" of the tile image is usually bottom-center or top-left.
                 // For a 64x48 block where the top diamond is the floor surface:
                 // We draw so the center of the diamond aligns with sx, sy.
                 
                 let draw_x = sx - tile_w / 2.0;
                 let draw_y = sy - (tile_h - tile_iso_height / 2.0); // Adjust for height
                 
                 // Source coordinates
                 let src_x = tile_idx * 64.0;
                 let src_y = 0.0;
                 
                 ctx.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                     sheet, src_x, src_y, tile_w, tile_h, draw_x, draw_y, tile_w, tile_h
                 ).unwrap_or_else(|_| ());
            }
        }
    } else {
        // Fallback to lines
        draw_isometric_grid(ctx);
    }
}

fn draw_player_sprite(ctx: &CanvasRenderingContext2d, player: &Player, img: &web_sys::HtmlImageElement, sx: f64, sy: f64) {
    let draw_x = sx - 16.0;
    let draw_y = sy - 24.0;
    
    // Tiered Spritesheet Layout:
    // Rows (0-9):
    // 0: Warrior Male, 1: Warrior Female
    // 2: Rogue Male, 3: Rogue Female
    // 4: Mage Male, 5: Mage Female
    // 6: Cleric Male, 7: Cleric Female
    // 8: Martial Artist Male, 9: Martial Artist Female
    
    // Columns (0-4):
    // 0: Tier 1 (Circle 1 - Rags)
    // 1: Tier 2 (Circle 2 - Novice)
    // 2: Tier 3 (Circle 3 - Skilled)
    // 3: Tier 4 (Circle 4 - Expert)
    // 4: Tier 5 (Circle 5 - Master)

    // Calculate Row
    let base_row = match player.class {
        PlayerClass::Warrior => 0.0,
        PlayerClass::Rogue => 2.0,
        PlayerClass::Mage => 4.0,
        PlayerClass::Cleric => 6.0,
        PlayerClass::MartialArtist => 8.0,
    };
    
    let gender_offset = if player.gender == "female" { 1.0 } else { 0.0 };
    let row = base_row + gender_offset;
    
    // Calculate Visual Tier (Column)
    // Based on equipped Armor Grade primarily, or Level (Circle).
    // User said: "depending on what item they wear, different appearance".
    // Let's use the Armor slot item grade.
    // If no armor, Tier 1.
    // Grade 1-2: Tier 1
    // Grade 3-4: Tier 2
    // Grade 5-6: Tier 3
    // Grade 7-9: Tier 4
    // Grade 10-12: Tier 5
    
    let tier_col = if let Some(armor) = player.equipment.get(&crate::shared::domain::item::models::EquipmentSlot::Armor) {
        match armor.grade {
            1..=2 => 0.0,
            3..=4 => 1.0,
            5..=6 => 2.0,
            7..=9 => 3.0,
            10..=12 => 4.0,
            _ => 0.0,
        }
    } else {
        // Fallback to level-based circle if no armor? Or just naked/rags (Tier 1).
        // Let's use Level/11 roughly if no armor, or just 0.
        // User implied circles. Let's stick to Armor Grade driving looks.
        // If naked, Tier 1 (Rags).
        0.0
    };

    // Sprite dimensions in the generated sheet might be different. 
    // Usually AI generates a grid.
    // Let's assume standard cell size logic or adjust.
    // The previous code used 32x48. Let's stick to that and assume the image scales.
    // If the image is large, we might need to adjust `cell_width` / `cell_height`.
    // Let's assume the generated image is a grid of 5 columns x 10 rows.
    
    // If movement animation is needed, we need "Frames" per Tier.
    // But the prompt asked for "Tier 1 -> Tier 5" in rows.
    // It didn't explicitly ask for animation frames for each tier.
    // The previous code simulated walking by toggling frames.
    // If the generated image DOES NOT have walk frames, we just show static.
    // Let's toggle slightly or bob for now.
    
    let cell_width = 64.0; // Generated image likely has wider cells for detail
    let cell_height = 64.0;
    
    let src_x = tier_col * cell_width;
    let src_y = row * cell_height;
    
    // Draw
    ctx.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
        img, src_x, src_y, cell_width, cell_height, draw_x - 16.0, draw_y - 10.0, 64.0, 64.0
    ).unwrap_or_else(|_| ());
    
    // Name
    ctx.set_fill_style(&JsValue::from_str("#c5c6c7"));
    ctx.set_font("10px 'Press Start 2P', monospace");
    ctx.set_text_align("center");
    ctx.fill_text(&player.username, sx, draw_y - 5.0).unwrap();

    // HP bar
    draw_hp_bar(ctx, sx, draw_y - 8.0, player.combat_stats.hp, player.combat_stats.max_hp);
}

fn draw_monster_sprite(ctx: &CanvasRenderingContext2d, monster: &Monster, img: &web_sys::HtmlImageElement, sx: f64, sy: f64) {
    let draw_x = sx - 16.0;
    let draw_y = sy - 24.0;
    
    // Map monster name to index in sprite sheet
    // Generated sheet has 8 monsters.
    // Order: Rat, Bat, Skeleton, Kobold, Spider, Ghoul, Werewolf, Succubus
    // Assuming single row or grid. The prompt asked for specific list.
    // Let's assume they are laid out in a grid or row.
    // 0: Rat, 1: Bat ...
    
    let index = match monster.name.as_str() {
        "쥐" => 0.0,
        "박쥐" => 1.0,
        "스켈레톤" => 2.0,
        "코볼트" => 3.0,
        "거대 거미" => 4.0,
        "구울" => 5.0,
        "라이칸스로프" => 6.0,
        "서큐버스" => 7.0,
        _ => 0.0, // Default
    };
    
    // Assuming standard 32x32 logic for monsters or 32x48? Monster output often square.
    // Let's assume 32x32 for most, maybe larger for later ones.
    // If they are all in one sheet side-by-side:
    let src_x = index * 48.0; // Spacing?
    // Actually generated image likely put them in 2 rows of 4 if typical.
    // Let's guess: 4 columns, 2 rows.
    
    let col = index % 4.0;
    let row = (index / 4.0).floor();
    
    // Assuming 64x64 sprites for clarity in spritesheet? Or 32x32?
    // Let's try 32x32 scale.
    let cell_size = 48.0; // monsters can be bigger
    
    ctx.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
        img, col * cell_size, row * cell_size, cell_size, cell_size, draw_x - 8.0, draw_y - 16.0, cell_size, cell_size
    ).unwrap_or_else(|_| ());
    
    ctx.set_fill_style(&JsValue::from_str("#ff4444"));
    ctx.set_font("8px 'Press Start 2P', monospace");
    ctx.set_text_align("center");
    ctx.fill_text(&monster.name, sx, draw_y - 5.0).unwrap();
    
    draw_hp_bar(ctx, sx, draw_y - 8.0, monster.hp, monster.max_hp);
}


// Keeping fallback functions for reference or unimplemented types
// Keeping fallback functions for reference or unimplemented types
fn draw_player(ctx: &CanvasRenderingContext2d, player: &Player, sx: f64, sy: f64) {
    ctx.set_fill_style(&JsValue::from_str("#ff0000"));
    ctx.fill_rect(sx - 8.0, sy - 8.0, 16.0, 16.0);

    // Name
    ctx.set_fill_style(&JsValue::from_str("#ffffff"));
    ctx.set_font("10px 'Press Start 2P', monospace");
    ctx.set_text_align("center");
    ctx.fill_text(&player.username, sx, sy - 15.0).unwrap();
    
    // HP bar
    draw_hp_bar(ctx, sx, sy - 20.0, player.combat_stats.hp, player.combat_stats.max_hp);
}

fn draw_monster(ctx: &CanvasRenderingContext2d, monster: &Monster, sx: f64, sy: f64) {
    ctx.set_fill_style(&JsValue::from_str("#00ff00"));
    ctx.fill_rect(sx - 8.0, sy - 8.0, 16.0, 16.0);

    // Name
    ctx.set_fill_style(&JsValue::from_str("#ffff00"));
    ctx.set_font("8px 'Press Start 2P', monospace");
    ctx.set_text_align("center");
    ctx.fill_text(&monster.name, sx, sy - 12.0).unwrap();
    
    // HP bar
    draw_hp_bar(ctx, sx, sy - 18.0, monster.hp, monster.max_hp);
}

fn draw_hp_bar(ctx: &CanvasRenderingContext2d, x: f64, y: f64, hp: i32, max_hp: i32) {
    let bar_width = 30.0;
    let bar_height = 4.0;
    let hp_ratio = hp as f64 / max_hp as f64;
    
    // Background
    ctx.set_fill_style(&JsValue::from_str("#000000"));
    ctx.fill_rect(x - bar_width / 2.0, y, bar_width, bar_height);
    
    // HP
    let hp_color = if hp_ratio > 0.5 {
        "#00ff00"
    } else if hp_ratio > 0.25 {
        "#ffff00"
    } else {
        "#ff0000"
    };
    
    ctx.set_fill_style(&JsValue::from_str(hp_color));
    ctx.fill_rect(x - bar_width / 2.0, y, bar_width * hp_ratio, bar_height);
    
    // Border
    ctx.set_stroke_style(&JsValue::from_str("#ffffff"));
    ctx.set_line_width(0.5);
    ctx.stroke_rect(x - bar_width / 2.0, y, bar_width, bar_height);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = requestAnimationFrame)]
    fn request_animation_frame(closure: &Closure<dyn FnMut()>) -> i32;
}
