use imgui;
use super::global_state::*;

pub fn set_server_win_do_display(ui: &imgui::Ui, state: &mut SetServerWinState) {
    let mut did_close_window: bool = false;
    if state.should_display {
        state.should_clear_buf = true;
        ui.window("Set Database Server")
            .size([400.0, 200.0], imgui::Condition::FirstUseEver)
            .opened(&mut state.should_display)
            .build(|| {
                ui.text_wrapped("Enter the URL for your SQL server into the box below, then press OK.");
                ui.set_cursor_pos([ui.cursor_pos()[0], ui.cursor_pos()[1] + 20.0]);
                ui.input_text("SQL Database URL", &mut state.string_buf).build();
                if ui.button("Cancel") {
                    did_close_window = true;
                }
                
                ui.same_line();

                if ui.button("OK") {
                    did_close_window = true;
                }
            });

        if did_close_window {
            state.should_display = false;
        }
    } else if state.should_clear_buf {
        state.string_buf.clear();
        state.should_clear_buf = false;
    }
}