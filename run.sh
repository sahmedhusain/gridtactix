#!/bin/bash

# Configuration Variables
CONTAINER="filler-bot"
BOT="solution/target/release/filler"
ENGINE="./linux_game_engine"

function print_help() {
    echo "================================================="
    echo " Filler Project Runing Script "
    echo "================================================="
    echo "Usage: ./run.sh [command] [args...]"
    echo ""
    echo "Commands:"
    echo "  docker        - Stops, rebuilds, and starts the Docker container"
    echo "  cargo [args]  - Runs cargo commands inside the Docker container "
    echo "                  (Examples: ./run.sh cargo check, ./run.sh cargo clippy)"
    echo "  build         - Shortcut to compile the bot in release mode"
    echo "  test          - Shortcut to run 'cargo test' inside the container"
    echo "  audit [m1]    - Runs the full sequence of matches specified in audit.md"
    echo "                  (Pass 'm1' as an argument to run natively bypassing Docker)"
    echo "  shell         - Drops you into the interactive bash shell inside the container"
    echo "================================================="
}

function run_docker() {
    echo "🐳 Restarting the Docker environment..."
    docker compose down
    docker compose up -d
    echo "✅ Docker container '$CONTAINER' is now running in the background."
}

function run_cargo() {
    if [ ! "$(docker ps -q -f name=$CONTAINER)" ]; then
        echo "❌ Error: Docker container is not running. Please run './run.sh docker' first."
        exit 1
    fi
    echo "🦀 Running cargo $@..."
    docker exec -it $CONTAINER bash -c "cd solution && cargo $@"
}

function run_match() {
    local map=$1
    local opp=$2
    local arch=$3
    local total_runs=5
    local wins=0

    local engine="./linux_game_engine"
    local robots_dir="linux_robots"
    local use_docker=1

    if [ "$arch" == "m1" ]; then
        engine="./m1_game_engine"
        robots_dir="m1_robots"
        use_docker=0
    fi

    # Ensure engine and bots are executable before starting matches
    docker exec $CONTAINER bash -c "chmod +x $engine $robots_dir/* 2>/dev/null" || true

    echo "================================================="
    echo " ⚔️  Audit Test: Map $map vs $opp ($total_runs matches) "
    echo "================================================="

    for (( i=1; i<=$total_runs; i++ ))
    do
        echo -n "--> Match $i of $total_runs... "
        
        local is_p1=1
        if [ $((i % 2)) -eq 0 ]; then
            is_p1=0
        fi

        local cmd=""
        if [ $is_p1 -eq 1 ]; then
            # We are Player 1
            cmd="$engine -f maps/$map -p1 $BOT -p2 $robots_dir/$opp -q"
        else
            # We are Player 2
            cmd="$engine -f maps/$map -p1 $robots_dir/$opp -p2 $BOT -q"
        fi

        local output=""
        output=$(docker exec $CONTAINER bash -c "$cmd" 2>&1)

        # Parse scores correctly based on output 'Player1 ( path ): 180'
        local p1_score=$(echo "$output" | awk '/^Player1 \(/ {print $NF}')
        local p2_score=$(echo "$output" | awk '/^Player2 \(/ {print $NF}')
        
        # Fallback if no score parsed
        if [ -z "$p1_score" ]; then p1_score=0; fi
        if [ -z "$p2_score" ]; then p2_score=0; fi

        local won=0
        if [ $is_p1 -eq 1 ] && [ "$p1_score" -gt "$p2_score" ]; then won=1; fi
        if [ $is_p1 -eq 0 ] && [ "$p2_score" -gt "$p1_score" ]; then won=1; fi

        if [ $won -eq 1 ]; then
            ((wins++))
            if [ $is_p1 -eq 1 ]; then echo "WON (Us/P1: $p1_score | Them/P2: $p2_score)"; else echo "WON (Us/P2: $p2_score | Them/P1: $p1_score)"; fi
        else
            if [ $is_p1 -eq 1 ]; then echo "LOST (Us/P1: $p1_score | Them/P2: $p2_score)"; else echo "LOST (Us/P2: $p2_score | Them/P1: $p1_score)"; fi
            if [ "$p1_score" -eq 0 ] && [ "$p2_score" -eq 0 ]; then
                echo -e "\033[0;33m⚠️  Engine Output Dump:\033[0m"
                echo "$output" | tail -n 6
            fi
        fi
        
        sleep 0.5
    done

    echo "-------------------------------------------------"
    if [ $wins -ge 4 ]; then
        echo -e "\033[0;32m✅ PASSED: Won $wins / $total_runs matches against $opp.\033[0m"
    else
        echo -e "\033[0;31m❌ FAILED: Won $wins / $total_runs matches against $opp.\033[0m"
    fi
    echo ""
}

function run_audit() {
    local arch=$1

    if [ ! "$(docker ps -q -f name=$CONTAINER)" ]; then
        echo "❌ Error: Docker container is not running. Please run './run.sh docker' first."
        exit 1
    fi

    # Intelligent Architecture Checking
    local container_arch=$(docker exec $CONTAINER uname -m | tr -d '\r')
    if [ "$arch" == "m1" ] && [ "$container_arch" != "aarch64" ]; then
        echo -e "\033[0;31m❌ Architecture Mismatch Error!\033[0m"
        echo "You requested the 'm1' execution, which requires the native ARM (aarch64) container."
        echo "However, your Docker container is currently cached as: $container_arch"
        echo ""
        echo "To flush the x86 cache and build the genuine ARM environment, copy and paste this:"
        echo -e "\033[0;33mdocker compose down\033[0m"
        echo -e "\033[0;33mdocker pull --platform linux/arm64 rust:1.63-buster\033[0m"
        echo -e "\033[0;33mdocker compose build --no-cache\033[0m"
        echo -e "\033[0;33mdocker compose up -d\033[0m"
        exit 1
    elif [ "$arch" != "m1" ] && [ "$container_arch" != "x86_64" ]; then
        echo -e "\033[0;31m❌ Architecture Mismatch Error!\033[0m"
        echo "You requested standard execution, which requires the Intel (x86_64) container."
        echo "However, your Docker container is currently cached as: $container_arch"
        echo ""
        echo "To revert your environment back to x86_64 (Intel), copy and paste this:"
        echo -e "\033[0;33mdocker compose down\033[0m"
        echo -e "\033[0;33mdocker pull --platform linux/amd64 rust:1.63-buster\033[0m"
        echo -e "\033[0;33mdocker compose build --no-cache\033[0m"
        echo -e "\033[0;33mdocker compose up -d\033[0m"
        exit 1
    fi

    echo "🚀 Building Release Bot in DOCKER before starting audit matches..."
    docker exec -it $CONTAINER bash -c "cd solution && cargo build --release"
    
    echo "🔥 Starting Audit Sequence (Quiet mode enabled to reduce spam)..."
    run_match map00 wall_e "$arch"
    run_match map01 h2_d2 "$arch"
    run_match map02 bender "$arch"
    
    # Bonus Map
    run_match map01 terminator "$arch"
    
    echo "✅ All Audit matches completed!"
}

case "$1" in
    docker)
        run_docker
        ;;
    cargo)
        shift # Remove 'cargo' from args
        run_cargo "$@"
        ;;
    build)
        run_cargo build --release
        ;;
    test)
        run_cargo test
        ;;
    audit)
        run_audit "$2"
        ;;
    shell)
        docker exec -it $CONTAINER bash
        ;;
    *)
        print_help
        ;;
esac
