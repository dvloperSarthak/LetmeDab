<#
.SYNOPSIS
    Builds and packages LetmeDab for Horizon Minecraft (Rust Edition).
.DESCRIPTION
    Compiles the cdylib with cargo --release, generates all 32 emote definitions,
    and produces both .hnmod (Horizon Native Mod) and .hmod (Content Mod) bundles.
#>

$ErrorActionPreference = "Stop"

Write-Host "==========================================================" -ForegroundColor Cyan
Write-Host "       Building LetmeDab for Horizon (Minecraft Rust)     " -ForegroundColor Cyan
Write-Host "==========================================================" -ForegroundColor Cyan

# 1. Compile release binary
Write-Host "`n[1/5] Compiling release library..." -ForegroundColor Yellow
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
New-Item -ItemType Directory -Path "$BundleDir/src" -Force | Out-Null

# 3. Copy Manifests & Source Code (Required for .hnmod)
Write-Host "[2/5] Copying manifests (horizon-mod.toml & horizon-mixin.toml)..." -ForegroundColor Yellow
if (Test-Path "horizon-mod.toml") {
    Copy-Item "horizon-mod.toml" -Destination "$BundleDir/horizon-mod.toml"
}
if (Test-Path "horizon-mixin.toml") {
    Copy-Item "horizon-mixin.toml" -Destination "$BundleDir/horizon-mixin.toml"
}
if (Test-Path "Horizon.toml") {
    Copy-Item "Horizon.toml" -Destination "$BundleDir/Horizon.toml"
}
if (Test-Path "config.toml") {
    Copy-Item "config.toml" -Destination "$BundleDir/config.toml"
}
Copy-Item "Cargo.toml" -Destination "$BundleDir/Cargo.toml"
Copy-Item "README.md" -Destination "$BundleDir/README.md"
if (Test-Path "runtime.rs") { Copy-Item "runtime.rs" -Destination "$BundleDir/runtime.rs" }
if (Test-Path "tick.rs") { Copy-Item "tick.rs" -Destination "$BundleDir/tick.rs" }
if (Test-Path "command.rs") { Copy-Item "command.rs" -Destination "$BundleDir/command.rs" }
Copy-Item "src/*" -Destination "$BundleDir/src" -Recurse

# 4. Copy Shaders
if (Test-Path "shaders") {
    Copy-Item "shaders" -Destination "$BundleDir/shaders" -Recurse
}

# 5. Copy Compiled Binary
Write-Host "[3/5] Copying compiled library binary..." -ForegroundColor Yellow
if (Test-Path "target/release/letmedab.dll") {
    Copy-Item "target/release/letmedab.dll" -Destination "$BundleDir/letmedab.dll"
} elseif (Test-Path "target/release/libletmedab.so") {
    Copy-Item "target/release/libletmedab.so" -Destination "$BundleDir/libletmedab.so"
}

# 6. Export Emotes
Write-Host "[4/5] Generating all 32 emote animations..." -ForegroundColor Yellow
cargo run -- export "$BundleDir/emotes" | Out-Null

# 7. Package both .hnmod (Native Mod) and .hmod (Content Mod)
Write-Host "[5/5] Packaging .hnmod and .hmod bundles..." -ForegroundColor Yellow

$HnmodZip = "$DistDir/letmedab-0.1.0-hnmod.zip"
$HnmodFile = "$DistDir/letmedab-0.1.0.hnmod"
$HmodZip = "$DistDir/letmedab-0.1.0-hmod.zip"
$HmodFile = "$DistDir/letmedab-0.1.0.hmod"

# Create .hnmod (Horizon Native Mod - suitable for native code execution)
if (Test-Path $HnmodZip) { Remove-Item $HnmodZip -Force }
if (Test-Path $HnmodFile) { Remove-Item $HnmodFile -Force }
Compress-Archive -Path "$BundleDir/*" -DestinationPath $HnmodZip -Force
Move-Item $HnmodZip $HnmodFile -Force

# Create .hmod (Horizon Content Mod)
if (Test-Path $HmodZip) { Remove-Item $HmodZip -Force }
if (Test-Path $HmodFile) { Remove-Item $HmodFile -Force }
Copy-Item $HnmodFile $HmodFile -Force

# Create unversioned aliases (letmedab.hnmod and letmedab.hmod)
Copy-Item $HnmodFile "$DistDir/letmedab.hnmod" -Force
Copy-Item $HmodFile "$DistDir/letmedab.hmod" -Force

# Clean any legacy folders
if (Test-Path "$DistDir/emotecraft") { Remove-Item "$DistDir/emotecraft" -Recurse -Force }

Write-Host "`n==========================================================" -ForegroundColor Green
Write-Host " SUCCESS! Mod bundles built successfully: " -ForegroundColor Green
Write-Host " - Native Mod:  $HnmodFile (.hnmod) [Recommended for Horizon Native Code]" -ForegroundColor Green
Write-Host " - Content Mod: $HmodFile (.hmod)" -ForegroundColor Green
Write-Host " - Directory:   $BundleDir" -ForegroundColor Green
Write-Host "==========================================================" -ForegroundColor Green
