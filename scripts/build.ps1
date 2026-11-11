param(
    [string]$MinGWPath
)

$ErrorActionPreference = 'Stop'
if (-not $MinGWPath) {
    throw 'MinGWPath is required. Pass -MinGWPath <path to mingw32\bin>.'
}
$root = Split-Path $PSScriptRoot -Parent
Set-Location $root

$gcc = Join-Path $MinGWPath 'gcc.exe'
$env:Path = "$MinGWPath;$env:Path"
$env:CARGO_TARGET_I686_PC_WINDOWS_GNU_LINKER = $gcc
$env:WINDRES = Join-Path $MinGWPath 'windres.exe'
$env:WINDRES_PREPROCESSOR = Join-Path $MinGWPath 'cpp.exe'

$target = 'i686-pc-windows-gnu'

cargo +stable-x86_64-pc-windows-gnu build -p imouto-speedhack --locked --release --target $target
if ($LASTEXITCODE -ne 0) {
    exit $LASTEXITCODE
}

cargo +stable-x86_64-pc-windows-gnu build -p imouto-life-fantasy-trainer --locked --release --target $target
if ($LASTEXITCODE -ne 0) {
    exit $LASTEXITCODE
}

$exe = Join-Path $root "target\$target\release\ImoutoLifeFantasyTrainer.exe"
$size = [math]::Round((Get-Item $exe).Length / 1MB, 1)
Write-Host "Built $exe ($size MB)"
