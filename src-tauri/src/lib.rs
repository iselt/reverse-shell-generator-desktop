// Desktop shell for reverse-shell-generator.
// Routes outbound HTTP(S) through a local proxy on 127.0.0.1:7897
// (common Clash / V2Ray mixed-port default) when present.

use std::env;

const DEFAULT_PROXY: &str = "http://127.0.0.1:7897";

fn apply_local_proxy() {
    // Prefer an explicit override, otherwise default to the local mixed port.
    let proxy = env::var("RSG_HTTP_PROXY")
        .or_else(|_| env::var("HTTPS_PROXY"))
        .or_else(|_| env::var("HTTP_PROXY"))
        .or_else(|_| env::var("https_proxy"))
        .or_else(|_| env::var("http_proxy"))
        .unwrap_or_else(|_| DEFAULT_PROXY.to_string());

    // Keep loopback / LAN traffic direct so local raw/blob URLs still work.
    let no_proxy = env::var("NO_PROXY")
        .or_else(|_| env::var("no_proxy"))
        .unwrap_or_else(|_| "localhost,127.0.0.1,::1".to_string());

    // Standard env vars consulted by curl, reqwest, and most system stacks.
    // SAFETY: set before any multi-threaded work starts (called from setup).
    unsafe {
        env::set_var("HTTP_PROXY", &proxy);
        env::set_var("HTTPS_PROXY", &proxy);
        env::set_var("http_proxy", &proxy);
        env::set_var("https_proxy", &proxy);
        env::set_var("ALL_PROXY", &proxy);
        env::set_var("all_proxy", &proxy);
        env::set_var("NO_PROXY", &no_proxy);
        env::set_var("no_proxy", &no_proxy);
    }

    eprintln!("[rsg-desktop] network proxy: {proxy} (NO_PROXY={no_proxy})");
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    apply_local_proxy();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
