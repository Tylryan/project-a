#!/usr/bin/bash



rm -rf ./test/*
cargo r add deck russian
cargo r add deck russian-verbs
cargo r add deck russian-nouns
cargo r add deck russian-phrases

cargo r add card hello::there russian
cargo r add card hi::there russian

cargo r add card hello::there russian-verbs
cargo r add card hi::there russian-verbs

cargo r add card hello::there russian-nouns
cargo r add card hi::there russian-nouns

cargo r add card hello::there russian-phrases
cargo r add card hi::there russian-phrases
