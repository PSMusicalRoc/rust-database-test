mod libraries;
mod windows;

use libraries::logger::log;

use windows::{
    *,
    tomlstruct::*,
    global_state::*
};

use sdl2::{
    video::*,
    event::*
};

use imgui::Context;
use imgui_sdl2_support::SdlPlatform;
use imgui_glow_renderer::{
    AutoRenderer,
    glow,
    glow::HasContext
};

const ROC_GL_MAJOR_VERSION: u8 = 3;
const ROC_GL_MINOR_VERSION: u8 = 3;


fn glow_context(window: &Window) -> imgui_glow_renderer::glow::Context {
    unsafe {
        log::info("Creating OpenGL Context!");
        imgui_glow_renderer::glow::Context::from_loader_function(|s| window.subsystem().gl_get_proc_address(s) as _)
    }
}

fn main() {
    if ensure_data_exists("data.toml").is_err() {
        panic!("Somehow, we could not ensure that data.toml exists!");
    }

    let mut data: TomlData = tomlstruct::load_tomldata("data.toml");
    let mut state: ApplicationState = ApplicationState {..Default::default()}; 

    log::info("Welcome to Roc's Oxidized Test!");

    log::info("Creating SDL Context!");
    let sdl_context = match sdl2::init() {
        Ok(outsdl) => {outsdl},
        Err(msg) => {
            log::error(msg.as_str(), line!(), file!());
            panic!();
        }
    };

    log::info("Attaining the video subsystem...");
    let video_subsystem = match sdl_context.video() {
        Ok(subsys) => {subsys},
        Err(msg) => {
            log::error(msg.as_str(), line!(), file!());
            panic!();
        }
    };


    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_version(ROC_GL_MAJOR_VERSION, ROC_GL_MINOR_VERSION);
    gl_attr.set_context_profile(GLProfile::Core);
    log::info(format!(
        "Setting GL Version to {}.{} Core...",
        ROC_GL_MAJOR_VERSION,
        ROC_GL_MINOR_VERSION
    ).as_str());

    // let window: *mut SDL_Window = SDL_CreateWindow(b"A thing\0".as_ptr().cast(),
    // SDL_WINDOWPOS_CENTERED, SDL_WINDOWPOS_CENTERED,
    // 1000, 800, 0);

    log::info("Initializing window...");
    let window = match video_subsystem.window(
        "Oxidized Demo", data.windowsettings.width, data.windowsettings.height
    ).position_centered()
        .resizable()
        .opengl()
        .build() {
            Ok(win) => {win},
            Err(msg) => {
                log::error(msg.to_string().as_str(), line!(), file!());
                panic!();
            }
        };
        
    
    log::info("Creating SDL2 OpenGL Context...");
    let gl_context = match window.gl_create_context() {
        Ok(context) => {context},
        Err(msg) => {
            log::error(msg.as_str(), line!(), file!());
            panic!();
        }
    };
    window.gl_make_current(&gl_context).unwrap();

    // Uncommenting the below will enable vsync
    // window.subsystem().gl_set_swap_interval(1).unwrap();


    // Imgui and glow contexts

    let gl = glow_context(&window);
    let mut imgui = Context::create();

    // disables ini files
    imgui.set_ini_filename(None);
    imgui.set_log_filename(None);

    // set some basic imgui features, like docking support
    imgui.io_mut().config_flags |= imgui::ConfigFlags::DOCKING_ENABLE;
    imgui.io_mut().config_docking_with_shift = true;


    // add fonts

    imgui.fonts()
        .add_font(&[imgui::FontSource::TtfData {
            data: include_bytes!("../resources/fonts/inter_font.ttf"),
            size_pixels: 16.0,
            config: None
        }]);

    let mut platform = SdlPlatform::init(&mut imgui);
    let mut renderer = AutoRenderer::initialize(gl, &mut imgui).unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();


    // Main Loop

    'mainloop: loop {
        for event in event_pump.poll_iter() {
            platform.handle_event(&mut imgui, &event);
            
            if let Event::Quit {..} = event {
                // save our current tomldata such that it reloads
                // next time we run
                match tomlstruct::write_tomldata("data.toml", &data) {
                    Ok(()) => {
                        log::info("Succesfully saved data.toml!");
                    },
                    Err(msg) => {
                        log::error(msg.as_str(), line!(), file!());
                    }
                }

                break 'mainloop;
            }

            if let Event::Window {win_event, ..} = event {
                match win_event {
                    WindowEvent::Resized(w, h) => {
                        data.windowsettings.width = w.try_into().unwrap();
                        data.windowsettings.height = h.try_into().unwrap();
                    },
                    _ => {}
                }
            }
        }

        let mut menubar_height: f32 = 0.0;

        platform.prepare_frame(&mut imgui, &window, &event_pump);
        let ui = imgui.new_frame();

        ui.main_menu_bar(|| {

            ui.menu("Debug - Windows", || {
                if ui.menu_item("Change Server Window") {
                    state.set_server_win_state.should_display = true;
                }
                if ui.menu_item("Login Window") {
                    state.login_win_state.should_display = true;
                }
            });

            menubar_height = ui.window_size()[1];
        });

        ui.window("mainwindow")
            .size(
                [ui.io().display_size[0], ui.io().display_size[1] - menubar_height],
                imgui::Condition::Always
            )
            .position(
                [0.0, menubar_height], imgui::Condition::Always
            )
            .flags(
                imgui::WindowFlags::NO_BRING_TO_FRONT_ON_FOCUS |
                imgui::WindowFlags::NO_MOVE |
                imgui::WindowFlags::NO_DECORATION |
                imgui::WindowFlags::NO_RESIZE
            )
            .bg_alpha(0.2)
            .build(|| {
            });

        set_server_win::set_server_win_do_display(&ui, &mut state.set_server_win_state);
        login_win::login_win_do_display(&ui, &mut state);

        let draw_data = imgui.render();

        unsafe { renderer.gl_context().clear( glow::COLOR_BUFFER_BIT ) };
        renderer.render(draw_data).unwrap();

        window.gl_swap_window();

    }

    log::info("Exiting Roc's Oxidized Test!");
}