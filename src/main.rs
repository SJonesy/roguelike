// https://github.com/globalcitizen/zomia/blob/master/USEFUL-UNICODE.md

use console_engine::{Color, ConsoleEngine, KeyCode, events::Event, pixel, rect_style, screen::Screen};
mod gamestate;

const MAP_X_SIZE: usize = 25;
const MAP_Y_SIZE: usize = 25;

fn main() {
    let world = gamestate::GameState {
        x: Vec::new(),
        y: Vec::new()
    };

    let mut input_state = InputState::Main;

    // TOOD: this should be part of ECS model
    let mut player_position = Point{x:5, y:5};

    // TODO real map
    let mut map = [["."; MAP_X_SIZE]; MAP_Y_SIZE];
    map[5][5] = "#";

    // Full screen terminal size
    // TODO: init_fill_require when screen layout is finalized
    let mut engine = ConsoleEngine::init_fill(30).unwrap();

    // Build main game view port
    let mut mw = Screen::new(
        (engine.get_width() as f32 * 0.70) as u32,
        (engine.get_height() as f32 * 0.70) as u32
    );

    // Used to display messages in the bottom left corner
    let mut message = String::new();

    // Used at launch and on window resize
    fn build_screen(mw: &mut Screen, engine: &mut ConsoleEngine) -> () {
        mw.clear();
        engine.clear_screen();
    
        mw.resize((engine.get_width() as f32 * 0.70) as u32, (engine.get_height() as f32 * 0.70) as u32);
        mw.rect_border(0, 0, mw.get_width() as i32 - 1, mw.get_height() as i32 - 1, rect_style::BorderStyle::new_heavy());
        mw.fill_circle(5,6, 3, pixel::pxl_fg('*', Color::Blue));
    }


    build_screen(&mut mw, &mut engine);

    // MAIN GAME LOOP
    loop {
        // DO CONTINUOUS LOGIC

        // DO ENGINE LOOP
        match engine.poll() {
            Event::Resize(_, _) => {
                engine.check_resize();
                build_screen(&mut mw, &mut engine);
            }

            // DRAW EVERY FRAME
            Event::Frame => {
                // PRINT WORLD
                let viewport_width = (mw.get_width() - 1) as i32;
                let viewport_height = (mw.get_height() - 1) as i32;
                let viewport_topleft = Point {
                    x: player_position.x - (viewport_width / 2),
                    y: player_position.y - (viewport_height / 2)
                };
                for x_offset in 1..viewport_width {
                    for y_offset in 1..viewport_height {
                        let next_map_x = (viewport_topleft.x + x_offset) as usize;
                        let next_map_y = (viewport_topleft.y + y_offset) as usize;

                        // Check for out of bounds
                        // TODO: Smarter skipping of out of bound ranges
                        // using usize rollover here
                        if next_map_x > MAP_X_SIZE - 1 {
                            mw.print(x_offset, y_offset, " ");
                            continue;
                        }
                        if next_map_y > MAP_Y_SIZE - 1 {
                            mw.print(x_offset, y_offset, " ");
                            continue;
                        }

                        mw.print(
                            x_offset,
                            y_offset, 
                            map[next_map_x][next_map_y]
                        );

                        // TODO REALLY NEED TO MAKE ECS MAPPING FOR PLAYER
                        if next_map_x == player_position.x as usize && next_map_y == player_position.y as usize {
                            mw.print(
                                x_offset,
                                y_offset, 
                                "@"
                            );
                        }
                    }
                }

                engine.print_screen(1, 2, &mw);

                if input_state != InputState::Exiting {
                    message = format!("player X:{}, Y:{}; mw w:{}, h:{}; viewport w:{}, h:{} tl:x:{}, y:{}",
                        player_position.x, player_position.y, mw.get_width(), mw.get_height(),
                        viewport_width, viewport_height, viewport_topleft.x, viewport_topleft.y);
                }
                engine.draw();
                engine.print(1, engine.get_height() as i32 - 1, &message);
            }

            // HANDLE INPUT
            Event::Key(keyevent) => {
                match keyevent.code {
                    // Clear all state on ESC
                    KeyCode::Esc => { 
                        input_state = InputState::Exiting;
                        message.clear();
                    },
                    // Exit
                    KeyCode::Char('q') => { 
                        if input_state == InputState::Exiting {
                            return();
                        }
                        else {
                            message = String::from("Press (q) again to exit.                                               ");
                            input_state = InputState::Exiting;
                        }
                    },
                    // Movement
                      KeyCode::Char('1')
                    | KeyCode::Char('2')
                    | KeyCode::Char('3')
                    | KeyCode::Char('4')
                    | KeyCode::Char('5')
                    | KeyCode::Char('6')
                    | KeyCode::Char('7')
                    | KeyCode::Char('8')
                    | KeyCode::Char('9') => {
                        input_state = InputState::Main;
                        movement(keyevent.code, &mut &mut player_position);
                    },
                    // Default
                    _ => ()
                };

                // message = format!("Key: {:?}", keyevent.code);
            }

            // Mouse has been moved or clicked
            Event::Mouse(_mouseevent) => {
                // message = format!(
                //     "Mouse: {:?} ({},{})",
                //     mouseevent.kind, mouseevent.column, mouseevent.row
                // );
            }


        }
    }
}


fn movement(keycode: KeyCode, player_position: &mut Point) {
    let mut move_x = 0;
    let mut move_y = 0;

    match keycode {
        KeyCode::Char('1') => { move_x -= 1; move_y += 1; },
        KeyCode::Char('2') => { move_y += 1; },
        KeyCode::Char('3') => { move_x += 1; move_y += 1; },
        KeyCode::Char('4') => { move_x -= 1; },
        KeyCode::Char('5') => { },
        KeyCode::Char('6') => { move_x += 1; },
        KeyCode::Char('7') => { move_x -= 1; move_y -= 1; },
        KeyCode::Char('8') => { move_y -= 1; },
        KeyCode::Char('9') => { move_x += 1; move_y -= 1; },
        _ => ()
    };

    let target_player_x = (move_x + player_position.x) as usize;
    let target_player_y = (move_y + player_position.y) as usize;

    // TODO check for entity

    if target_player_x > MAP_X_SIZE -1 || target_player_y > MAP_Y_SIZE -1 {
        return;
    }

    player_position.x += move_x;
    player_position.y += move_y;
}

#[derive(PartialEq)]
enum InputState {
    Main,
    Exiting,
}


struct Point {
    pub x: i32,
    pub y: i32,
}

// #[derive(Copy, Clone)]
// enum Tile {
//     None
// }