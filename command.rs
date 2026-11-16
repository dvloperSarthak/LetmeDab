fn letmedab_command(game: &mut crate::game::Game<'_>, command: &str, ran: &mut bool) {
    if let Some(output) = crate::game::letmedab::handle_command(command) {
        *ran = true;
        log::info!("[LetmeDab] {}", output);
    }
}
