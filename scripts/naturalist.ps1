# The Naturalist persona from the night's tables (docs/99, phiace/personas).
#
#   powershell -ExecutionPolicy Bypass -File C:\Users\phiac\Workspace\gemphi\phinet\crates\phiano\scripts\naturalist.ps1
#
# 1. Reuses night_2026-09-29/<seed>/relations.tsv and asks Phi-4 only the voice
#    question for each word those tables asked about (one short question a
#    word); `cat` has no night table, so its quest runs in full.
# 2. Builds the persona and its relation model (inheritance as the file says).
# 3. Asks the model a few questions and the persona one.
$ErrorActionPreference = "Continue"
$env:Path = [Environment]::GetEnvironmentVariable("Path", "Machine") + ";" + [Environment]::GetEnvironmentVariable("Path", "User")
$phiano = Split-Path -Parent (Split-Path -Parent $MyInvocation.MyCommand.Path)
$phiace = Join-Path (Split-Path -Parent (Split-Path -Parent (Split-Path -Parent $phiano))) "phiace"
$persona = Join-Path $phiace "personas\naturalist.md"
$root = Join-Path $phiano "data\personas"
$night = Join-Path $phiano "data\quest\night_2026-09-29"
Add-Type -Namespace Phi -Name Power -MemberDefinition '[DllImport("kernel32.dll")] public static extern uint SetThreadExecutionState(uint f);'
[Phi.Power]::SetThreadExecutionState([uint32]"0x80000001") | Out-Null
try {
    Set-Location $phiace
    cargo run -p phiecs --release --example persona_quest -- $persona $root --reuse $night ollama run phi4
    Set-Location $phiano
    cargo run --release --bin persona_from_quest -- $root $persona
    foreach ($w in "cat", "bread", "doctor") { cargo run --release --bin relate_ask -- (Join-Path $root "naturalist\naturalist.relate") $w }
    $env:PHIANO_PERSONAS = Join-Path $root "personas.json"
    cargo run --release --bin persona_chat -- Naturalist "what is bread made of?"
}
finally {
    [Phi.Power]::SetThreadExecutionState([uint32]"0x80000000") | Out-Null
}
