# DP Management System

This project is a DP (Data Processing) Management System that consists of a Rust backend using Actix and a React TypeScript frontend. 

## Project Structure

```
dp-management-system
├── backend
│   ├── Cargo.toml
│   └── src
│       └── main.rs
├── frontend
│   ├── index.html
│   ├── package.json
│   ├── tsconfig.json
│   └── src
│       ├── App.tsx
│       └── main.tsx
└── README.md
```

## Backend Setup

1. Navigate to the `backend` directory:
   ```
   cd backend
   ```

2. Build the Rust project:
   ```
   cargo build
   ```

3. Run the Rust server:
   ```
   cargo run
   ```

The backend will start on `http://localhost:8080` by default.

## Frontend Setup

1. Navigate to the `frontend` directory:
   ```
   cd frontend
   ```

2. Install the dependencies:
   ```
   npm install
   ```

3. Start the React application:
   ```
   npm run dev
   ```

The frontend will be available at `http://localhost:3000` by default.

## Usage

- The frontend communicates with the backend to manage data processing tasks.
- You can access the frontend interface to interact with the DP management system.

## Contributing

Feel free to fork the repository and submit pull requests for any improvements or features you would like to add.

## License

This project is licensed under the MIT License.