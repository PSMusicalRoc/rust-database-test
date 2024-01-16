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

pub fn login_win_do_display(ui: &imgui::Ui, state: &mut ApplicationState) {
    let log_state = &mut state.login_win_state;
    let mut window_closed = false;
    if log_state.should_display {
        ui.window("Login to Database")
            .size([500.0, 200.0], imgui::Condition::Always)
            .always_auto_resize(true)
            .flags(imgui::WindowFlags::NO_RESIZE)
            .build(|| {
                ui.text_wrapped("Input your username and password. Reminder, they're specific to the database!");
                ui.input_text("Username", &mut log_state.userbuf).build();
                ui.input_text("Password", &mut log_state.passbuf).flags(
                    imgui::InputTextFlags::PASSWORD
                ).build();

                if ui.button("Cancel") {
                    window_closed = true;
                    log_state.should_display = false;
                }
                
                ui.same_line();

                if ui.button("OK") {
                    match block_on(connect_to_sql(
                        log_state.userbuf.clone(),
                        log_state.passbuf.clone(),
                        String::from_str("localhost").unwrap()
                    )) {
                        Ok(()) => {},
                        Err(err) => {
                            println!("Error in SQL-land:\n{}", err.to_string());
                        }
                    }
                    window_closed = true;
                    log_state.should_display = false;
                }

            });
        if window_closed {
            log_state.userbuf.clear();
            log_state.passbuf.clear();
        }
    }


}