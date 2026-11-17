# Build aimcalc and aimcount, run aimcalc's tests, and put both executables in bin\ (Windows).
# Run it yourself, once, from any folder: pwsh -File <skill-root>\build.ps1
$ErrorActionPreference = 'Stop'
$root = $PSScriptRoot
$target = Join-Path $root 'target'
foreach ($crate in 'aimcalc', 'aimcount') {
    cargo build --release --manifest-path (Join-Path $root "$crate\Cargo.toml") --target-dir $target
    if ($LASTEXITCODE -ne 0) { throw "cargo build failed for $crate" }
}
cargo test --release --manifest-path (Join-Path $root 'aimcalc\Cargo.toml') --target-dir $target
if ($LASTEXITCODE -ne 0) { throw 'aimcalc tests failed' }
$bin = Join-Path $root 'bin'
New-Item -ItemType Directory -Force $bin | Out-Null
Copy-Item (Join-Path $target 'release\aimcalc.exe'), (Join-Path $target 'release\aimcount.exe') $bin -Force
"Built $bin\aimcalc.exe and $bin\aimcount.exe"
