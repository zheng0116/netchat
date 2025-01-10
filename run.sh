case $1 in 
"clean")
    cargo clean
    rm cargo.lock
    ;;
"format")
    cargo fmt 
    cargo clippy
    ;;
"build")
    cargo build --release
    mv target/release/netchat bin/
    ;;
"start")
    # Generate JWT secret if not exists
    if [ ! -f .jwt_secret ]; then
        openssl rand -base64 32 > .jwt_secret
    fi
    export JWT_SECRET=$(cat .jwt_secret)
    
    if [ "$2" = "dev" ]; then
        if ! command -v cargo-watch &> /dev/null; then
            echo "Installing cargo-watch..."
            cargo install cargo-watch
        fi
        echo "Starting in development mode..."
        cargo watch -x run
    else
        echo "Starting in production mode..."
        cargo run --release
    fi
    ;;
esac