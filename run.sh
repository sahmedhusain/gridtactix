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
    echo "  audit         - Runs the full sequence of matches specified in audit.md"
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
    local total_runs=5
    local wins=0

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

        local output=""
        if [ $is_p1 -eq 1 ]; then
            # We are Player 1
            output=$(docker exec $CONTAINER bash -c "$ENGINE -f maps/$map -p1 $BOT -p2 linux_robots/$opp -q" 2>&1)
        else
            # We are Player 2
            output=$(docker exec $CONTAINER bash -c "$ENGINE -f maps/$map -p1 linux_robots/$opp -p2 $BOT -q" 2>&1)
        fi

        # Parse scores
        local p1_score=$(echo "$output" | grep -oE "== player1: [0-9]+" | awk '{print $3}')
        local p2_score=$(echo "$output" | grep -oE "== player2: [0-9]+" | awk '{print $3}')
        
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
    if [ ! "$(docker ps -q -f name=$CONTAINER)" ]; then
        echo "❌ Error: Docker container is not running. Please run './run.sh docker' first."
        exit 1
    fi
    
    echo "🚀 Building Release Bot before starting audit matches..."
    docker exec -it $CONTAINER bash -c "cd solution && cargo build --release"
    
    echo "🔥 Starting Audit Sequence (Quiet mode enabled to reduce spam)..."
    run_match map00 wall_e
    run_match map01 h2_d2
    run_match map02 bender
    
    # Bonus Map
    run_match map01 terminator
    
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
        run_audit
        ;;
    shell)
        docker exec -it $CONTAINER bash
        ;;
    *)
        print_help
        ;;
esac
