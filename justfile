alias dev-front := run-frontend-dev
alias dev-back := run-backend-dev

# ==== RELEASE ====
run params:
    cd ./build/ && ./release {{params}}

build: backend-rel frontend-rel

# ==== RUNNING DEV ====
run-backend-dev params: backend-dev
    cd ./build/ && ./debug {{params}}

run-frontend-dev:
    cd ./src/frontend/ && npx vite

# ==== BUILDING ====
backend-dev: build-dir
    cd ./src/backend/ && cargo +nightly build
    cp ./src/backend/target/debug/backend ./build/debug

backend-rel: build-dir
    cd ./src/backend/ && cargo +nightly build --release
    cp ./src/backend/target/release/backend ./build/release

frontend-rel: build-dir npm-dep
    cd ./src/frontend/ && npx vite build
    rm -rf ./build/dist
    cp -r ./src/frontend/dist ./build/dist

npm-dep:
    cd ./src/frontend/ && npm install

build-dir:
    mkdir -p build
