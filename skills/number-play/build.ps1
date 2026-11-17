# Build numplay, run its tests, and put the executable in bin\ (Windows).
# Run it yourself, once, from any folder: pwsh -File <skill-root>\build.ps1
$ErrorActionPreference = 'Stop'
$root = $PSScriptRoot
$target = Join-Path $root 'target'
$manifest = Join-Path $root 'numplay\Cargo.toml'
cargo build --release --manifest-path $manifest --target-dir $target
if ($LASTEXITCODE -ne 0) { throw 'cargo build failed for numplay' }
cargo test --release --manifest-path $manifest --target-dir $target
if ($LASTEXITCODE -ne 0) { throw 'numplay tests failed' }
$bin = Join-Path $root 'bin'
New-Item -ItemType Directory -Force $bin | Out-Null
Copy-Item (Join-Path $target 'release\numplay.exe') $bin -Force
"Built $bin\numplay.exe"
