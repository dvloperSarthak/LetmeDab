<#
.SYNOPSIS
    Builds and packages LetmeDab for Horizon Minecraft (Rust Edition).
.DESCRIPTION
    Compiles the cdylib with cargo --release, generates all 32 emote definitions,
    and produces a ready-to-use letmedab.hmod bundle.
#>

$ErrorActionPreference = "Stop"

Write-Host "==========================================================" -ForegroundColor Cyan
Write-Host "       Building LetmeDab for Horizon (Minecraft Rust)     " -ForegroundColor Cyan
Write-Host "==========================================================" -ForegroundColor Cyan

# 1. Compile release binary
Write-Host "`n[1/4] Compiling release library..." -ForegroundColor Yellow
cargo build --release
if ($LASTEXITCODE -ne 0) {
    Write-Error "Cargo build failed!"
    exit 1
}

# 2. Prepare staging directory
$DistDir = "dist"
$BundleDir = "$DistDir/letmedab"
if (Test-Path $BundleDir) {
    Remove-Item $BundleDir -Recurse -Force
}
New-Item -ItemType Directory -Path "$BundleDir/emotes" -Force | Out-Null

# 3. Copy Manifest & Assets
Write-Host "[2/4] Copying horizon-mod.toml and assets..." -ForegroundColor Yellow
if (Test-Path "horizon-mod.toml") {
    Copy-Item "horizon-mod.toml" -Destination "$BundleDir/horizon-mod.toml"
}
if (Test-Path "Horizon.toml") {
    Copy-Item "Horizon.toml" -Destination "$BundleDir/Horizon.toml"
}
Copy-Item "README.md" -Destination "$BundleDir/README.md"

# 4. Copy Compiled Binary
Write-Host "[3/4] Copying compiled binary..." -ForegroundColor Yellow
if (Test-Path "target/release/letmedab.dll") {
    Copy-Item "target/release/letmedab.dll" -Destination "$BundleDir/letmedab.dll"
} elseif (Test-Path "target/release/libletmedab.so") {
    Copy-Item "target/release/libletmedab.so" -Destination "$BundleDir/libletmedab.so"
}

# 5. Export Emotes
Write-Host "[4/4] Generating all 32 emote animations..." -ForegroundColor Yellow
cargo run -- export "$BundleDir/emotes" | Out-Null

# 6. Compress into .hmod archive
$ZipFile = "$DistDir/letmedab-0.1.0.zip"
$HmodFile = "$DistDir/letmedab-0.1.0.hmod"
if (Test-Path $ZipFile) { Remove-Item $ZipFile -Force }
if (Test-Path $HmodFile) { Remove-Item $HmodFile -Force }
Compress-Archive -Path "$BundleDir/*" -DestinationPath $ZipFile -Force
Move-Item $ZipFile $HmodFile -Force

Write-Host "`n==========================================================" -ForegroundColor Green
Write-Host " SUCCESS! LetmeDab mod bundle built successfully: " -ForegroundColor Green
Write-Host " - Directory: $BundleDir" -ForegroundColor Green
Write-Host " - Archive:   $HmodFile (.hmod)" -ForegroundColor Green
Write-Host " Place '$HmodFile' or the 'letmedab' folder into your Horizon 'mods/' directory!" -ForegroundColor Green
Write-Host "==========================================================" -ForegroundColor Green
