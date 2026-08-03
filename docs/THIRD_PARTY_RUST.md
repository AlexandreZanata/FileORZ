# Third-party Rust crates

License inventory for the FileORZ Cargo workspace (transitive deps).
FileORZ itself is **GPL-3.0-or-later** — see [LICENSE](../LICENSE) and
[THIRD_PARTY_NOTICES.md](../THIRD_PARTY_NOTICES.md).

Regenerate:

```bash
python3 scripts/gen_third_party_rust.py
# or: cargo about generate  (if cargo-about is installed)
```

Lockfile packages with `source`: **518**
(registry lookup misses, often macOS-only: **2**).

## Summary by license expression

| Count | License |
|------:|---------|
| 220 | `MIT OR Apache-2.0` |
| 137 | `MIT` |
| 39 | `Apache-2.0 OR MIT` |
| 26 | `MIT/Apache-2.0` |
| 18 | `Unicode-3.0` |
| 14 | `Apache-2.0` |
| 8 | `Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT` |
| 8 | `Unlicense OR MIT` |
| 8 | `Zlib OR Apache-2.0 OR MIT` |
| 7 | `MIT OR Apache-2.0 OR Zlib` |
| 6 | `Apache-2.0/MIT` |
| 3 | `BSD-3-Clause` |
| 2 | `BSD-2-Clause OR Apache-2.0 OR MIT` |
| 2 | `BSD-3-Clause OR MIT OR Apache-2.0` |
| 2 | `BSL-1.0` |
| 2 | `ISC` |
| 2 | `MISSING-FROM-REGISTRY` |
| 2 | `MIT OR Apache-2.0 OR LGPL-2.1-or-later` |
| 2 | `Unlicense/MIT` |
| 1 | `(Apache-2.0 OR MIT) AND BSD-3-Clause` |
| 1 | `(MIT OR Apache-2.0) AND Unicode-3.0` |
| 1 | `0BSD OR MIT OR Apache-2.0` |
| 1 | `Apache-2.0 AND MIT` |
| 1 | `Apache-2.0 OR GPL-2.0-only` |
| 1 | `BSD-2-Clause` |
| 1 | `CC0-1.0` |
| 1 | `MIT OR Zlib OR Apache-2.0` |
| 1 | `Unlicense` |
| 1 | `Zlib` |

## Full list

| Crate | Version | License |
|-------|---------|---------|
| `ab_glyph` | 0.2.32 | `Apache-2.0` |
| `ab_glyph_rasterizer` | 0.1.10 | `Apache-2.0` |
| `adler2` | 2.0.1 | `0BSD OR MIT OR Apache-2.0` |
| `aes` | 0.8.4 | `MIT OR Apache-2.0` |
| `ahash` | 0.7.8 | `MIT OR Apache-2.0` |
| `ahash` | 0.8.12 | `MIT OR Apache-2.0` |
| `allocator-api2` | 0.2.21 | `MIT OR Apache-2.0` |
| `android-activity` | 0.6.1 | `MIT OR Apache-2.0` |
| `android-properties` | 0.2.2 | `MIT` |
| `android_system_properties` | 0.1.5 | `MIT/Apache-2.0` |
| `approx` | 0.5.1 | `Apache-2.0` |
| `arrayref` | 0.3.9 | `BSD-2-Clause` |
| `arrayvec` | 0.7.8 | `MIT OR Apache-2.0` |
| `as-raw-xcb-connection` | 1.0.1 | `MIT OR Apache-2.0` |
| `ash` | 0.37.3+1.3.251 | `MIT OR Apache-2.0` |
| `ashpd` | 0.11.1 | `MIT` |
| `async-broadcast` | 0.7.2 | `MIT OR Apache-2.0` |
| `async-channel` | 2.5.0 | `Apache-2.0 OR MIT` |
| `async-executor` | 1.14.0 | `Apache-2.0 OR MIT` |
| `async-fs` | 2.2.0 | `Apache-2.0 OR MIT` |
| `async-io` | 2.6.0 | `Apache-2.0 OR MIT` |
| `async-lock` | 3.4.2 | `Apache-2.0 OR MIT` |
| `async-net` | 2.0.0 | `Apache-2.0 OR MIT` |
| `async-process` | 2.5.0 | `Apache-2.0 OR MIT` |
| `async-recursion` | 1.1.1 | `MIT OR Apache-2.0` |
| `async-signal` | 0.2.14 | `Apache-2.0 OR MIT` |
| `async-task` | 4.7.1 | `Apache-2.0 OR MIT` |
| `async-trait` | 0.1.91 | `MIT OR Apache-2.0` |
| `atomic-waker` | 1.1.2 | `Apache-2.0 OR MIT` |
| `autocfg` | 1.5.1 | `Apache-2.0 OR MIT` |
| `bit-set` | 0.5.3 | `MIT/Apache-2.0` |
| `bit-vec` | 0.6.3 | `MIT/Apache-2.0` |
| `bitflags` | 1.3.2 | `MIT/Apache-2.0` |
| `bitflags` | 2.13.1 | `MIT OR Apache-2.0` |
| `block` | 0.1.6 | `MIT` |
| `block-buffer` | 0.10.4 | `MIT OR Apache-2.0` |
| `block-padding` | 0.3.3 | `MIT OR Apache-2.0` |
| `block2` | 0.5.1 | `MIT` |
| `block2` | 0.6.2 | `MISSING-FROM-REGISTRY` |
| `blocking` | 1.6.2 | `Apache-2.0 OR MIT` |
| `bumpalo` | 3.20.3 | `MIT OR Apache-2.0` |
| `by_address` | 1.2.1 | `MIT OR Apache-2.0` |
| `bytecount` | 0.6.9 | `Apache-2.0/MIT` |
| `bytemuck` | 1.25.2 | `Zlib OR Apache-2.0 OR MIT` |
| `bytemuck_derive` | 1.11.0 | `Zlib OR Apache-2.0 OR MIT` |
| `bytes` | 1.12.1 | `MIT` |
| `calloop` | 0.13.0 | `MIT` |
| `calloop` | 0.14.4 | `MIT` |
| `calloop-wayland-source` | 0.3.0 | `MIT` |
| `calloop-wayland-source` | 0.4.1 | `MIT` |
| `cbc` | 0.1.2 | `MIT OR Apache-2.0` |
| `cc` | 1.4.0 | `MIT OR Apache-2.0` |
| `cfg-if` | 1.0.4 | `MIT OR Apache-2.0` |
| `cfg_aliases` | 0.1.1 | `MIT` |
| `cfg_aliases` | 0.2.2 | `MIT` |
| `chrono` | 0.4.45 | `MIT OR Apache-2.0` |
| `cipher` | 0.4.4 | `MIT OR Apache-2.0` |
| `clipboard-win` | 5.4.1 | `BSL-1.0` |
| `clipboard_macos` | 0.1.1 | `Apache-2.0` |
| `clipboard_wayland` | 0.2.2 | `Apache-2.0` |
| `clipboard_x11` | 0.4.3 | `MIT` |
| `codespan-reporting` | 0.11.1 | `Apache-2.0` |
| `com` | 0.6.0 | `MIT` |
| `com_macros` | 0.6.0 | `MIT` |
| `com_macros_support` | 0.6.0 | `MIT` |
| `combine` | 4.6.7 | `MIT` |
| `concurrent-queue` | 2.5.0 | `Apache-2.0 OR MIT` |
| `core-foundation` | 0.9.4 | `MIT OR Apache-2.0` |
| `core-foundation-sys` | 0.8.7 | `MIT OR Apache-2.0` |
| `core-graphics` | 0.23.2 | `MIT OR Apache-2.0` |
| `core-graphics-types` | 0.1.3 | `MIT OR Apache-2.0` |
| `cosmic-text` | 0.12.1 | `MIT OR Apache-2.0` |
| `cpufeatures` | 0.2.17 | `MIT OR Apache-2.0` |
| `crc32fast` | 1.5.0 | `MIT OR Apache-2.0` |
| `crossbeam-deque` | 0.8.7 | `MIT OR Apache-2.0` |
| `crossbeam-epoch` | 0.9.20 | `MIT OR Apache-2.0` |
| `crossbeam-utils` | 0.8.22 | `MIT OR Apache-2.0` |
| `crunchy` | 0.2.4 | `MIT` |
| `crypto-common` | 0.1.7 | `MIT OR Apache-2.0` |
| `ctor` | 0.10.1 | `Apache-2.0 OR MIT` |
| `cursor-icon` | 1.2.0 | `MIT OR Apache-2.0 OR Zlib` |
| `d3d12` | 0.19.0 | `MIT OR Apache-2.0` |
| `dark-light` | 1.1.1 | `MIT/Apache-2.0` |
| `dconf_rs` | 0.3.0 | `MIT` |
| `defmt` | 1.1.1 | `MIT OR Apache-2.0` |
| `defmt-macros` | 1.1.1 | `MIT OR Apache-2.0` |
| `defmt-parser` | 1.0.0 | `MIT OR Apache-2.0` |
| `deranged` | 0.5.8 | `MIT OR Apache-2.0` |
| `detect-desktop-environment` | 0.2.0 | `MIT` |
| `digest` | 0.10.7 | `MIT OR Apache-2.0` |
| `dirs` | 4.0.0 | `MIT OR Apache-2.0` |
| `dirs-sys` | 0.3.7 | `MIT OR Apache-2.0` |
| `dispatch` | 0.2.0 | `MIT` |
| `dispatch2` | 0.3.1 | `Zlib OR Apache-2.0 OR MIT` |
| `displaydoc` | 0.2.7 | `MIT OR Apache-2.0` |
| `dlib` | 0.5.3 | `MIT` |
| `dlv-list` | 0.3.0 | `MIT` |
| `downcast-rs` | 1.2.1 | `MIT/Apache-2.0` |
| `dpi` | 0.1.2 | `Apache-2.0 AND MIT` |
| `drm` | 0.14.1 | `MIT` |
| `drm-ffi` | 0.9.1 | `MIT` |
| `drm-fourcc` | 2.2.0 | `MIT` |
| `drm-sys` | 0.8.1 | `MIT` |
| `dtor` | 0.8.1 | `Apache-2.0 OR MIT` |
| `ecb` | 0.1.2 | `MIT` |
| `either` | 1.17.0 | `MIT OR Apache-2.0` |
| `encoding_rs` | 0.8.35 | `(Apache-2.0 OR MIT) AND BSD-3-Clause` |
| `endi` | 1.1.1 | `MIT` |
| `enumflags2` | 0.7.12 | `MIT OR Apache-2.0` |
| `enumflags2_derive` | 0.7.12 | `MIT OR Apache-2.0` |
| `equivalent` | 1.0.2 | `Apache-2.0 OR MIT` |
| `errno` | 0.3.14 | `MIT OR Apache-2.0` |
| `error-code` | 3.3.2 | `BSL-1.0` |
| `etagere` | 0.2.15 | `MIT/Apache-2.0` |
| `euclid` | 0.22.14 | `MIT OR Apache-2.0` |
| `event-listener` | 5.4.2 | `Apache-2.0 OR MIT` |
| `event-listener-strategy` | 0.5.4 | `Apache-2.0 OR MIT` |
| `fastrand` | 2.5.0 | `Apache-2.0 OR MIT` |
| `fdeflate` | 0.3.7 | `MIT OR Apache-2.0` |
| `find-msvc-tools` | 0.1.9 | `MIT OR Apache-2.0` |
| `flate2` | 1.1.9 | `MIT OR Apache-2.0` |
| `fluent` | 0.17.0 | `Apache-2.0 OR MIT` |
| `fluent-bundle` | 0.16.0 | `Apache-2.0 OR MIT` |
| `fluent-langneg` | 0.13.1 | `Apache-2.0 OR MIT` |
| `fluent-syntax` | 0.12.0 | `Apache-2.0 OR MIT` |
| `font-types` | 0.7.3 | `MIT OR Apache-2.0` |
| `fontconfig-parser` | 0.5.8 | `MIT` |
| `fontdb` | 0.16.2 | `MIT` |
| `foreign-types` | 0.5.0 | `MIT/Apache-2.0` |
| `foreign-types-macros` | 0.2.4 | `MIT/Apache-2.0` |
| `foreign-types-shared` | 0.3.1 | `MIT/Apache-2.0` |
| `form_urlencoded` | 1.2.2 | `MIT OR Apache-2.0` |
| `futures` | 0.3.33 | `MIT OR Apache-2.0` |
| `futures-channel` | 0.3.33 | `MIT OR Apache-2.0` |
| `futures-core` | 0.3.33 | `MIT OR Apache-2.0` |
| `futures-executor` | 0.3.33 | `MIT OR Apache-2.0` |
| `futures-io` | 0.3.33 | `MIT OR Apache-2.0` |
| `futures-lite` | 2.6.1 | `Apache-2.0 OR MIT` |
| `futures-macro` | 0.3.33 | `MIT OR Apache-2.0` |
| `futures-sink` | 0.3.33 | `MIT OR Apache-2.0` |
| `futures-task` | 0.3.33 | `MIT OR Apache-2.0` |
| `futures-util` | 0.3.33 | `MIT OR Apache-2.0` |
| `generic-array` | 0.14.7 | `MIT` |
| `gethostname` | 1.1.0 | `Apache-2.0` |
| `getrandom` | 0.2.17 | `MIT OR Apache-2.0` |
| `getrandom` | 0.3.4 | `MIT OR Apache-2.0` |
| `getrandom` | 0.4.3 | `MIT OR Apache-2.0` |
| `gl_generator` | 0.14.0 | `Apache-2.0` |
| `glam` | 0.25.0 | `MIT OR Apache-2.0` |
| `glow` | 0.13.1 | `MIT OR Apache-2.0 OR Zlib` |
| `glutin_wgl_sys` | 0.5.0 | `Apache-2.0` |
| `gpu-alloc` | 0.6.2 | `MIT OR Apache-2.0` |
| `gpu-alloc-types` | 0.3.1 | `MIT OR Apache-2.0` |
| `gpu-allocator` | 0.25.0 | `MIT OR Apache-2.0` |
| `gpu-descriptor` | 0.2.4 | `MIT OR Apache-2.0` |
| `gpu-descriptor-types` | 0.1.2 | `MIT OR Apache-2.0` |
| `guillotiere` | 0.6.2 | `MIT/Apache-2.0` |
| `half` | 2.7.1 | `MIT OR Apache-2.0` |
| `hashbrown` | 0.12.3 | `MIT OR Apache-2.0` |
| `hashbrown` | 0.14.5 | `MIT OR Apache-2.0` |
| `hashbrown` | 0.17.1 | `MIT OR Apache-2.0` |
| `hassle-rs` | 0.11.0 | `MIT` |
| `hermit-abi` | 0.5.2 | `MIT OR Apache-2.0` |
| `hex` | 0.4.3 | `MIT OR Apache-2.0` |
| `hexf-parse` | 0.2.1 | `CC0-1.0` |
| `iana-time-zone` | 0.1.65 | `MIT OR Apache-2.0` |
| `iana-time-zone-haiku` | 0.1.2 | `MIT OR Apache-2.0` |
| `iced` | 0.13.1 | `MIT` |
| `iced_core` | 0.13.2 | `MIT` |
| `iced_futures` | 0.13.2 | `MIT` |
| `iced_glyphon` | 0.6.0 | `MIT OR Apache-2.0 OR Zlib` |
| `iced_graphics` | 0.13.0 | `MIT` |
| `iced_renderer` | 0.13.0 | `MIT` |
| `iced_runtime` | 0.13.2 | `MIT` |
| `iced_tiny_skia` | 0.13.0 | `MIT` |
| `iced_wgpu` | 0.13.5 | `MIT` |
| `iced_widget` | 0.13.4 | `MIT` |
| `iced_winit` | 0.13.0 | `MIT` |
| `icu_collections` | 2.2.0 | `Unicode-3.0` |
| `icu_locale_core` | 2.2.0 | `Unicode-3.0` |
| `icu_normalizer` | 2.2.0 | `Unicode-3.0` |
| `icu_normalizer_data` | 2.2.0 | `Unicode-3.0` |
| `icu_properties` | 2.2.0 | `Unicode-3.0` |
| `icu_properties_data` | 2.2.0 | `Unicode-3.0` |
| `icu_provider` | 2.2.0 | `Unicode-3.0` |
| `idna` | 1.1.0 | `MIT OR Apache-2.0` |
| `idna_adapter` | 1.2.2 | `Apache-2.0 OR MIT` |
| `indexmap` | 2.14.0 | `Apache-2.0 OR MIT` |
| `inout` | 0.1.4 | `MIT OR Apache-2.0` |
| `instant` | 0.1.13 | `BSD-3-Clause` |
| `intl-memoizer` | 0.5.3 | `Apache-2.0 OR MIT` |
| `intl_pluralrules` | 7.0.2 | `Apache-2.0/MIT` |
| `is-docker` | 0.2.0 | `MIT` |
| `is-wsl` | 0.4.0 | `MIT` |
| `itoa` | 1.0.18 | `MIT OR Apache-2.0` |
| `jiff` | 0.2.35 | `Unlicense OR MIT` |
| `jiff-core` | 0.1.0 | `Unlicense OR MIT` |
| `jiff-static` | 0.2.35 | `Unlicense OR MIT` |
| `jiff-tzdb` | 0.1.8 | `Unlicense OR MIT` |
| `jiff-tzdb-platform` | 0.1.3 | `Unlicense OR MIT` |
| `jni` | 0.22.4 | `MIT OR Apache-2.0` |
| `jni-macros` | 0.22.4 | `MIT OR Apache-2.0` |
| `jni-sys` | 0.3.1 | `MIT OR Apache-2.0` |
| `jni-sys` | 0.4.1 | `MIT OR Apache-2.0` |
| `jni-sys-macros` | 0.4.1 | `MIT OR Apache-2.0` |
| `jobserver` | 0.1.35 | `MIT OR Apache-2.0` |
| `js-sys` | 0.3.103 | `MIT OR Apache-2.0` |
| `khronos-egl` | 6.0.0 | `MIT/Apache-2.0` |
| `khronos_api` | 3.1.0 | `Apache-2.0` |
| `ksni` | 0.3.6 | `Unlicense` |
| `kurbo` | 0.10.4 | `MIT OR Apache-2.0` |
| `libc` | 0.2.189 | `MIT OR Apache-2.0` |
| `libloading` | 0.7.4 | `ISC` |
| `libloading` | 0.8.9 | `ISC` |
| `libm` | 0.2.16 | `MIT` |
| `libredox` | 0.1.19 | `MIT` |
| `linux-raw-sys` | 0.12.1 | `Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT` |
| `linux-raw-sys` | 0.4.15 | `Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT` |
| `linux-raw-sys` | 0.9.4 | `Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT` |
| `litemap` | 0.8.2 | `Unicode-3.0` |
| `lock_api` | 0.4.14 | `MIT OR Apache-2.0` |
| `log` | 0.4.33 | `MIT OR Apache-2.0` |
| `lopdf` | 0.39.0 | `MIT` |
| `lru` | 0.12.5 | `MIT` |
| `malloc_buf` | 0.0.6 | `MIT` |
| `md-5` | 0.10.6 | `MIT OR Apache-2.0` |
| `memchr` | 2.8.3 | `Unlicense OR MIT` |
| `memmap2` | 0.9.11 | `MIT OR Apache-2.0` |
| `memoffset` | 0.9.1 | `MIT` |
| `metal` | 0.27.0 | `MIT OR Apache-2.0` |
| `miniz_oxide` | 0.8.9 | `MIT OR Zlib OR Apache-2.0` |
| `mio` | 1.2.2 | `MIT` |
| `naga` | 0.19.2 | `MIT OR Apache-2.0` |
| `ndk` | 0.9.0 | `MIT OR Apache-2.0` |
| `ndk-context` | 0.1.1 | `MIT OR Apache-2.0` |
| `ndk-sys` | 0.5.0+25.2.9519653 | `MIT OR Apache-2.0` |
| `ndk-sys` | 0.6.0+11769913 | `MIT OR Apache-2.0` |
| `nix` | 0.29.0 | `MIT` |
| `nom` | 8.0.0 | `MIT` |
| `nom_locate` | 5.0.0 | `MIT` |
| `num-conv` | 0.2.2 | `MIT OR Apache-2.0` |
| `num-traits` | 0.2.19 | `MIT OR Apache-2.0` |
| `num_enum` | 0.7.6 | `BSD-3-Clause OR MIT OR Apache-2.0` |
| `num_enum_derive` | 0.7.6 | `BSD-3-Clause OR MIT OR Apache-2.0` |
| `objc` | 0.2.7 | `MIT` |
| `objc-sys` | 0.3.5 | `MIT` |
| `objc2` | 0.5.2 | `MIT` |
| `objc2` | 0.6.4 | `MIT` |
| `objc2-app-kit` | 0.2.2 | `MIT` |
| `objc2-app-kit` | 0.3.2 | `MISSING-FROM-REGISTRY` |
| `objc2-cloud-kit` | 0.2.2 | `MIT` |
| `objc2-contacts` | 0.2.2 | `MIT` |
| `objc2-core-data` | 0.2.2 | `MIT` |
| `objc2-core-foundation` | 0.3.2 | `Zlib OR Apache-2.0 OR MIT` |
| `objc2-core-graphics` | 0.3.2 | `Zlib OR Apache-2.0 OR MIT` |
| `objc2-core-image` | 0.2.2 | `MIT` |
| `objc2-core-location` | 0.2.2 | `MIT` |
| `objc2-encode` | 4.1.0 | `MIT` |
| `objc2-foundation` | 0.2.2 | `MIT` |
| `objc2-foundation` | 0.3.2 | `MIT` |
| `objc2-io-surface` | 0.3.2 | `Zlib OR Apache-2.0 OR MIT` |
| `objc2-link-presentation` | 0.2.2 | `MIT` |
| `objc2-metal` | 0.2.2 | `MIT` |
| `objc2-quartz-core` | 0.2.2 | `MIT` |
| `objc2-quartz-core` | 0.3.2 | `Zlib OR Apache-2.0 OR MIT` |
| `objc2-symbols` | 0.2.2 | `MIT` |
| `objc2-ui-kit` | 0.2.2 | `MIT` |
| `objc2-uniform-type-identifiers` | 0.2.2 | `MIT` |
| `objc2-user-notifications` | 0.2.2 | `MIT` |
| `objc_exception` | 0.1.2 | `MIT` |
| `once_cell` | 1.21.4 | `MIT OR Apache-2.0` |
| `open` | 5.4.0 | `MIT` |
| `orbclient` | 0.3.55 | `MIT` |
| `ordered-multimap` | 0.4.3 | `MIT` |
| `ordered-stream` | 0.2.0 | `MIT OR Apache-2.0` |
| `owned_ttf_parser` | 0.25.1 | `Apache-2.0` |
| `palette` | 0.7.7 | `MIT OR Apache-2.0` |
| `palette_derive` | 0.7.7 | `MIT OR Apache-2.0` |
| `palette_math` | 0.7.7 | `MIT OR Apache-2.0` |
| `parking` | 2.2.1 | `Apache-2.0 OR MIT` |
| `parking_lot` | 0.11.2 | `Apache-2.0/MIT` |
| `parking_lot` | 0.12.5 | `MIT OR Apache-2.0` |
| `parking_lot_core` | 0.8.6 | `Apache-2.0/MIT` |
| `parking_lot_core` | 0.9.12 | `MIT OR Apache-2.0` |
| `paste` | 1.0.15 | `MIT OR Apache-2.0` |
| `pastey` | 0.2.3 | `MIT OR Apache-2.0` |
| `percent-encoding` | 2.3.2 | `MIT OR Apache-2.0` |
| `phf` | 0.13.1 | `MIT` |
| `phf_shared` | 0.13.1 | `MIT` |
| `pin-project` | 1.1.13 | `Apache-2.0 OR MIT` |
| `pin-project-internal` | 1.1.13 | `Apache-2.0 OR MIT` |
| `pin-project-lite` | 0.2.17 | `Apache-2.0 OR MIT` |
| `pin-utils` | 0.1.0 | `MIT OR Apache-2.0` |
| `piper` | 0.2.5 | `MIT OR Apache-2.0` |
| `pkg-config` | 0.3.33 | `MIT OR Apache-2.0` |
| `plain` | 0.2.3 | `MIT/Apache-2.0` |
| `png` | 0.17.16 | `MIT OR Apache-2.0` |
| `polling` | 3.11.0 | `Apache-2.0 OR MIT` |
| `pollster` | 0.4.0 | `Apache-2.0/MIT` |
| `portable-atomic` | 1.14.0 | `Apache-2.0 OR MIT` |
| `portable-atomic-util` | 0.2.7 | `Apache-2.0 OR MIT` |
| `potential_utf` | 0.1.5 | `Unicode-3.0` |
| `powerfmt` | 0.2.0 | `MIT OR Apache-2.0` |
| `ppv-lite86` | 0.2.21 | `MIT OR Apache-2.0` |
| `presser` | 0.3.1 | `MIT OR Apache-2.0` |
| `proc-macro-crate` | 3.5.0 | `MIT OR Apache-2.0` |
| `proc-macro2` | 1.0.107 | `MIT OR Apache-2.0` |
| `profiling` | 1.0.18 | `MIT OR Apache-2.0` |
| `quick-xml` | 0.41.0 | `MIT` |
| `quote` | 1.0.47 | `MIT OR Apache-2.0` |
| `r-efi` | 5.3.0 | `MIT OR Apache-2.0 OR LGPL-2.1-or-later` |
| `r-efi` | 6.0.0 | `MIT OR Apache-2.0 OR LGPL-2.1-or-later` |
| `rand` | 0.8.7 | `MIT OR Apache-2.0` |
| `rand` | 0.9.5 | `MIT OR Apache-2.0` |
| `rand_chacha` | 0.3.1 | `MIT OR Apache-2.0` |
| `rand_chacha` | 0.9.0 | `MIT OR Apache-2.0` |
| `rand_core` | 0.6.4 | `MIT OR Apache-2.0` |
| `rand_core` | 0.9.5 | `MIT OR Apache-2.0` |
| `range-alloc` | 0.1.5 | `MIT OR Apache-2.0` |
| `rangemap` | 1.7.1 | `MIT/Apache-2.0` |
| `raw-window-handle` | 0.6.2 | `MIT OR Apache-2.0 OR Zlib` |
| `rayon` | 1.12.0 | `MIT OR Apache-2.0` |
| `rayon-core` | 1.13.0 | `MIT OR Apache-2.0` |
| `read-fonts` | 0.22.7 | `MIT OR Apache-2.0` |
| `redox_syscall` | 0.2.16 | `MIT` |
| `redox_syscall` | 0.4.1 | `MIT` |
| `redox_syscall` | 0.5.18 | `MIT` |
| `redox_syscall` | 0.9.1 | `MIT` |
| `redox_users` | 0.4.6 | `MIT` |
| `renderdoc-sys` | 1.1.0 | `MIT OR Apache-2.0` |
| `rfd` | 0.15.4 | `MIT` |
| `roxmltree` | 0.20.0 | `MIT OR Apache-2.0` |
| `rust-ini` | 0.18.0 | `MIT` |
| `rustc-hash` | 1.1.0 | `Apache-2.0/MIT` |
| `rustc-hash` | 2.1.3 | `Apache-2.0 OR MIT` |
| `rustc_version` | 0.4.1 | `MIT OR Apache-2.0` |
| `rustix` | 0.38.44 | `Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT` |
| `rustix` | 1.1.4 | `Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT` |
| `rustversion` | 1.0.23 | `MIT OR Apache-2.0` |
| `rustybuzz` | 0.14.1 | `MIT` |
| `same-file` | 1.0.6 | `Unlicense/MIT` |
| `scoped-tls` | 1.0.1 | `MIT/Apache-2.0` |
| `scopeguard` | 1.2.0 | `MIT OR Apache-2.0` |
| `sctk-adwaita` | 0.10.1 | `MIT` |
| `self_cell` | 1.3.0 | `Apache-2.0 OR GPL-2.0-only` |
| `semver` | 1.0.28 | `MIT OR Apache-2.0` |
| `serde` | 1.0.229 | `MIT OR Apache-2.0` |
| `serde_core` | 1.0.229 | `MIT OR Apache-2.0` |
| `serde_derive` | 1.0.229 | `MIT OR Apache-2.0` |
| `serde_json` | 1.0.151 | `MIT OR Apache-2.0` |
| `serde_repr` | 0.1.21 | `MIT OR Apache-2.0` |
| `sha1` | 0.10.7 | `MIT OR Apache-2.0` |
| `sha2` | 0.10.9 | `MIT OR Apache-2.0` |
| `shlex` | 2.0.1 | `MIT OR Apache-2.0` |
| `signal-hook-registry` | 1.4.8 | `MIT OR Apache-2.0` |
| `simd-adler32` | 0.3.10 | `MIT` |
| `simd_cesu8` | 1.2.0 | `Apache-2.0 OR MIT` |
| `simdutf8` | 0.1.5 | `MIT OR Apache-2.0` |
| `siphasher` | 1.0.3 | `MIT/Apache-2.0` |
| `skrifa` | 0.22.3 | `MIT OR Apache-2.0` |
| `slab` | 0.4.12 | `MIT` |
| `slotmap` | 1.1.1 | `Zlib` |
| `smallvec` | 1.15.2 | `MIT OR Apache-2.0` |
| `smithay-client-toolkit` | 0.19.2 | `MIT` |
| `smithay-client-toolkit` | 0.20.0 | `MIT` |
| `smithay-clipboard` | 0.7.3 | `MIT` |
| `smol_str` | 0.2.2 | `MIT OR Apache-2.0` |
| `socket2` | 0.6.5 | `MIT OR Apache-2.0` |
| `softbuffer` | 0.4.8 | `MIT OR Apache-2.0` |
| `spirv` | 0.3.0+sdk-1.3.268.0 | `Apache-2.0` |
| `stable_deref_trait` | 1.2.1 | `MIT OR Apache-2.0` |
| `static_assertions` | 1.1.0 | `MIT OR Apache-2.0` |
| `strict-num` | 0.1.1 | `MIT` |
| `stringprep` | 0.1.5 | `MIT/Apache-2.0` |
| `svg_fmt` | 0.4.5 | `MIT/Apache-2.0` |
| `swash` | 0.1.19 | `Apache-2.0 OR MIT` |
| `syn` | 1.0.109 | `MIT OR Apache-2.0` |
| `syn` | 2.0.119 | `MIT OR Apache-2.0` |
| `syn` | 3.0.3 | `MIT OR Apache-2.0` |
| `synstructure` | 0.13.2 | `MIT` |
| `sys-locale` | 0.3.2 | `MIT OR Apache-2.0` |
| `tempfile` | 3.27.0 | `MIT OR Apache-2.0` |
| `termcolor` | 1.4.1 | `Unlicense OR MIT` |
| `thiserror` | 1.0.69 | `MIT OR Apache-2.0` |
| `thiserror` | 2.0.19 | `MIT OR Apache-2.0` |
| `thiserror-impl` | 1.0.69 | `MIT OR Apache-2.0` |
| `thiserror-impl` | 2.0.19 | `MIT OR Apache-2.0` |
| `time` | 0.3.55 | `MIT OR Apache-2.0` |
| `time-core` | 0.1.9 | `MIT OR Apache-2.0` |
| `time-macros` | 0.2.32 | `MIT OR Apache-2.0` |
| `tiny-skia` | 0.11.4 | `BSD-3-Clause` |
| `tiny-skia-path` | 0.11.4 | `BSD-3-Clause` |
| `tiny-xlib` | 0.2.5 | `MIT OR Apache-2.0 OR Zlib` |
| `tinystr` | 0.8.3 | `Unicode-3.0` |
| `tinyvec` | 1.12.0 | `Zlib OR Apache-2.0 OR MIT` |
| `tinyvec_macros` | 0.1.1 | `MIT OR Apache-2.0 OR Zlib` |
| `tokio` | 1.53.1 | `MIT` |
| `tokio-macros` | 2.7.2 | `MIT` |
| `toml_datetime` | 1.1.1+spec-1.1.0 | `MIT OR Apache-2.0` |
| `toml_edit` | 0.25.13+spec-1.1.0 | `MIT OR Apache-2.0` |
| `toml_parser` | 1.1.3+spec-1.1.0 | `MIT OR Apache-2.0` |
| `tracing` | 0.1.44 | `MIT` |
| `tracing-attributes` | 0.1.31 | `MIT` |
| `tracing-core` | 0.1.36 | `MIT` |
| `ttf-parser` | 0.20.0 | `MIT OR Apache-2.0` |
| `ttf-parser` | 0.21.1 | `MIT OR Apache-2.0` |
| `ttf-parser` | 0.25.1 | `MIT OR Apache-2.0` |
| `type-map` | 0.5.1 | `MIT/Apache-2.0` |
| `typenum` | 1.20.1 | `MIT OR Apache-2.0` |
| `uds_windows` | 1.2.1 | `MIT` |
| `unic-langid` | 0.9.6 | `MIT OR Apache-2.0` |
| `unic-langid-impl` | 0.9.6 | `MIT OR Apache-2.0` |
| `unicode-bidi` | 0.3.18 | `MIT OR Apache-2.0` |
| `unicode-bidi-mirroring` | 0.2.0 | `MIT/Apache-2.0` |
| `unicode-ccc` | 0.2.0 | `MIT/Apache-2.0` |
| `unicode-ident` | 1.0.24 | `(MIT OR Apache-2.0) AND Unicode-3.0` |
| `unicode-linebreak` | 0.1.5 | `Apache-2.0` |
| `unicode-normalization` | 0.1.25 | `MIT OR Apache-2.0` |
| `unicode-properties` | 0.1.4 | `MIT/Apache-2.0` |
| `unicode-script` | 0.5.8 | `MIT OR Apache-2.0` |
| `unicode-segmentation` | 1.13.3 | `MIT OR Apache-2.0` |
| `unicode-width` | 0.1.14 | `MIT OR Apache-2.0` |
| `unicode-xid` | 0.2.6 | `MIT OR Apache-2.0` |
| `url` | 2.5.8 | `MIT OR Apache-2.0` |
| `urlencoding` | 2.1.3 | `MIT` |
| `utf8_iter` | 1.0.4 | `Apache-2.0 OR MIT` |
| `uuid` | 1.24.0 | `Apache-2.0 OR MIT` |
| `version_check` | 0.9.5 | `MIT/Apache-2.0` |
| `walkdir` | 2.5.0 | `Unlicense/MIT` |
| `wasi` | 0.11.1+wasi-snapshot-preview1 | `Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT` |
| `wasip2` | 1.0.4+wasi-0.2.12 | `Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT` |
| `wasm-bindgen` | 0.2.126 | `MIT OR Apache-2.0` |
| `wasm-bindgen-futures` | 0.4.76 | `MIT OR Apache-2.0` |
| `wasm-bindgen-macro` | 0.2.126 | `MIT OR Apache-2.0` |
| `wasm-bindgen-macro-support` | 0.2.126 | `MIT OR Apache-2.0` |
| `wasm-bindgen-shared` | 0.2.126 | `MIT OR Apache-2.0` |
| `wasm-timer` | 0.2.5 | `MIT` |
| `wayland-backend` | 0.3.16 | `MIT` |
| `wayland-client` | 0.31.15 | `MIT` |
| `wayland-csd-frame` | 0.3.0 | `MIT` |
| `wayland-cursor` | 0.31.14 | `MIT` |
| `wayland-protocols` | 0.32.13 | `MIT` |
| `wayland-protocols-experimental` | 20250721.0.1 | `MIT` |
| `wayland-protocols-misc` | 0.3.12 | `MIT` |
| `wayland-protocols-plasma` | 0.3.12 | `MIT` |
| `wayland-protocols-wlr` | 0.3.12 | `MIT` |
| `wayland-scanner` | 0.31.11 | `MIT` |
| `wayland-sys` | 0.31.11 | `MIT` |
| `web-sys` | 0.3.103 | `MIT OR Apache-2.0` |
| `web-time` | 1.1.0 | `MIT OR Apache-2.0` |
| `weezl` | 0.1.12 | `MIT OR Apache-2.0` |
| `wgpu` | 0.19.4 | `MIT OR Apache-2.0` |
| `wgpu-core` | 0.19.4 | `MIT OR Apache-2.0` |
| `wgpu-hal` | 0.19.5 | `MIT OR Apache-2.0` |
| `wgpu-types` | 0.19.2 | `MIT OR Apache-2.0` |
| `widestring` | 1.2.1 | `MIT OR Apache-2.0` |
| `winapi` | 0.3.9 | `MIT/Apache-2.0` |
| `winapi-i686-pc-windows-gnu` | 0.4.0 | `MIT/Apache-2.0` |
| `winapi-util` | 0.1.11 | `Unlicense OR MIT` |
| `winapi-x86_64-pc-windows-gnu` | 0.4.0 | `MIT/Apache-2.0` |
| `window_clipboard` | 0.4.1 | `MIT` |
| `windows` | 0.52.0 | `MIT OR Apache-2.0` |
| `windows-core` | 0.52.0 | `MIT OR Apache-2.0` |
| `windows-core` | 0.62.2 | `MIT OR Apache-2.0` |
| `windows-implement` | 0.60.2 | `MIT OR Apache-2.0` |
| `windows-interface` | 0.59.3 | `MIT OR Apache-2.0` |
| `windows-link` | 0.2.1 | `MIT OR Apache-2.0` |
| `windows-result` | 0.4.1 | `MIT OR Apache-2.0` |
| `windows-strings` | 0.5.1 | `MIT OR Apache-2.0` |
| `windows-sys` | 0.52.0 | `MIT OR Apache-2.0` |
| `windows-sys` | 0.59.0 | `MIT OR Apache-2.0` |
| `windows-sys` | 0.61.2 | `MIT OR Apache-2.0` |
| `windows-targets` | 0.52.6 | `MIT OR Apache-2.0` |
| `windows_aarch64_gnullvm` | 0.52.6 | `MIT OR Apache-2.0` |
| `windows_aarch64_msvc` | 0.52.6 | `MIT OR Apache-2.0` |
| `windows_i686_gnu` | 0.52.6 | `MIT OR Apache-2.0` |
| `windows_i686_gnullvm` | 0.52.6 | `MIT OR Apache-2.0` |
| `windows_i686_msvc` | 0.52.6 | `MIT OR Apache-2.0` |
| `windows_x86_64_gnu` | 0.52.6 | `MIT OR Apache-2.0` |
| `windows_x86_64_gnullvm` | 0.52.6 | `MIT OR Apache-2.0` |
| `windows_x86_64_msvc` | 0.52.6 | `MIT OR Apache-2.0` |
| `winit` | 0.30.13 | `Apache-2.0` |
| `winnow` | 1.0.4 | `MIT` |
| `winreg` | 0.10.1 | `MIT` |
| `wit-bindgen` | 0.57.1 | `Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT` |
| `writeable` | 0.6.3 | `Unicode-3.0` |
| `x11-dl` | 2.21.0 | `MIT` |
| `x11rb` | 0.13.2 | `MIT OR Apache-2.0` |
| `x11rb-protocol` | 0.13.2 | `MIT OR Apache-2.0` |
| `xcursor` | 0.3.11 | `MIT` |
| `xdg-home` | 1.3.0 | `MIT` |
| `xkbcommon-dl` | 0.4.2 | `MIT` |
| `xkeysym` | 0.2.1 | `MIT OR Apache-2.0 OR Zlib` |
| `xml-rs` | 0.8.28 | `MIT` |
| `yazi` | 0.1.6 | `MIT OR Apache-2.0` |
| `yoke` | 0.8.3 | `Unicode-3.0` |
| `yoke-derive` | 0.8.2 | `Unicode-3.0` |
| `zbus` | 4.4.0 | `MIT` |
| `zbus` | 5.18.0 | `MIT` |
| `zbus_macros` | 4.4.0 | `MIT` |
| `zbus_macros` | 5.18.0 | `MIT` |
| `zbus_names` | 3.0.0 | `MIT` |
| `zbus_names` | 4.3.4 | `MIT` |
| `zeno` | 0.2.3 | `MIT OR Apache-2.0` |
| `zerocopy` | 0.8.55 | `BSD-2-Clause OR Apache-2.0 OR MIT` |
| `zerocopy-derive` | 0.8.55 | `BSD-2-Clause OR Apache-2.0 OR MIT` |
| `zerofrom` | 0.1.8 | `Unicode-3.0` |
| `zerofrom-derive` | 0.1.7 | `Unicode-3.0` |
| `zerotrie` | 0.2.4 | `Unicode-3.0` |
| `zerovec` | 0.11.6 | `Unicode-3.0` |
| `zerovec-derive` | 0.11.3 | `Unicode-3.0` |
| `zmij` | 1.0.23 | `MIT` |
| `zvariant` | 4.2.0 | `MIT` |
| `zvariant` | 5.13.1 | `MIT` |
| `zvariant_derive` | 4.2.0 | `MIT` |
| `zvariant_derive` | 5.13.1 | `MIT` |
| `zvariant_utils` | 2.1.0 | `MIT` |
| `zvariant_utils` | 3.5.0 | `MIT` |

## Notes

- Linux release links only crates needed for `x86_64-unknown-linux-gnu`;
  some lockfile entries (e.g. `objc2-*`) are other-target transitive noise.
- Prefer permissive dual-licensed crates; review any copyleft-only additions
  before shipping a release tarball.
