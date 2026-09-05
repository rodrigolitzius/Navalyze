# ==== Building the backend ====
cd src/backend/
cargo +nightly build --release

cd ../

# ==== building the frontend (npm) ====
cd frontend

# Installing dependencies for the website
npm install

npx vite build
