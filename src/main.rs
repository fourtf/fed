// mod lsp;
// use serde::Serialize;
// use std::{io::Read, sync::mpsc::channel, env};

use crate::model::make_ref;
use clap::Clap;
use std::path::PathBuf;

mod gui;
mod model;
mod input;
//mod syntax;

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

    let state = make_ref(model::EditorState {
        open_file: model::OpenFile::new(path.clone()),
        input: crate::input::VimInput::new(),
        work_dir: path,
    });

    gui::run(state);
}

// fn main() {
//     println!("Hello, world!");

//     let (tx, rx) = channel();

//     let client = lsp::Client::new("rls".into());
//     client.run(rx).unwrap();

//     let req = make_init();

//     println!("{}", String::from_utf8(to_json(&req)).unwrap());

//     tx.send(to_json(&req)).unwrap();

//     std::io::stdin().bytes().next();
// }

// fn to_json<T: Serialize>(t: &T) -> Vec<u8> {
//     let data = serde_json::to_string(&t).unwrap();

//     //Content-Type: application/vscode-jsonrpc; charset=utf-8\r\n
//     let data = format!(
//         "Content-Length: {}\r\n\r\n{}",
//         data.len(),
//         data
//     );

//     return data.bytes().collect();
// }

// fn make_init() -> lsp::Request::<lsp::InitializeParams> {
//     let current_dir = env::current_dir().unwrap();

//     return lsp::Request::<lsp::InitializeParams> {
//         id: lsp::RequestId::Number(0),
//         method: "initialize".into(),
//         params: Some(
//             lsp::InitializeParams {
//                 rootPath : Some(current_dir.to_string_lossy().into()),
//                 ..Default::default()
//             }
//         ),
//     };
// }
