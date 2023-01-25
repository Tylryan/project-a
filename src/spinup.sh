#!/usr/bin/bash



rm -rf ./test/*
cargo r add deck russian
cargo r add deck russian-verbs

cargo r add card hi::привет russian
cargo r add card hello::there russian
cargo r add card vi::there russian
cargo r add card zi::there russian
cargo r add card sky::there russian
cargo r add card lie::there russian
cargo r add card tie::there russian
cargo r add card rye::there russian

cargo r add card hello::there russian-verbs
cargo r add card hi::there russian-verbs

