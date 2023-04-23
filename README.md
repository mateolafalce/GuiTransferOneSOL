<div align="center">
  <h1>GUI Transfers one SOL</h1>

  This program, developed with fltk.rs, aims to provide a user interface for native SOL transfers on the Solana blockchain.
</div>

<h3>App logic</h3>

The app allows the user to send native SOL transfers on the Solana blockchain.

The application has three buttons: "Enter your Wallet", "Leave a message to the recipient" and "Send the Tx". When the user clicks "Enter your Wallet", a file selection dialog opens allowing the user to select a JSON file containing their wallet information. The wallet information is stored using the store_wallet function. When the user clicks "Send the Tx", the program sends the SOL transfer using the extracted wallet information and the message provided by the user.

The code snippet also creates a top bar that contains the Solana logo and the title of the app.
