#!/sbin/fish

trap 'kill 0' SIGINT;
cargo watch -w shaders/ -s "fish ./compile_shaders.fish" --use-shell=bash &
cargo watch -w src/ -w compiled/ -x run 
