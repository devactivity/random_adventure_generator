mod adventure;
mod game_state;
mod ui;

use adventure::Adventure;
use color_eyre::eyre::Result;
use game_state::GameState;
use ui::{Action, UI};

use std::thread;
use std::time::Duration;

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut game_state = GameState::new();
    let mut ui = UI::new()?;

    let spinner_chars = ['|', '/', '-', '\\'];
    let mut spinner_index = 0;

    loop {
        ui.display(&game_state)?;

        if let Some(action) = ui.handle_input()? {
            match action {
                Action::Generate => {
                    // Display spinner while generating
                    for _ in 0..10 {
                        ui.display_spinner(spinner_chars[spinner_index])?;
                        spinner_index = (spinner_index + 1) % spinner_chars.len();
                        thread::sleep(Duration::from_millis(100));
                    }

                    let adventure = Adventure::generate(&game_state.settings);
                    game_state.set_current_adventure(adventure);
                }
                Action::Customize => {
                    game_state.customize_settings(&mut ui.terminal)?;
                }
                Action::Save => {
                    game_state.save_adventure()?;
                }
                Action::Load => {
                    game_state.load_adventure()?;
                }
                Action::Quit => break,
            }
        }
    }

    Ok(())
}
