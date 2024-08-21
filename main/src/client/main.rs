use super::view;

pub fn run() -> Result<(), view::ViewError> {
    let mut player = view::welcome_player()?;
    view::returning_warriors(&mut player)?;
    view::register_to_tournament(&mut player)?;
    Ok(())
}
