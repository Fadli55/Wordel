# WordCraft for Android

WordCraft on Android runs natively via **Rust**, **eframe** (egui), **android-activity (GameActivity)**, and **WGPU** (Vulkan / OpenGL ES backend).

It brings the full clean-room desktop word processor engine (`crates/engine`, `crates/layout`, `crates/docx`, `crates/ui-egui`) to Android devices with hardware-accelerated rendering and touch support.

---

## Architecture Overview

- **Crate:** [`apps/wordcraft-android`](../../apps/wordcraft-android)
- **Library Output:** `libwordcraft_android.so` (`cdylib`)
- **Native Activity:** Android `GameActivity` / `android-activity`
- **Graphics Pipeline:** WGPU -> Android Native Window (Vulkan on Android 10+, GLES fallback)
- **UI Framework:** `egui` with ribbon interface, canvas scaling, and touch interaction
- **File System:** Android internal sandbox + external storage integration

---

## Prerequisites

1. **Rust Toolchain:**
   Ensure Rust is installed (via [rustup.rs](https://rustup.rs/)).

2. **Android Targets:**
   Add the Android architectures you want to compile for:
   ```bash
   # 64-bit ARM (Standard for modern phones/tablets)
   rustup target add aarch64-linux-android

   # 32-bit ARM (Older devices)
   rustup target add armv7-linux-androideabi

   # 64-bit x86 (Android Emulator on PC)
   rustup target add x86_64-linux-android
   ```

3. **Android SDK & NDK:**
   - Install Android Studio or Android Command Line Tools with Android SDK Platform 34 and NDK (version 25 or newer).
   - Set environment variables:
     - `ANDROID_HOME`: path to your Android SDK (e.g. `%LOCALAPPDATA%\Android\Sdk` on Windows)
     - `NDK_HOME` or `ANDROID_NDK_HOME`: path to your NDK (e.g. `%ANDROID_HOME%\ndk\<version>`)

---

## Build Methods

### Method 1: Using `cargo-apk` (Recommended CLI)

The easiest and fastest way to build an APK directly from Cargo without needing a full Gradle build:

1. Install `cargo-apk`:
   ```bash
   cargo install cargo-apk
   ```

2. Build debug APK:
   ```bash
   cargo apk build -p wordcraft-android
   ```

3. Build release APK:
   ```bash
   cargo apk build --release -p wordcraft-android
   ```

The compiled APK will be at:
```
target/release/apk/WordCraft.apk
```

---

### Method 2: Using Android Studio & Gradle

The repository provides a complete Android Studio project under `packaging/android`:

1. Open `packaging/android` in **Android Studio**.
2. Sync Gradle dependencies.
3. Or build via command line:
   ```bash
   # Windows
   packaging\android\build.bat

   # Linux / macOS
   ./packaging/android/build.sh
   ```

The generated APK will be at:
```
packaging/android/app/build/outputs/apk/release/app-release.apk
```

---

## Installing & Running on Device / Emulator

1. Connect your Android phone via USB (with USB Debugging enabled) or start an Android Emulator.
2. Verify device connection:
   ```bash
   adb devices
   ```
3. Install the APK:
   ```bash
   adb install -r target/release/apk/WordCraft.apk
   ```
4. Launch WordCraft from your home screen or via `adb`:
   ```bash
   adb shell am start -n ai.storyteller.wordcraft/.MainActivity
   ```

---

## Viewing Logs & Diagnostics

WordCraft initializes Android Logcat logging under the tag `WordCraft`:

```bash
adb logcat -s WordCraft:I rust:I eframe:I wgpu:I
```

---

## Document Association

WordCraft's `AndroidManifest.xml` registers file handlers for:
- `.docx` (Office Open XML Document)
- `.doc` (Legacy Word binary document)
- `.odt` (OpenDocument Text)
- `.rtf` (Rich Text Format)
- `.txt` (Plain text)

Tapping on any document file in Android's Files / Download manager will prompt to open directly in WordCraft.
