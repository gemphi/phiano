# Phi-4 taught a new word by pointing (Philosophical Investigations §28-30, docs/116).
#
#   powershell -ExecutionPolicy Bypass -File C:\Users\phiac\Workspace\gemphi\phinet\crates\phiano\scripts\phi4_sepia.ps1
#   ... phi4_sepia.ps1 -N 30 -Parallel 2
#
# Four teachings, as for our students (paper 19): 1 or 3 pointings at a group,
# with or without the grammar word `colour`. Each is told in the prompt, then
# Phi-4 (temperature 0, roles stated) is given N orders that use `sepia`.
# Answers are saved as they arrive; a stopped run resumes.
# Report: data\builders\sepia\sepia.md (Phi-4 beside our counts student on
# the same orders).
# The needle (paper 24 §7): the two `colour` teachings again with 25, 100
# and 400 other exchanges between the pointing and the order, so the
# teaching sits that far back in Phi-4's transcript. Our counts student
# does not have a transcript: its number is the same at any distance.
param([int]$N = 50, [int]$Parallel = 3, [int]$Seed = 7, [int[]]$Between = @(25, 100, 400))
$ErrorActionPreference = "Continue"
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8
$env:Path = [Environment]::GetEnvironmentVariable("Path", "Machine") + ";" + [Environment]::GetEnvironmentVariable("Path", "User")
$phiano = Split-Path -Parent (Split-Path -Parent $MyInvocation.MyCommand.Path)
# phiano is a member of the phinet workspace: its binaries land in the workspace's target directory.
$target = (cargo metadata --no-deps --format-version 1 --manifest-path (Join-Path $phiano "Cargo.toml") | ConvertFrom-Json).target_directory
$out = Join-Path $phiano "data\builders\sepia"
New-Item -ItemType Directory -Force $out | Out-Null
$modelfile = Join-Path $phiano "data\builders\phi4-builder.Modelfile"
"FROM phi4`nPARAMETER temperature 0`nPARAMETER seed $Seed" | Set-Content $modelfile -Encoding ascii
ollama create phi4-builder -f $modelfile
Add-Type -Namespace Phi -Name Power -MemberDefinition '[DllImport("kernel32.dll")] public static extern uint SetThreadExecutionState(uint f);'
[Phi.Power]::SetThreadExecutionState([uint32]"0x80000001") | Out-Null
try {
    Set-Location $phiano
    cargo build --release --example language_game
    $exe = Join-Path $target "release\language_game.exe"
    & $exe sepia-ask $out $N --parallel $Parallel --seed $Seed ollama run phi4-builder
    foreach ($b in $Between) {
        & $exe sepia-ask $out $N --parallel $Parallel --seed $Seed --between $b ollama run phi4-builder
    }
    & $exe sepia-report $out (Join-Path $out "sepia.md") --seed $Seed
    Get-Content (Join-Path $out "sepia.md")
}
finally {
    [Phi.Power]::SetThreadExecutionState([uint32]"0x80000000") | Out-Null
}
