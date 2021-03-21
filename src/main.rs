use crate::model::make_ref;
use clap::Clap;
use std::path::PathBuf;

mod gui;
mod input;
mod lsp;
mod model;
pub mod args;

#[derive(Clap)]
struct Opts {
    file: Option<PathBuf>,
    #[clap(long)]
    verbose_lsp: bool,
    #[clap(long)]
    verbose_lsp_stderr: bool,
}

fn main() {
    let opts = Opts::parse();
    args::VERBOSE_LSP.store(opts.verbose_lsp, std::sync::atomic::Ordering::Relaxed);
    args::VERBOSE_LSP_STDERR.store(opts.verbose_lsp_stderr, std::sync::atomic::Ordering::Relaxed);

    // open main.rs by default for development
    let path = opts.file.unwrap_or_else(|| {
        println!("Usage: fed <file>");
        std::process::exit(1);
    });

    let work_dir = path.clone().canonicalize().unwrap_or_else(|_| path.clone());

    let mut state = make_ref(model::EditorState {
        open_file: model::OpenFile::new(path.clone()),
        input: crate::input::VimInput::new(),
        work_dir: work_dir.clone(),
        lsp_client: Some(crate::lsp::Client::new("rls".into(), work_dir)),
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

