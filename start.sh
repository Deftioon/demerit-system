#!/bin/bash

# Start the server
cd /home/deftioon/Github/demerit-backend/backend
cargo build --release
cargo run &

# Start the frontend
cd /home/deftioon/Github/demerit-backend/frontend
bun run dev

cd ..
