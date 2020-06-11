use anyhow::{Context, Result};
use glib::clone;
use golemate::backends::UciOutput;
use gtk::prelude::*;
use gtk::{
    main_quit, Box, Button, ButtonsType, DialogFlags, Entry, HeaderBar, Label, MessageDialog,
    MessageType, Orientation, SpinButton, Window, WindowType,
};
use std::path::PathBuf;
use std::thread;

pub struct App {
    pub window: Window,
    pub header: Header,
    pub container: Box,
    pub eval_button: Button,
}

pub struct Header {
    pub container: HeaderBar,
}

const APP_NAME: &str = "Golemate";

fn launch_golemate_native(engine_path: PathBuf, fen: &str, depth: u32) -> Result<UciOutput> {
    use golemate::backends::{NativeUci, UciBackend};
    let backend = NativeUci::new(engine_path);
    let cmds = backend.generate_uci(fen, depth);
    let output = backend.execute_uci(cmds).context("Executing UCI")?;
    // if opts.raw_uci {
    //     for line in output {
    //         println!("{}", line);
    //     }
    // } else {
    //     let an_res = analysis::interpret_uci(opts.fen, output)?;
    //     println!(
    //         "Analysis depth: {}. {}. The best move is {}",
    //         an_res.depth,
    //         an_res.describe_advantage(),
    //         an_res.best_move
    //     )
    // }
    println!("output: {:?}", output);

    Ok(output)
}

const EVALUATE_TEXT: &str = "Evaluate position";
const EVALUATING_TEXT: &str = "Evaluate position";
const VERTICAL_SPACING: i32 = 6;
const BORDER_WIDTH: u32 = 10;

impl App {
    fn new() -> App {
        let window = Window::new(WindowType::Toplevel);
        let header = Header::new();

        window.set_titlebar(Some(&header.container));
        window.set_title(APP_NAME);
        window.set_border_width(BORDER_WIDTH);
        window.set_default_geometry(800, 600);
        // Set the window manager class.
        //window.set_wmclass("golemate-name", APP_NAME);

        window.connect_delete_event(move |_, _| {
            main_quit();
            Inhibit(false)
        });

        let container = Box::new(Orientation::Vertical, VERTICAL_SPACING);
        let eval_button = Button::new_with_label(EVALUATE_TEXT);
        let engine_path = Entry::new();
        engine_path.set_placeholder_text(Some("Engine path"));
        let position_fen = Entry::new();
        position_fen.set_placeholder_text(Some("FEN"));

        let depth = SpinButton::new_with_range(1.0, 100.0, 1.0);
        depth.set_value(15.0);
        let depth_label = Label::new(Some("Depth:"));
        let depth_box = Box::new(Orientation::Horizontal, 0);
        depth_box.pack_start(&depth_label, false, false, 0);
        depth_box.pack_start(&depth, true, true, 0);

        container.pack_start(&engine_path, false, false, 0);
        container.pack_start(&position_fen, false, false, 0);
        container.pack_start(&depth_box, false, false, 0);
        container.pack_start(&eval_button, false, false, 0);
        window.add(&container);

        eval_button.connect_clicked(clone!(@weak window => move |eval_button| {
            let eval_button = eval_button.clone();
            eval_button.set_label(EVALUATING_TEXT);
            eval_button.set_sensitive(false);

            let fen = position_fen.get_buffer().get_text();
            let engine = PathBuf::from(engine_path.get_buffer().get_text());
            let depth = depth.get_value_as_int() as u32;

            let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

            thread::spawn(move || {
                // FIXME set depth
                let res = launch_golemate_native(engine, &fen, depth);
                tx.send(res)
            });

            rx.attach(None, clone!(@strong window => move |val| {
                eval_button.set_sensitive(true);
                eval_button.set_label(EVALUATE_TEXT);
                let dialog = match val {
                    Ok(output) => MessageDialog::new(
                        Some(&window),
                        DialogFlags::empty(),
                        MessageType::Info,
                        ButtonsType::Ok,
                        &output.join("\n"),
                    ),
                    Err(e) => MessageDialog::new(
                        Some(&window),
                        DialogFlags::MODAL,
                        MessageType::Error,
                        ButtonsType::Ok,
                        &format!("Error: {:?}", e),
                    )
                };
                dialog.run();
                dialog.destroy();
                Continue(true)
            }));
        }));

        let app = App {
            window: window.clone(),
            header,
            container,
            eval_button: eval_button.clone(),
        };

        app
    }
}

impl Header {
    fn new() -> Header {
        let container = HeaderBar::new();

        container.set_title(Some("Golemate"));
        container.set_show_close_button(true);

        Header { container }
    }
}

fn main() -> Result<()> {
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );

    if gtk::init().is_err() {
        anyhow::bail!("failed to initialize GTK Application");
    }

    let app = App::new();
    app.window.show_all();
    gtk::main();

    Ok(())
}
