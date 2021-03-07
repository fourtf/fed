// mod lsp;
// use serde::Serialize;
// use std::{io::Read, sync::mpsc::channel, env};

use crate::model::make_ref;
use clap::Clap;
use std::path::PathBuf;

mod gui;
mod model;
mod input;

#[derive(Clap)]
struct Opts {
    file: Option<PathBuf>,
}

fn main() {
    let opts = Opts::parse();

    // open main.rs by default for development
    let path = opts.file.unwrap_or_else(|| {
        let mut path = std::env::current_dir().unwrap();
        path.push("src");
        path.push("main.rs");
        path
    });

    let doc = model::TextModel::new();
    let doc = match doc.load_from(&path) {
        Err(e) => { println!("Error loading file: {}", e); doc },
        Ok(doc) => doc,
    };

    let state = make_ref(model::EditorState {
        open_file: model::OpenFile {
            model: doc,
            path,
            ..Default::default()
        },
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
