pub struct LoginWinState {
    pub should_display: bool,
    pub userbuf: String,
    pub passbuf: String
}

impl Default for LoginWinState {
    fn default() -> Self {
        LoginWinState {
            should_display: false,
            userbuf: String::with_capacity(200),
            passbuf: String::with_capacity(200)
        }
    }
}

pub struct SetServerWinState {
    pub should_display: bool,
    pub should_clear_buf: bool,
    pub string_buf: String
}

impl Default for SetServerWinState {
    fn default() -> Self {
        SetServerWinState {
            should_display: false,
            should_clear_buf: false,
            string_buf: String::with_capacity(200)
        }
    }
}

/* GLOBAL APPLICATION STATE */

pub struct ApplicationState {
    pub set_server_win_state: SetServerWinState,
    pub login_win_state: LoginWinState
}

impl Default for ApplicationState {
    fn default() -> Self {
        ApplicationState {
            set_server_win_state: SetServerWinState {..Default::default()},
            login_win_state: LoginWinState {..Default::default()}
        }
    }
}