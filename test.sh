#!/usr/bin/env bash

set -e

echo "Feature is defined for library. If the macro definition is resolved in the library, the feature will be found"
cargo run

echo
echo
echo "Feature is defined for app. If the macro definition is resolved in the app, the feature will be found"
cargo run --features my_feature
