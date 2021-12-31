# BoringSSL example

Using `boring` crate from iOS application.

1. Checkout
   ```
   git clone git@github.com:ilammy/boring-example.git
   cd boring-example
   git submodule init
   git submodule update
   ```
2. Install Rust toolchains for iOS
   ```
   rustup target add aarch64-apple-ios      # iOS devices
   rustup target add aarch64-apple-ios-sim  # iOS Simulator (for M1)
   rustup target add x86_64-apple-ios       # iOS Simulator (for Intel)
   ```
3. Build `boring-example`
   ```
   cd boring-example
   cargo build --release --target aarch64-apple-ios
   cargo build --release --target x86_64-apple-ios
   ```
4. Tweak Xcode project
   * rustc cannot build fat binaries.
     You will need some jockeying to configure the project to link against the proper `libboring_example.a`.
     Currently it's configured for `x86_64-apple-ios`.
5. Build & run
   * You should see some SHA-256 computed on the screen.
