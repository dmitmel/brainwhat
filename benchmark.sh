#!/usr/bin/env bash

# prints text to stderr
log() { echo "$@" >&2; }
# prints an info message
info() { log "$(tput setaf 6)# $1$(tput sgr0)"; }
# prints an error message
error() { log "$(tput setaf 1)$(tput bold)error:$(tput sgr0) $1"; }

# set error handler
trap 'error "exiting with code $?"; exit $?' ERR

# print command and then execute it
cmd_echo() {
  log "$(tput setaf 8)\$ $*$(tput sgr0)"
  "$@"
}

command -v hyperfine &>/dev/null || {
  error "'$(tput bold)hyperfine$(tput sgr0)' is not installed"
  error "run '$(tput bold)cargo install hyperfine$(tput sgr0)' to install it"
  exit 1
}

TMP_DIR=".benchmark"

[[ -d "$TMP_DIR" ]] || cmd_echo mkdir "$TMP_DIR"
[[ -d "$TMP_DIR/bin" ]] || cmd_echo mkdir "$TMP_DIR/bin"

info "building 'brainwhat'"
cmd_echo cargo build --release
cmd_echo cp target/release/brainwhat "$TMP_DIR/bin/brainwhat"

cmd_echo cd "$TMP_DIR"

if [[ ! -f bin/bff ]]; then
  if [[ ! -d bff ]]; then
    info "downloading 'bff'"
    cmd_echo git clone https://github.com/apankrat/bff
  fi

  info "building 'bff'"
  cmd_echo cc bff/bff.c -o bin/bff -O3 -ansi -DNDEBUG
fi

if [[ ! (-f bin/bff4 && -f bin/bff4lnr) ]]; then
  if [[ ! -f bff4.c ]]; then
    info "downloading 'bff4'"
    cmd_echo curl http://mazonka.com/brainf/bff4.c --remote-name
  fi

  info "building 'bff4'"
  [[ -f bff4 ]] || cmd_echo cc bff4.c -o bin/bff4 -O3 -ansi -DNDEBUG -DNOLNR
  [[ -f bff4lnr ]] || cmd_echo cc bff4.c -o bin/bff4lnr -O3 -ansi -DNDEBUG
fi

bench_level="${1:-0}"
case "$bench_level" in
  0) bf_si_input=",.,.,.,.,.,.,.,.,." ;;
  1) bf_si_input=",.++++,.++,.++++[-],.++,.+++[-],.+++[-],.+++[-],.,." ;;
  2) bf_si_input=",.++++,.++[->+++<],.++++[-],.++,.+++[-],.+++[-],.+++[-],.,." ;;
  *) error "unknown benchmark level: $bench_level"; exit 1 ;;
esac

# 'si' stands for 'self interpreter'
bf_si="../programs/dbfi.b"

cp $bf_si bench_prog
echo "$(<$bf_si)!${bf_si_input}!hello123" > bench_input

echo "$(<$bf_si)!$(<$bf_si)!${bf_si_input}!hello123" > bench_bff4_input

benchmarks=(
  "bin/bff bench_prog < bench_input"
  "bin/bff4 < bench_bff4_input"
  # "bff4/bff4lnr bench_prog < bench_input"
  "bin/brainwhat bench_prog < bench_input"
)

info "runing benchmarks"
cmd_echo hyperfine "${benchmarks[@]}"
log "$(tput setaf 2)# done!$(tput sgr0)"
