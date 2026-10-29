@echo off
setlocal

echo ===================================================
echo WordCraft Android APK Build Script
echo ===================================================

where cargo-apk >nul 2>&1
if %ERRORLEVEL% equ 0 (
    echo [1/2] Building APK using cargo-apk...
    cargo apk build --release -p wordcraft-android
    if %ERRORLEVEL% equ 0 (
        echo [OK] APK successfully built!
        echo Location: target\release\apk\WordCraft.apk
        exit /b 0
    ) else (
        echo [WARN] cargo-apk failed. Trying Gradle build...
    )
) else (
    echo [INFO] cargo-apk not detected in PATH.
    echo To install cargo-apk: cargo install cargo-apk
)

echo [2/2] Building using Gradle...
cd packaging\android
call gradlew.bat assembleRelease
if %ERRORLEVEL% equ 0 (
    echo [OK] Gradle APK successfully built!
    echo Location: packaging\android\app\build\outputs\apk\release\app-release.apk
) else (
    echo [ERROR] Gradle build failed. Ensure Android SDK and NDK are configured.
    exit /b 1
)

cd ..\..
