mod adventure;
mod ai_client;
mod game_state;
mod ui;

use adventure::Adventure;
use color_eyre::Result;
use game_state::GameState;
use ui::{Action, UI};

use std::time::Duration;
use tokio::time::interval;

#[tokio::main]
async fn main() -> Result<()> {
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
                    // display spinner and loading text while generating
                    let generate_future = Adventure::generate(&game_state.settings);
                    let mut spinner_interval = interval(Duration::from_millis(100));
                    let mut loading_text_interval = interval(Duration::from_millis(5_000));

                    let adventure = tokio::select! {
                        adventure = generate_future => adventure?,
                        _ = async {
                                loop {
                                    tokio::select! {
                                        _ = spinner_interval.tick() => {
                                            ui.update_spinner(spinner_chars[spinner_index])?;
                                            spinner_index = (spinner_index + 1) % spinner_chars.len()
                                        }
                                        _ = loading_text_interval.tick() => {
                                            ui.update_loading_text()?;
                                        }
                                    }
                                }
                                #[allow(unreachable_code)]
                                Ok::<_, color_eyre::eyre::Error>(())
                            } => unreachable!()
                    };

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
