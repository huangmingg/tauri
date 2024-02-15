// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::WebviewWindowBuilder;

#[cfg(not(feature = "webview-data-url"))]
fn main() {
  compile_error!("Feature `webview-data-url` is required to run this example");
}

#[cfg(feature = "webview-data-url")]
fn main() {
  tauri::Builder::default()
    .setup(|app| {
      let html = r#"
      <html>
      <body>
      /+&'=#%?\^`{}|[]~ <br/>
      Hello World <br/>
      สวัสดีชาวโลก! <br/>
      你好世界！<br/>
      </body>
      </html>"#;
      let data = format!("data:text/html,{}", html);
      #[allow(unused_mut)]
      let mut builder =
        WebviewWindowBuilder::new(app, "Rust", tauri::WebviewUrl::DataUrl(data.parse().unwrap()));
      #[cfg(target_os = "macos")]
      {
        builder = builder.tabbing_identifier("Rust");
      }
      let _webview = builder.title("Tauri - Rust").build()?;

      Ok(())
    })
    .run(tauri::generate_context!(
      "../../examples/data-url/tauri.conf.json"
    ))
    .expect("error while running tauri application");
}
