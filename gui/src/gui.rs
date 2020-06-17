use anyhow::{Context, Result};
use glib::clone;
use golemate::backends::UciOutput;
use gtk::prelude::*;
use gtk::{
    main_quit, Button, ButtonsType, DialogFlags, Entry, HeaderBar, Label, MessageDialog,
    MessageType, Orientation, SpinButton, WindowType,
};
use shakmaty::fen::Fen;

use std::ops::Deref;
use std::path::PathBuf;
use std::thread;

use golemate::analysis;
use golemate::backends::{GWasmUci, NativeUci, UciBackend};

pub struct App {
    pub window: gtk::Window,
    pub header: Header,
    pub container: gtk::Box,
    pub eval_button: Button,
}

pub struct Header {
    pub container: HeaderBar,
}

const APP_NAME: &str = "Golemate";

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

fn launch_golemate<B: Deref<Target = dyn UciBackend>>(
    backend: B,
    fen: &str,
    depth: u32,
) -> Result<UciOutput> {
    let cmds = backend.generate_uci(fen, depth);
    let output = backend.execute_uci(cmds).context("Executing UCI")?;
    Ok(output)
}

const EVALUATE_TEXT: &str = "Evaluate position";
const EVALUATING_TEXT: &str = "Evaluating...";
const VERTICAL_SPACING: i32 = 6;
const BORDER_WIDTH: u32 = 10;

const NATIVE_PANE_NAME: &str = "Native";
const GWASM_PANE_NAME: &str = "gWASM";

impl App {
    fn new() -> App {
        let window = gtk::Window::new(WindowType::Toplevel);
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

        // Setup the common controls
        let eval_button = Button::new_with_label(EVALUATE_TEXT);
        let position_fen = Entry::new();
        position_fen.set_placeholder_text(Some("FEN"));

        let depth = SpinButton::new_with_range(1.0, 100.0, 1.0);
        depth.set_value(15.0);
        let depth_label = Label::new(Some("Depth:"));
        let depth_box = gtk::Box::new(Orientation::Horizontal, 0);
        depth_box.pack_start(&depth_label, false, false, 0);
        depth_box.pack_start(&depth, true, true, 0);

        // Setup the native controls
        let engine_path = Entry::new();
        engine_path.set_placeholder_text(Some("Engine path"));

        // Setup the gWASM controls
        let gwasm_container = gtk::Box::new(Orientation::Vertical, 0);

        let wasm_path = Entry::new();
        wasm_path.set_placeholder_text(Some("WASM path"));
        let js_path = Entry::new();
        js_path.set_placeholder_text(Some("JS path"));
        let workspace_path = Entry::new();
        workspace_path.set_placeholder_text(Some("Workspace path"));
        let datadir_path = Entry::new();
        datadir_path.set_placeholder_text(Some("Datadir path"));

        gwasm_container.pack_start(&wasm_path, false, false, 0);
        gwasm_container.pack_start(&js_path, false, false, 0);
        gwasm_container.pack_start(&workspace_path, false, false, 0);
        gwasm_container.pack_start(&datadir_path, false, false, 0);

        // Add native & gWASM controls to the stack
        let stack = gtk::Stack::new();
        stack.add_titled(&engine_path, NATIVE_PANE_NAME, NATIVE_PANE_NAME);
        stack.add_titled(&gwasm_container, GWASM_PANE_NAME, GWASM_PANE_NAME);
        stack.set_homogeneous(false);
        let stackswitcher = gtk::StackSwitcher::new();
        stackswitcher.set_stack(Some(&stack));
        stackswitcher.set_hexpand(true);
        stackswitcher.set_halign(gtk::Align::Center);

        // Setup the main view
        let container = gtk::Box::new(Orientation::Vertical, VERTICAL_SPACING);
        container.pack_start(&stackswitcher, false, false, 0);
        container.pack_start(&stack, false, false, 0);
        container.pack_start(&position_fen, false, false, 0);
        container.pack_start(&depth_box, false, false, 0);
        container.pack_start(&eval_button, false, false, 0);
        window.add(&container);

        eval_button.connect_clicked(clone!(@weak window => move |eval_button| {
            let eval_button = eval_button.clone();
            eval_button.set_label(EVALUATING_TEXT);
            eval_button.set_sensitive(false);

            let fen = position_fen.get_buffer().get_text();
            let fen2: Result<Fen, _> = fen.clone().parse();
            let depth = depth.get_value_as_int() as u32;

            let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

            let engine = PathBuf::from(engine_path.get_buffer().get_text());
            let wasm = PathBuf::from(wasm_path.get_buffer().get_text());
            let js = PathBuf::from(js_path.get_buffer().get_text());
            let workspace = PathBuf::from(workspace_path.get_buffer().get_text());
            let datadir = PathBuf::from(datadir_path.get_buffer().get_text());

            let visible_child = stack.get_visible_child_name().map(|s| s.as_str().to_owned());

            thread::spawn(move || {
                let backend: Box<dyn UciBackend> = match visible_child.as_ref().map(String::as_str) {
                    Some(NATIVE_PANE_NAME) => {
                        Box::new(NativeUci::new(engine))
                    },
                    Some(GWASM_PANE_NAME) => {
                        match GWasmUci::new(&wasm, &js, workspace, datadir) {
                            Ok(back) => Box::new(back),
                            Err(e) => {
                                tx.send(Err(e)).expect("Send failed");
                                return;
                            }
                        }
                    },
                    x => panic!("Invalid pane name: {:?}", x),
                };
                let res = launch_golemate(backend, &fen, depth);
                tx.send(res).expect("Send failed");
            });

            rx.attach(None, clone!(@strong window => move |val| {
                eval_button.set_sensitive(true);
                eval_button.set_label(EVALUATE_TEXT);
                let dialog_type;
                let dialog_body;
                match val {
                    Ok(output) => {
                        let fen2 = fen2.clone().expect("internal error, invalid fen");
                        let an_res = analysis::interpret_uci(fen2, output).expect("internal analysis error");
                        dialog_body = an_res.describe();
                        dialog_type = MessageType::Info;
                    },
                    Err(e) => {
                        dialog_type = MessageType::Error;
                        dialog_body = format!("Error: {:?}", e);
                    }
                };
                let dialog = MessageDialog::new(
                    Some(&window),
                    DialogFlags::empty(),
                    dialog_type,
                    ButtonsType::Ok,
                    &dialog_body
                );
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
