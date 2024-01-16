use std::str::FromStr;

use futures::executor::block_on;
use imgui;
use super::global_state::*;

use sqlx::{
    *,
    mysql::*
};

#[derive(sqlx::FromRow)]
struct PersonRow {
    pub rcsid: String,
    pub firstname: String,
    pub lastname: String,
    pub gradyear: i32,
    pub rfid: String
}

impl ToString for PersonRow {
    fn to_string(&self) -> String {
        format!(
            "RCSID: {}\nWhoAmI: {} {}, '{}\nRFID Value: {}",
            self.rcsid,
            self.firstname,
            self.lastname,
            self.gradyear,
            self.rfid
        )
    }
}


async fn connect_to_sql(user:String, pass:String, server:String) -> Result<(), sqlx::Error> {
    let sql_opts = MySqlConnectOptions::new()
        .host(&server)
        .username(&user)
        .password(&pass)
        .database("motorsports");
    let mut conn = MySqlConnection::connect_with(&sql_opts).await?;

    let rows = sqlx::query_as::<_, PersonRow>("SELECT * FROM motorsports.people")
        .fetch_all(&mut conn).await?;

    for row in rows {
        println!("{}", row.to_string());
        println!();
    }

    Ok(())
}

fn login_win_pressed_ok(state: &mut ApplicationState) {
    match block_on(connect_to_sql(
        state.login_win_state.userbuf.clone(),
        state.login_win_state.passbuf.clone(),
        String::from_str("localhost").unwrap()
    )) {
        Ok(()) => {},
        Err(err) => {
            println!("Error in SQL-land:\n{}", err.to_string());
        }
    }
    state.login_win_state.should_display = false;
}

pub fn login_win_do_display(ui: &imgui::Ui, state: &mut ApplicationState) {
    let mut window_closed = false;
    let mut ok_pressed: bool = false;
    if state.login_win_state.should_display {
        ui.window("Login to Database")
            .size([500.0, 200.0], imgui::Condition::Always)
            .always_auto_resize(true)
            .flags(imgui::WindowFlags::NO_RESIZE)
            .build(|| {
                ui.text_wrapped("Input your username and password. Reminder, they're specific to the database!");
                if ui.input_text("Username", &mut state.login_win_state.userbuf)
                .flags(imgui::InputTextFlags::ENTER_RETURNS_TRUE)
                .build() {
                    ok_pressed = true;
                }
                if ui.input_text("Password", &mut state.login_win_state.passbuf).flags(
                    imgui::InputTextFlags::PASSWORD |
                    imgui::InputTextFlags::ENTER_RETURNS_TRUE
                ).build() {
                    ok_pressed = true;
                }

                if ui.button("Cancel") {
                    window_closed = true;
                    state.login_win_state.should_display = false;
                }
                
                ui.same_line();

                if ui.button("OK") {
                    ok_pressed = true;
                }

            });
        if ok_pressed {
            login_win_pressed_ok(state);
            window_closed = true;
        }
        if window_closed {
            state.login_win_state.userbuf.clear();
            state.login_win_state.passbuf.clear();
        }
    }


}