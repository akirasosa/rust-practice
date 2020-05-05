#!/usr/bin/env bash

#cp src/bin/template.rs "src/bin/$1.rs"
cp src/bin/template-v1.rs "src/bin/$1.rs"
sed -e "s/xxxxx/$1/g" tests/template.rs > "tests/$1.rs"
