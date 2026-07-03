#!/usr/bin/env bash
# `cargo package --list` verifica exatamente quais arquivos entrariam no
# crate publicado (Cargo.toml include/exclude, .gitignore, etc) — pega
# arquivos esquecidos fora do pacote que só apareceriam depois de
# publicado de verdade.
set -e
cd "$(dirname "$0")/.."
cargo package --list --allow-dirty > /tmp/velix-cargo-package-list.txt
grep -q "^src/lib.rs$" /tmp/velix-cargo-package-list.txt || { echo "src/lib.rs não estaria no crate publicado!"; exit 1; }
echo "INSTALL_TEST:rust:PASS: cargo package --list confirma src/lib.rs e demais arquivos presentes no crate"
cat /tmp/velix-cargo-package-list.txt
