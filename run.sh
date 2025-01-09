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
    # generate random JWT secret
    if [ ! -f .jwt_secret ]; then
        openssl rand -base64 32 > .jwt_secret
    fi
    export JWT_SECRET=$(cat .jwt_secret)

    # OpenAI API config
    export OPENAI_API_KEY="your_key"

    # IP and port
    export SERVER_HOST="0.0.0.0"
    export SERVER_PORT="3000"

    # file upload config
    export UPLOAD_DIR="uploads"
    export MAX_FILE_SIZE="10485760"  # 10MB in bytes

    # create necessary directory
    mkdir -p $UPLOAD_DIR

    # add .jwt_secret to .gitignore
    if ! grep -q "^.jwt_secret$" .gitignore 2>/dev/null; then
        echo ".jwt_secret" >> .gitignore
    fi

    # check if it is development environment
    if [ "$1" == "dev" ]; then
        echo "Starting in development mode..."
        cargo watch -x run
    else
        echo "Starting in production mode..."
        cargo run --release
    fi 
    ;;
esac