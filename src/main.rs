extern crate web_view;

use web_view::*;

fn main() {
    let size = (400, 700);
    let resizable = true;
    let debug = false;
    let titlebar_transparent = true;
    let frontend_cb = |_webview: &mut _, _arg: &_, _userdata: &mut _| {};
    let userdata = ();

    let html = format!(include_str!("../www/index.html"),
                       css = include_str!("../www/main.css"),
                       js = include_str!("../www/main.js"));

    run(
        "",
        Content::Html(html),
        Some(size),
        resizable,
        debug,
        titlebar_transparent,
        move |mut webview| {
            webview.set_background_color(0.11, 0.12, 0.13, 1.0);
        },
        frontend_cb,
        userdata
    );
}