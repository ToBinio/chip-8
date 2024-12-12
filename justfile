run path:
    cargo run --features cli --no-default-features -- {{path}}

ibm: (run "programs/ibm-logo.ch8")

chip8: (run "programs/chip8-logo.ch8")

corax: (run "programs/corax.ch8")

flags: (run "programs/flags.ch8")

keypad: (run "programs/keypad.ch8")

break: (run "programs/breakout.ch8")

tic-tac-toe: (run "programs/ultimatetictactoe.ch8")

quirks: (run "programs/quirks.ch8")

web-dev:
    cd web && bun dev

wasm-build:
    wasm-pack build --features wasm --no-default-features

wasm-update: wasm-build
    cd web && bun i