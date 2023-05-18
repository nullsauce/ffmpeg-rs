## Dependencies

On *nix systems, `clang`, `pkg-config` and FFmpeg libraries (including development headers) are required.

On macOS:

    brew install pkg-config ffmpeg

On Debian-based systems:

    apt install -y clang libavcodec-dev libavformat-dev libavutil-dev pkg-config

Other `libav*-dev` and `libsw*-dev` packages may be required if you enable the corresponding features, e.g., `libavdevice-dev` for the `device` feature.

## Feature selection

At the moment **`codec` and `format`** are mandatory features, or `ffmpeg-sys-next` won't build.

## Building on Windows

### MSVC toolchain

- Install LLVM (through official installer, Visual Studio, Chocolatey, or any other means), and add LLVM's `bin` path to `PATH`, or set `LIBCLANG_PATH` to that (see [`clang-sys` documentation](https://github.com/KyleMayes/clang-sys#environment-variables) for additional info).
- Install FFmpeg (complete with headers) through any means, e.g. downloading a pre-built "full_build-shared" version from https://ffmpeg.org/download.html. Set `FFMPEG_DIR` to the directory containing `include` and `lib`.
- `cargo build`.

You can find an example in https://github.com/zmwangx/rust-ffmpeg/blob/master/.github/workflows/build.yml.

### GNU toolchain

It works with GNU toolchain(haven't checked with MSVC), so you should:
1. Install MSYS2
2. In `.cargo/config` add

   ```toml
   [target.x86_64-pc-windows-gnu]
   linker = "C:\\msys64\\mingw64\\bin\\gcc.exe
   ```

3. Install these packages: `pacman -S mingw-w64-x86_64-toolchain mingw-w64-x86_64-ffmpeg mingw-w64-x86_64-clang`
4. Add `C:\msys64\mingw64\bin` to your `PATH` environment variable


## Building on Raspberry Pi

To build against an FFmpeg with rpi-specific patches (tell: `rpi` can be found in `ffmpeg -hwaccels`), the `rpi` feature needs to be enabled.