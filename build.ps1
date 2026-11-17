# Build every skill's tools and run their tests (Windows).
# Run it yourself, once, from any folder: pwsh -File <repo-root>\build.ps1
$ErrorActionPreference = 'Stop'
$scripts = Get-ChildItem (Join-Path $PSScriptRoot 'skills') -Directory |
    ForEach-Object { Join-Path $_.FullName 'build.ps1' } |
    Where-Object { Test-Path $_ }
foreach ($script in $scripts) {
    "== $script"
    & $script
}
"Built $($scripts.Count) skills"
