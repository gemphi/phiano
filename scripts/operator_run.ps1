# The operator's long run (docs/150): phinet's Joint FNO trained for hours on
# held-out Emma's told `word next word` rows, a checkpoint and a measurement
# of the two strata (docs/137) every -Every epochs.
#
#   powershell -ExecutionPolicy Bypass -File C:\Users\phiac\Workspace\gemphi\phinet\crates\phiano\scripts\operator_run.ps1
#   ... operator_run.ps1 -R 128 -Epochs 96 -Every 8 -Seed 7
#   ... operator_run.ps1 -R 128 -Epochs 32 -Every 4 -Ahead 3   (the offsets, docs/151)
#
# 1. Keeps the PC awake while it runs.
# 2. Builds operator_run once (release).
# 3. Trains and measures; every checkpoint lands in data\runs\operator_r<R>_s<Seed>\
#    (data\runs\operator_r<R>_s<Seed>_ahead<A>\ with -Ahead above 1)
#    (epoch_<n>.relate) and its strata table is appended to log.md there.
# Stopped (Ctrl+C, a reboot)? Run the same command again: it resumes from the
# latest checkpoint. The cloud run of the same binary is R = 64, seed 7; this
# one defaults to R = 128 so the two together say what R buys.
param([int]$R = 128, [int]$Epochs = 96, [int]$Every = 8, [int]$Seed = 7, [int]$Ahead = 1)
$ErrorActionPreference = "Continue"
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8
$env:Path = [Environment]::GetEnvironmentVariable("Path", "Machine") + ";" + [Environment]::GetEnvironmentVariable("Path", "User")
$phiano = Split-Path -Parent (Split-Path -Parent $MyInvocation.MyCommand.Path)
$target = (cargo metadata --no-deps --format-version 1 --manifest-path (Join-Path $phiano "Cargo.toml") | ConvertFrom-Json).target_directory
$out = Join-Path $phiano ("data\runs\operator_r{0}_s{1}" -f $R, $Seed)
if ($Ahead -gt 1) { $out = "{0}_ahead{1}" -f $out, $Ahead }
New-Item -ItemType Directory -Force $out | Out-Null
Add-Type -Namespace Phi -Name Power -MemberDefinition '[DllImport("kernel32.dll")] public static extern uint SetThreadExecutionState(uint f);'
[Phi.Power]::SetThreadExecutionState([uint32]"0x80000001") | Out-Null
try {
    Set-Location $phiano
    cargo build --release --example operator_run
    $run = Join-Path $target "release\operator_run.exe"
    & $run data\text --in text\emma --held-out 0.05 --seed $Seed --r $R --epochs $Epochs --every $Every --ahead $Ahead --out $out 2>&1 | Tee-Object -FilePath (Join-Path $out "console.txt") -Append
}
finally {
    [Phi.Power]::SetThreadExecutionState([uint32]"0x80000000") | Out-Null
}
