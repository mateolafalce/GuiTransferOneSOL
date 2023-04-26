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
    // Clear the data
    clear::clear().unwrap();
    // Load the icon image for the window
    let icon: IcoImage = IcoImage::load(&Path::new("src/transfer-sol.ico")).unwrap();
    // Create the main application window
    let app: App = App::default();
    let mut wind: Window = Window::new(100, 100, 1000, 600, "GUI - Transfer SOL");
    wind.set_icon(Some(icon));
    wind.set_border(true);
    wind.set_color(Color::from_rgb(155,155,155));
    wind.make_resizable(true);
    // Create the input button for the wallet address
    let mut btn_input_wallet: Button = Button::new(350, 90, 350, 30, "Enter your Wallet");
    btn_input_wallet.set_frame(FrameType::GleamThinDownBox);
    // Create the frame and input box for leaving a message
    let mut leave_a_message: Frame = Frame::new(370, 135, 300, 30, "Leave a message to the recipient");
    leave_a_message.set_label_size(16);
    let mut message: Input = Input::new(350, 165, 350, 30, None);
    message.set_frame(FrameType::RoundedBox);
    // Create the send button for sending the transaction
    let mut btn_send_tx: Button = Button::new(350, 215, 350, 30, "Send the Tx");
    btn_send_tx.set_frame(FrameType::GleamThinDownBox);
    btn_send_tx.set_color(Color::from_rgb(0,143,57));
    // Create the top bar with the Solana logo and the transfer title
    let mut bar: Window = Window::new(0, 0, 1000, 50, None);
    bar.set_color(Color::from_rgb(0,0,0));
    bar.make_resizable(true);
    let mut solana_logo: IcoImage = IcoImage::load("src/transfer-sol.ico").unwrap();
    bar.draw(move |f| {
        solana_logo.scale(40, 40, true, true);
        solana_logo.draw(f.x() + 30, f.y() + 5, f.w(), f.h());
    });

    // Create a new Frame object with the given parameters
    let mut bar_text: Frame = Frame::new(15, 10, 1000, 30, "Transfer One SOL");
    // Set the font of the label of the Frame object
    bar_text.set_label_font(Font::by_index(2));
    // Set the color of the label of the Frame object
    bar_text.set_label_color(Color::from_rgb(255, 255, 255));
    // Set the size of the label of the Frame object
    bar_text.set_label_size(25);
    // Set the callback function for the input wallet button
    btn_input_wallet.set_callback(move |_| {
        // Create a new NativeFileChooser object
        let mut dialog: NativeFileChooser = NativeFileChooser::new(NativeFileChooserType::BrowseFile);
        dialog.set_filter("*.json"); // Set the filter for the file chooser to JSON files
        dialog.show(); // Show the file chooser dialog
        // Open the selected file
        let mut file: File = File::open(dialog.filename()).expect("could not open file");
        // Read the contents of the file into a String object
        let mut contents: String = String::new();
        file.read_to_string(&mut contents).expect("could not read file");
        // Parse the contents of the file as JSON
        let json: Value = serde_json::from_str(&contents).expect("Error");
        // Deserialize the JSON into a Rust data structure
        let content = serde_json::from_value(json);
        clear::clear().unwrap();
        // Match on the result of deserialization
        match content {
            // If deserialization succeeded, store the wallet
            Ok(content)=> store_wallet::store_wallet(content).expect("Error"),
            // If deserialization failed, show an error message
            Err(_) => {
                let _result = alert(500, 300, "Error processing the wallet");
            },
        }
    });
    // Set the callback function for the send transaction button
    btn_send_tx.set_callback(move|_| {
        // Get the message to send from the message input field
        let msg: String = message.value();
        // Create an asynchronous task to send the transaction
        let tx = async move {
            send_sol::send_sol(msg).expect("Error");
            clear::clear().unwrap();
        };
        tokio::spawn(tx); // Spawn the asynchronous task
    });
    wind.end(); // End the creation of the window
    wind.show(); // Show the window
    app.run().unwrap(); // Run the application event loop
}
