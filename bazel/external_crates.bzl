"""
This module declares all direct rust dependencies.
@generated by workspaceifier
"""

load("@rules_rust//crate_universe:defs.bzl", "crate", "crates_repository", "splicing_config")

def external_crates_repository(name, annotations):
    crates_repository(
        name = name,
        annotations = annotations,
        isolated = True,
        cargo_lockfile = "//:Cargo.Bazel.toml.lock",
        lockfile = "//:Cargo.Bazel.json.lock",
        cargo_config = "//:bazel/cargo.config",
        generator_urls = {
            "aarch64-apple-darwin": "https://github.com/bazelbuild/rules_rust/releases/download/0.9.0/cargo-bazel-aarch64-apple-darwin",
            "x86_64-pc-windows-gnu": "https://github.com/bazelbuild/rules_rust/releases/download/0.9.0/cargo-bazel-x86_64-pc-windows-gnu.exe",
            "x86_64-unknown-linux-gnu": "https://github.com/bazelbuild/rules_rust/releases/download/0.9.0/cargo-bazel-x86_64-unknown-linux-gnu",
            "x86_64-pc-windows-msvc": "https://github.com/bazelbuild/rules_rust/releases/download/0.9.0/cargo-bazel-x86_64-pc-windows-msvc.exe",
            "x86_64-apple-darwin": "https://github.com/bazelbuild/rules_rust/releases/download/0.9.0/cargo-bazel-x86_64-apple-darwin",
            "x86_64-unknown-linux-musl": "https://github.com/bazelbuild/rules_rust/releases/download/0.9.0/cargo-bazel-x86_64-unknown-linux-musl",
            "aarch64-unknown-linux-gnu": "https://github.com/bazelbuild/rules_rust/releases/download/0.9.0/cargo-bazel-aarch64-unknown-linux-gnu",
        },
        packages = {
            "actix-rt": crate.spec(
                version = "^2.2.0",
            ),
            "actix-web": crate.spec(
                version = "^4.0.0-beta.6",
            ),
            "arbitrary": crate.spec(
                version = "=1.1.3",
            ),
            "assert_approx_eq": crate.spec(
                version = "^1.1.0",
            ),
            "chacha20poly1305": crate.spec(
                version = "^0.10.0",
            ),
            "anyhow": crate.spec(
                version = "^1",
            ),
            "arrayvec": crate.spec(
                version = "^0.5.1",
            ),
            "askama": crate.spec(
                version = "^0.11.1",
                features = [
                    "serde-json",
                ],
            ),
            "assert-json-diff": crate.spec(
                version = "^2.0.1",
            ),
            "assert_cmd": crate.spec(
                version = "^0.12",
            ),
            "assert_matches": crate.spec(
                version = "^1.3.0",
            ),
            "async-recursion": crate.spec(
                version = "^0.3.2",
            ),
            "async-socks5": crate.spec(
                version = "^0.5.1",
            ),
            "async-stream": crate.spec(
                version = "^0.3.2",
            ),
            "async-trait": crate.spec(
                version = "^0.1.31",
            ),
            "axum": crate.spec(
                version = "^0.5.1",
            ),
            "backoff": crate.spec(
                version = "^0.3.0",
            ),
            "base32": crate.spec(
                version = "^0.4.0",
            ),
            "base64": crate.spec(
                version = "^0.11.0",
            ),
            "bech32": crate.spec(
                version = "^0.9.0",
            ),
            "bincode": crate.spec(
                version = "^1.2.1",
            ),
            "bindgen": crate.spec(
                version = "^0.59.0",
                default_features = False,
                features = ["runtime"],
            ),
            "bip32": crate.spec(
                version = "^0.4.0",
                features = [
                    "secp256k1",
                ],
            ),
            "bit-vec": crate.spec(
                version = "^0.6.3",
            ),
            "bitcoin": crate.spec(
                version = "^0.28.1",
                features = [
                    "default",
                    "rand",
                    "use-serde",
                ],
            ),
            "bitcoincore-rpc": crate.spec(
                version = "^0.15.0",
            ),
            "bitflags": crate.spec(
                version = "^1.2.1",
            ),
            "ic_bls12_381": crate.spec(
                version = "^0.7.1",
                features = [
                    "alloc",
                    "experimental",
                    "groups",
                    "pairings",
                    "zeroize",
                ],
                default_features = False,
            ),
            "build-info": crate.spec(
                git = "https://github.com/dfinity-lab/build-info",
                rev = "abb2971c5d07a9b40d41a0c84b63a3156f2ff764",
            ),
            "build-info-build": crate.spec(
                git = "https://github.com/dfinity-lab/build-info",
                rev = "abb2971c5d07a9b40d41a0c84b63a3156f2ff764",
                default_features = False,
            ),
            "byte-unit": crate.spec(
                version = "^4.0.14",
            ),
            "byteorder": crate.spec(
                version = "^1.3.4",
            ),
            "bytes": crate.spec(
                version = "^1.0.1",
            ),
            "candid": crate.spec(
                version = "^0.8.1",
            ),
            "cargo_metadata": crate.spec(
                version = "^0.14.2",
            ),
            "candid_derive": crate.spec(version = "^0.5.0"),
            "cc": crate.spec(
                version = "^1.0",
            ),
            "cddl": crate.spec(
                version = "^0.9.0-beta.1",
            ),
            "cfg-if": crate.spec(version = "^0.1.10"),
            "chrono": crate.spec(
                version = "=0.4.19",
            ),
            "ciborium": crate.spec(
                git = "https://github.com/enarx/ciborium",
                rev = "e719537c99b564c3674a56defe53713c702c6f46",
            ),
            "clap": crate.spec(
                version = "^3.1.6",
                features = [
                    "derive",
                ],
            ),
            "colored": crate.spec(
                version = "^2.0.0",
            ),
            "comparable": crate.spec(
                version = "^0.5",
                features = [
                    "derive",
                ],
            ),
            "console": crate.spec(
                version = "^0.11",
            ),
            "crc32fast": crate.spec(
                version = "^1.2.0",
            ),
            "criterion": crate.spec(
                version = "^0.3",
                features = [
                    "html_reports",
                ],
            ),
            "crossbeam": crate.spec(
                version = "^0.8.0",
            ),
            "crossbeam-channel": crate.spec(
                version = "^0.5.5",
            ),
            "crossbeam-utils": crate.spec(
                version = "^0.8.11",
            ),
            "csv": crate.spec(
                version = "^1.1",
            ),
            "curve25519-dalek": crate.spec(
                version = "^3.0.2",
            ),
            "cvt": crate.spec(
                version = "^0.1.1",
            ),
            "dashmap": crate.spec(
                version = "^5.3.4",
            ),
            "debug_stub_derive": crate.spec(
                version = "^0.3.0",
            ),
            "derive_more": crate.spec(
                git = "https://github.com/dfinity-lab/derive_more",
                rev = "9f1b894e6fde640da4e9ea71a8fc0e4dd98d01da",
            ),
            "digest": crate.spec(
                version = "^0.9.0",
            ),
            "ed25519-consensus": crate.spec(
                version = "^2.0.1",
            ),
            "either": crate.spec(
                version = "^1.6",
            ),
            "erased-serde": crate.spec(
                version = "^0.3.11",
            ),
            "escargot": crate.spec(
                version = "^0.5.7",
                features = ["print"],
            ),
            "exec": crate.spec(
                version = "^0.3.1",
            ),
            "eyre": crate.spec(
                version = "^0.6.8",
            ),
            "features": crate.spec(
                version = "^0.10.0",
            ),
            "ff": crate.spec(
                version = "^0.12.0",
                features = [
                    "std",
                ],
                default_features = False,
            ),
            "fix-hidden-lifetime-bug": crate.spec(
                version = "^0.2.4",
            ),
            "flate2": crate.spec(
                version = "^1.0.22",
            ),
            "float-cmp": crate.spec(
                version = "^0.9.0",
            ),
            "form_urlencoded": crate.spec(
                version = "^1.0.0",
            ),
            "fs_extra": crate.spec(
                version = "^1.2.0",
            ),
            "futures": crate.spec(
                version = "^0.3.6",
            ),
            "futures-util": crate.spec(
                version = "^0.3.8",
            ),
            "futures-core": crate.spec(
                version = "^0.3.21",
            ),
            "garcon": crate.spec(
                version = "^0.2.3",
            ),
            "getrandom": crate.spec(
                version = "^0.2",
                features = [
                    "custom",
                ],
            ),
            "gflags": crate.spec(
                version = "^0.3.7",
            ),
            "gflags-derive": crate.spec(
                version = "^0.1",
            ),
            "glob": crate.spec(
                version = "^0.3.0",
            ),
            "h2": crate.spec(
                version = "^0.3.14",
            ),
            "hashlink": crate.spec(
                version = "^0.8.0",
            ),
            "hex": crate.spec(
                version = "^0.4.3",
                features = [
                    "serde",
                ],
            ),
            "hex-literal": crate.spec(
                version = "^0.2.1",
            ),
            "http": crate.spec(
                version = "^0.2.6",
            ),
            "http-body": crate.spec(
                version = "^0.4",
            ),
            "httptest": crate.spec(
                version = "^0.15.4",
            ),
            "humantime": crate.spec(
                version = "^2.1.0",
            ),
            "humantime-serde": crate.spec(
                version = "^1.0",
            ),
            "hyper": crate.spec(
                version = "^0.14.18",
                features = [
                    "client",
                    "full",
                    "http1",
                    "http2",
                    "server",
                    "tcp",
                ],
            ),
            "hyper-rustls": crate.spec(
                version = "^0.23.0",
                features = [
                    "webpki-roots",
                ],
            ),
            "hyper-socks2": crate.spec(
                version = "^0.6.0",
            ),
            "hyper-tls": crate.spec(
                version = "^0.5.0",
            ),
            "iai": crate.spec(
                version = "^0.1",
            ),
            "ic-agent": crate.spec(
                version = "^0.22.0",
                features = [
                    "hyper",
                ],
            ),
            "ic-cdk": crate.spec(
                version = "^0.6.0",
                default_features = False,
            ),
            "ic-cdk-macros": crate.spec(
                version = "^0.6.0",
            ),
            "ic-certified-map": crate.spec(
                git = "https://github.com/dfinity/cdk-rs",
                rev = "2112e912e156b271389a51777680de542bb43980",
            ),
            "ic-identity-hsm": crate.spec(
                version = "=0.22.0",
            ),
            "ic-ledger-types": crate.spec(
                version = "^0.1.1",
            ),
            "ic-stable-structures": crate.spec(
                version = "^0.1.0",
            ),
            "ic-utils": crate.spec(
                version = "^0.22.0",
                features = [
                    "raw",
                ],
            ),
            "ic-wasm": crate.spec(
                version = "^0.1.3",
            ),
            "indicatif": crate.spec(
                version = "^0.15",
                features = [
                    "improved_unicode",
                ],
            ),
            "indoc": crate.spec(
                version = "^1.0.6",
            ),
            "insta": crate.spec(
                version = "=1.8.0",
            ),
            "intmap": crate.spec(
                version = "^1.1.0",
                features = ["serde"],
            ),
            "ipnet": crate.spec(
                version = "^2.5.0",
            ),
            "itertools": crate.spec(
                version = "^0.10.0",
            ),
            "jemalloc-ctl": crate.spec(
                version = "^0.3.3",
            ),
            "jemallocator": crate.spec(
                version = "^0.3.2",
            ),
            "json-patch": crate.spec(
                version = "^0.2.6",
            ),
            "json5": crate.spec(
                version = "^0.4.1",
            ),
            "k256": crate.spec(
                version = "^0.11.6",
                features = [
                    "arithmetic",
                    "ecdsa",
                ],
                default_features = False,
            ),
            "lazy_static": crate.spec(
                version = "^1.4.0",
            ),
            "lazy-regex": crate.spec(
                version = "^2",
            ),
            "leaky-bucket": crate.spec(
                version = "^0.11.0",
            ),
            "leb128": crate.spec(
                version = "^0.2.5",
            ),
            "libc": crate.spec(
                version = "^0.2.91",
            ),
            "libflate": crate.spec(
                version = "^1.1.2",
            ),
            "libfuzzer-sys": crate.spec(
                version = "^0.4",
            ),
            "libsecp256k1": crate.spec(
                version = "^0.7.0",
            ),
            "linked-hash-map": crate.spec(
                version = "^0.5.3",
            ),
            "log": crate.spec(
                version = "^0.4.14",
            ),
            "log4rs": crate.spec(
                version = "^1.1.1",
            ),
            "lru": crate.spec(
                version = "^0.7.1",
                default_features = False,
            ),
            "maplit": crate.spec(
                version = "^1.0.2",
            ),
            "mio": crate.spec(
                version = "^0.7",
                features = [
                    "os-ext",
                    "os-poll",
                    "pipe",
                ],
            ),
            "mockall": crate.spec(
                version = "^0.11.1",
            ),
            "mockall-0_7_2": crate.spec(
                package = "mockall",
                version = "^0.7.2",
            ),
            "mockall-0_8_3": crate.spec(
                package = "mockall",
                version = "^0.8.3",
            ),
            "native-tls": crate.spec(
                version = "^0.2.7",
                features = [
                    "alpn",
                ],
            ),
            "nix": crate.spec(
                version = "^0.23.0",
            ),
            "nonblock": crate.spec(
                version = "^0.1.0",
            ),
            "notify": crate.spec(
                version = "^4.0.12",
            ),
            "num": crate.spec(
                version = "^0.4.0",
            ),
            "num-bigint": crate.spec(
                version = "^0.4.0",
            ),
            "num-bigint-dig": crate.spec(
                version = "0.8",
                features = ["prime"],
            ),
            "num-integer": crate.spec(
                version = "^0.1.41",
            ),
            "num-rational": crate.spec(
                version = "^0.2.2",
            ),
            "num-traits": crate.spec(
                version = "^0.2.12",
                features = [
                    "libm",
                ],
                default_features = False,
            ),
            "num_cpus": crate.spec(
                version = "^1.13.1",
            ),
            "once_cell": crate.spec(
                version = "^1.8",
            ),
            "openssh-keys": crate.spec(
                version = "^0.5.0",
            ),
            "openssl": crate.spec(
                version = "^0.10.29",
            ),
            "opentelemetry": crate.spec(
                version = "^0.17.0",
            ),
            "opentelemetry-prometheus": crate.spec(
                version = "^0.10.0",
            ),
            "p256": crate.spec(
                version = "^0.10",
                features = [
                    "arithmetic",
                ],
                default_features = False,
            ),
            "pairing": crate.spec(
                version = "^0.22",
            ),
            "parity-wasm": crate.spec(
                version = "^0.42.2",
                features = [
                    "bulk",
                    "multi_value",
                    "std",
                ],
            ),
            "parking_lot": crate.spec(
                version = "^0.12.1",
            ),
            "parse_int": crate.spec(
                version = "^0.4.0",
            ),
            "paste": crate.spec(
                version = "^1.0.0",
            ),
            "pathdiff": crate.spec(
                version = "^0.2.1",
            ),
            "pem": crate.spec(
                version = "^1.0.1",
            ),
            "pico-args": crate.spec(
                version = "^0.3",
            ),
            "pkg-config": crate.spec(
                version = "^0.3",
            ),
            "pprof": crate.spec(
                version = "^0.10.1",
                features = [
                    "flamegraph",
                    "prost-codec",
                ],
                default_features = False,
            ),
            "predicates": crate.spec(
                version = "^1.0.1",
            ),
            "pretty-bytes": crate.spec(
                version = "^0.2.2",
            ),
            "pretty_assertions": crate.spec(
                version = "^0.6.1",
            ),
            "proc-macro2": crate.spec(
                version = "^1.0",
            ),
            "procfs": crate.spec(
                version = "^0.9",
                default_features = False,
            ),
            "prometheus": crate.spec(
                version = "^0.13.0",
                features = [
                    "process",
                ],
            ),
            "proptest": crate.spec(
                version = "^1.0.0",
            ),
            "test-strategy": crate.spec(
                version = "^0.2",
            ),
            "proptest-derive": crate.spec(
                version = "^0.3.0",
            ),
            "prost": crate.spec(
                version = "^0.11.0",
            ),
            "prost-build": crate.spec(
                version = "^0.11.0",
            ),
            "prost-derive": crate.spec(
                version = "^0.11",
            ),
            "protobuf": crate.spec(
                version = "^2.27.1",
            ),
            "quickcheck": crate.spec(
                version = "^1.0.3",
            ),
            "quote": crate.spec(
                version = "^1.0",
            ),
            "rand-0_8_4": crate.spec(
                package = "rand",
                version = "^0.8.4",
                features = [
                    "small_rng",
                ],
            ),
            "rand_chacha-0_3_1": crate.spec(
                package = "rand_chacha",
                version = "^0.3.1",
            ),
            "rand_distr-0_4": crate.spec(
                package = "rand_distr",
                version = "^0.4",
            ),
            "rand_pcg": crate.spec(
                version = "^0.3.1",
            ),
            "randomkit": crate.spec(
                version = "^0.1.1",
            ),
            "rayon": crate.spec(
                version = "^1.5.1",
            ),
            "regex": crate.spec(
                version = "^1.3.9",
            ),
            "reqwest": crate.spec(
                version = "^0.11.1",
                features = [
                    "blocking",
                    "json",
                    "multipart",
                    "native-tls",
                    "stream",
                ],
            ),
            "retain_mut": crate.spec(
                version = "^0.1",
            ),
            "ring": crate.spec(
                version = "^0.16.11",
                features = [
                    "std",
                ],
            ),
            "ripemd": crate.spec(
                version = "^0.1.1",
            ),
            "rocksdb": crate.spec(
                version = "^0.15.0",
                default_features = False,
            ),
            "rsa": crate.spec(
                version = "^0.6.1",
            ),
            "rsa-0_4_0": crate.spec(
                package = "rsa",
                version = "^0.4.0",
            ),
            "rusqlite": crate.spec(
                version = "^0.28.0",
                features = ["bundled"],
            ),
            "rust_decimal": crate.spec(
                version = "^1.25.0",
            ),
            "rust_decimal_macros": crate.spec(
                version = "^1.25.0",
            ),
            "rustc-hash": crate.spec(
                version = "^1.1.0",
            ),
            "rustls": crate.spec(
                version = "^0.20.4",
            ),
            "rustls-pemfile": crate.spec(
                version = "^1",
            ),
            "rustversion": crate.spec(
                version = "^1.0",
            ),
            "rusty-fork": crate.spec(
                version = "^0.3.0",
            ),
            "scoped_threadpool": crate.spec(
                version = "0.1.*",
            ),
            "semver": crate.spec(
                version = "^1.0.9",
                features = [
                    "serde",
                ],
            ),
            "serde": crate.spec(
                version = "^1.0.99",
                features = [
                    "derive",
                ],
                default_features = False,
            ),
            "serde-bytes-repr": crate.spec(
                version = "^0.1.5",
            ),
            "serde_bytes": crate.spec(
                version = "^0.11",
            ),
            "serde_cbor": crate.spec(
                version = "^0.11.2",
            ),
            "serde_derive": crate.spec(
                version = "^1.0",
            ),
            "serde_json": crate.spec(
                version = "^1.0.40",
            ),
            "serde_millis": crate.spec(
                version = "^0.1",
            ),
            "serde_with": crate.spec(
                version = "^1.6.2",
            ),
            "serde_yaml": crate.spec(
                version = "^0.8.24",
            ),
            "serial_test": crate.spec(
                version = "^0.8.0",
            ),
            "sha2": crate.spec(
                version = "^0.10.2",
            ),
            "sha2-0_9_1": crate.spec(
                package = "sha2",
                version = "^0.9.1",
            ),
            "sha3": crate.spec(
                version = "^0.9.1",
            ),
            "signal-hook": crate.spec(
                version = "^0.3.6",
                features = [
                    "iterator",
                ],
            ),
            "signal-hook-mio": crate.spec(
                version = "^0.2.0",
                features = [
                    "support-v0_7",
                ],
            ),
            "simple_asn1": crate.spec(
                version = "^0.5.4",
            ),
            "slog": crate.spec(
                version = "^2.5.2",
                features = [
                    "max_level_trace",
                    "nested-values",
                    "release_max_level_debug",
                    "release_max_level_trace",
                ],
            ),
            "slog-async": crate.spec(
                version = "^2.5",
                features = [
                    "nested-values",
                ],
            ),
            "slog-envlogger": crate.spec(
                version = "^2.2.0",
            ),
            "slog-json": crate.spec(
                version = "^2.3",
                features = [
                    "nested-values",
                ],
            ),
            "slog-scope": crate.spec(
                version = "^4.1.2",
            ),
            "slog-term": crate.spec(
                version = "^2.6.0",
            ),
            "slog_derive": crate.spec(
                version = "^0.2.0",
            ),
            "socket2": crate.spec(
                version = "^0.3.19",
                features = [
                    "reuseport",
                ],
            ),
            "ssh2": crate.spec(
                git = "https://github.com/dfinity-lab/ssh2-rs",
                rev = "f842906afaa2443206b8365d51950ed3ef85c940",
            ),
            "static_assertions": crate.spec(
                version = "^0.3.4",
            ),
            "statrs": crate.spec(
                version = "^0.15.0",
            ),
            "strum": crate.spec(
                version = "^0.23.0",
                features = [
                    "derive",
                ],
            ),
            "strum_macros": crate.spec(
                version = "^0.23.0",
            ),
            "substring": crate.spec(
                version = "^1.4.5",
            ),
            "subtle": crate.spec(
                version = "^2.4",
            ),
            "syn": crate.spec(
                version = "^1.0",
                features = [
                    "fold",
                    "full",
                ],
            ),
            "tar": crate.spec(
                version = "^0.4.38",
            ),
            "tarpc": crate.spec(
                version = "^0.27",
                features = [
                    "full",
                ],
            ),
            "tempfile": crate.spec(
                version = "^3.1.0",
            ),
            "tester": crate.spec(
                version = "^0.7.0",
            ),
            "thiserror": crate.spec(
                version = "^1.0",
            ),
            "thread_profiler": crate.spec(
                version = "^0.3",
            ),
            "threadpool": crate.spec(
                version = "^1.8.1",
            ),
            "tiny_http": crate.spec(
                version = "^0.10.0",
            ),
            "tokio": crate.spec(
                version = "^1.15.0",
                features = [
                    "full",
                    "io-util",
                    "macros",
                    "net",
                    "rt",
                    "sync",
                    "time",
                ],
            ),
            "tokio-openssl": crate.spec(
                version = "^0.6.1",
            ),
            "tokio-rustls": crate.spec(
                version = "^0.22.0",
                features = [
                    "dangerous_configuration",
                ],
            ),
            "tokio-serde": crate.spec(
                version = "^0.8",
                features = [
                    "bincode",
                    "json",
                ],
            ),
            "tokio-socks": crate.spec(
                version = "^0.5.1",
            ),
            "tokio-test": crate.spec(
                version = "^0.4.2",
            ),
            "tokio-util": crate.spec(
                version = "^0.6.8",
            ),
            "toml": crate.spec(
                version = "^0.5.9",
            ),
            "tonic": crate.spec(
                version = "^0.8.2",
            ),
            "tonic-build": crate.spec(
                version = "^0.8.2",
            ),
            "tower": crate.spec(
                version = "^0.4.11",
                features = [
                    "buffer",
                    "limit",
                    "load-shed",
                    "steer",
                    "timeout",
                    "util",
                ],
            ),
            "tower-http": crate.spec(
                version = "^0.3",
                features = [
                    "trace",
                ],
            ),
            "tower-test": crate.spec(
                version = "^0.4.0",
            ),
            "tracing": crate.spec(
                version = "^0.1.34",
            ),
            "tracing-appender": crate.spec(
                version = "^0.2.2",
            ),
            "tracing-subscriber": crate.spec(
                version = "^0.3.11",
                features = [
                    "json",
                ],
            ),
            "url": crate.spec(
                version = "^2.1.1",
                features = [
                    "serde",
                ],
            ),
            "uuid": crate.spec(
                version = "^0.8.1",
                features = [
                    "v4",
                ],
            ),
            "vsock": crate.spec(
                version = "^0.2.6",
            ),
            "walrus": crate.spec(
                version = "^0.19.0",
            ),
            "wait-timeout": crate.spec(
                version = "^0.2.0",
            ),
            "walkdir": crate.spec(
                version = "^2.3.1",
            ),
            "warp": crate.spec(
                version = "^0.3.2",
                features = [
                    "tls",
                ],
            ),
            "wasm-bindgen": crate.spec(
                version = "^0.2",
            ),
            "wasm-encoder": crate.spec(
                version = "^0.18.0",
            ),
            "wasmparser": crate.spec(
                version = "^0.92.0",
            ),
            "wasmtime": crate.spec(
                version = "^1.0.1",
                default_features = False,
                features = [
                    "cranelift",
                    "parallel-compilation",
                    "posix-signals-on-macos",
                ],
            ),
            "wasmtime-environ": crate.spec(
                version = "^1.0.1",
            ),
            "wasmtime-runtime": crate.spec(
                version = "^1.0.1",
            ),
            "webpki": crate.spec(
                version = "^0.21.4",
            ),
            "webpki-roots": crate.spec(
                version = "^0.22",
            ),
            "wee_alloc": crate.spec(
                version = "^0.4.3",
            ),
            "which": crate.spec(
                version = "^4.2.2",
            ),
            "wsl": crate.spec(
                version = "^0.1.0",
            ),
            "wycheproof": crate.spec(
                version = "^0.4",
            ),
            "x509-parser": crate.spec(
                version = "^0.12.0",
            ),
            "yansi": crate.spec(
                version = "^0.5.0",
            ),
            "zeroize": crate.spec(
                version = "^1.4.3",
                features = [
                    "zeroize_derive",
                ],
            ),
        },
        splicing_config = splicing_config(
            resolver_version = "2",
        ),
    )
