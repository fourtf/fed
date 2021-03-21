use crate::model::make_ref;
use clap::Clap;
use std::path::PathBuf;

mod gui;
mod input;
mod lsp;
mod model;

#[derive(Clap)]
struct Opts {
    file: Option<PathBuf>,
}

fn main() {
    let opts = Opts::parse();

    // open main.rs by default for development
    let path = opts.file.unwrap_or_else(|| {
        println!("Usage: fed <file>");
        std::process::exit(1);
    });

    let mut state = make_ref(model::EditorState {
        open_file: model::OpenFile::new(path.clone()),
        input: crate::input::VimInput::new(),
        work_dir: path.clone(),
        lsp_client: Some(crate::lsp::Client::new("rls".into(), path)),
    });

    match &mut state.borrow_mut().lsp_client {
        Some(client) => {
            client
                .run()
                .map_err(|e| eprintln!("error running lsp: {}", e))
                .ok();
            ()
        }
        None => (),
    }

    gui::run(state);
}

