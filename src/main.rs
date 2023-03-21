//#![windows_subsystem = "windows"]
mod functions;
use functions::{
    send_sol,
    store_wallet,
    clear
};
use fltk::{
    prelude::*,
    enums::{
        Color,
        FrameType,
        Font,
        Cursor
    },
    window::Window,
    frame::Frame,
    app::App,
    image::IcoImage,
    button::Button,
    input::Input,
    dialog::{
        NativeFileChooser,
        NativeFileChooserType,
        alert
    },
};
use std::{
    path::Path,
    fs::File,
    io::Read,
};
use serde_json::Value;

#[tokio::main]
async fn main() {
    clear::clear().unwrap();
    let icon: IcoImage = IcoImage::load(&Path::new("src/transfer-sol.ico")).unwrap();
    let app: App = App::default();

    //MAIN WINDOWS
    let mut wind: Window = Window::new(100, 100, 1000, 600, "GUI - Transfer SOL");
    wind.set_icon(Some(icon));
    wind.set_border(true);
    wind.set_color(Color::from_rgb(155,155,155));
    wind.make_resizable(true);

    //BODY
    let mut btn_input_wallet: Button = Button::new(350, 90, 350, 30, "Enter your Wallet");
    btn_input_wallet.set_frame(FrameType::GleamThinDownBox);
    let mut leave_a_message: Frame = Frame::new(370, 135, 300, 30, "Leave a message to the recipient");
    leave_a_message.set_label_size(16);
    let mut message: Input = Input::new(350, 165, 350, 30, "");
    message.set_frame(FrameType::RoundedBox);
    let mut btn_send_tx: Button = Button::new(350, 215, 350, 30, "Send the Tx");
    btn_send_tx.set_frame(FrameType::GleamThinDownBox);
    btn_send_tx.set_color(Color::from_rgb(0,143,57));

    //TOP-BAR
    let mut bar: Window = Window::new(0, 0, 1000, 50, "");
    bar.set_color(Color::from_rgb(0,0,0));
    bar.make_resizable(true);
    let mut solana_logo: IcoImage = IcoImage::load("src/transfer-sol.ico").unwrap();
    bar.draw(move |f| {
        solana_logo.scale(40, 40, true, true);
        solana_logo.draw(f.x() + 30, f.y() + 5, f.w(), f.h());
    });
    let mut bar_text: Frame = Frame::new(15, 10, 1000, 30, "Transfer One SOL");
    bar_text.set_label_font(Font::by_index(2));
    bar_text.set_label_color(Color::from_rgb(255, 255, 255));
    bar_text.set_label_size(25);

    //HERE GET THE WALLET
    btn_input_wallet.set_callback(move |_| {
        let mut dialog: NativeFileChooser = NativeFileChooser::new(NativeFileChooserType::BrowseFile);
        dialog.set_filter("*.json");
        dialog.show();
        let mut file: File = File::open(dialog.filename()).expect("could not open file");
        let mut contents: String = String::new();
        file.read_to_string(&mut contents).expect("could not read file");
        let json: Value = serde_json::from_str(&contents).expect("Error");
        let content = serde_json::from_value(json);
        clear::clear().unwrap();
        match content {
            Ok(content)=> store_wallet::store_wallet(content).expect("Error"),
            Err(_) => {
                let _result = alert(500, 300, "Error processing the wallet");
            },
        }
    });

    //HERE THE PROGRAM SEND THE TX
    btn_send_tx.set_callback(move|_| {
        let msg: String = message.value();
        let tx = async move {
            send_sol::send_sol(msg).expect("Error");
            clear::clear().unwrap();
        };
        tokio::spawn(tx);
    });

    wind.end();
    wind.show();
    app.run().unwrap();
}
