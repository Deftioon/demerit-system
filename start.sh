#!/bin/bash

# Start the server
cd /home/deftioon/Github/demerit-backend/backend
cargo build --release ./backend
cargo run &

cd /home/deftioon/Github/demerit-backend/frontend
bun run dev

cd ..
