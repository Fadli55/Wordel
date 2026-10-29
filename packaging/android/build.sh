#!/usr/bin/env bash
set -euo pipefail

echo "==================================================="
echo "WordCraft Android APK Build Script"
echo "==================================================="

if command -v cargo-apk &> /dev/null; then
    echo "[1/2] Building APK using cargo-apk..."
    if cargo apk build --release -p wordcraft-android; then
        echo "[OK] APK successfully built!"
        echo "Location: target/release/apk/WordCraft.apk"
        exit 0
    else
        echo "[WARN] cargo-apk failed. Trying Gradle build..."
    fi
else
    echo "[INFO] cargo-apk not found in PATH."
    echo "Install via: cargo install cargo-apk"
fi

echo "[2/2] Building using Gradle..."
(
    cd packaging/android
    ./gradlew assembleRelease
    echo "[OK] Gradle APK successfully built!"
    echo "Location: packaging/android/app/build/outputs/apk/release/app-release.apk"
)
