use clap::Clap;
use std::path::PathBuf;

pub mod args;
mod gui;
mod input;
mod lsp;
mod model;

#[derive(Clap)]
struct Opts {
    /// File or working directory to open
    file: Option<PathBuf>,

    /// Enable logging incoming and outgoing LSP messages to stderr
    #[clap(long)]
    verbose_lsp: bool,

    /// Inherit the stdout of LSP servers
    #[clap(long)]
    verbose_lsp_stderr: bool,
}

fn main() {
    let opts = Opts::parse();
    args::VERBOSE_LSP.store(opts.verbose_lsp, std::sync::atomic::Ordering::Relaxed);
    args::VERBOSE_LSP_STDERR.store(
        opts.verbose_lsp_stderr,
        std::sync::atomic::Ordering::Relaxed,
    );

    // open main.rs by default for development
    let path = opts.file.unwrap_or_else(|| {
        println!("Usage: fed <file>");
        std::process::exit(1);
    });

    let work_dir = path.clone().canonicalize().unwrap_or_else(|_| path.clone());

    let mut lsp_client = crate::lsp::Client::new("rls".into(), work_dir.clone());
    lsp_client
        .run()
        .map_err(|e| eprintln!("error running lsp: {}", e))
        .ok();

    let state = model::EditorState::new_ref(work_dir);
    let mut state_borrow = state.borrow_mut();
    state_borrow.set_open_file(Some(model::OpenFile::new(path.clone())));
    state_borrow.set_lsp_client(Some(lsp_client));
    drop(state_borrow);

    gui::run(state);
}
