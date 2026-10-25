# The story student asks Phi-4 for the words it does not know (docs/130).
#
#   powershell -ExecutionPolicy Bypass -File C:\Users\phiac\Workspace\gemphi\phinet\crates\phiano\scripts\phi4_story.ps1
#   ... phi4_story.ps1 -Rounds 3 -Seed 7
#
# One unattended run. The tree is data\text (Emma + the story). Each round:
# `story --unknown` lists the story's words said in fewer than 3 paragraphs
# of the whole tree; `story --uses` has Phi-4 (temperature 0) show each in
# three sentences, saved as data\text\uses\<word>.txt and told with the rest;
# the next round asks again for what is still unknown. Before and after,
# the story is gone on from (`--go … --beyond`) into data\text\story_go.md.
param([int]$Rounds = 3, [int]$Seed = 7)
$ErrorActionPreference = "Continue"
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8
$env:Path = [Environment]::GetEnvironmentVariable("Path", "Machine") + ";" + [Environment]::GetEnvironmentVariable("Path", "User")
$phiano = Split-Path -Parent (Split-Path -Parent $MyInvocation.MyCommand.Path)
# phiano is a member of the phinet workspace: its binaries land in the workspace's target directory.
$target = (cargo metadata --no-deps --format-version 1 --manifest-path (Join-Path $phiano "Cargo.toml") | ConvertFrom-Json).target_directory
$text = Join-Path $phiano "data\text"
$modelfile = Join-Path $phiano "data\builders\phi4-builder.Modelfile"
"FROM phi4`nPARAMETER temperature 0`nPARAMETER seed $Seed" | Set-Content $modelfile -Encoding ascii
ollama create phi4-builder -f $modelfile
Add-Type -Namespace Phi -Name Power -MemberDefinition '[DllImport("kernel32.dll")] public static extern uint SetThreadExecutionState(uint f);'
[Phi.Power]::SetThreadExecutionState([uint32]"0x80000001") | Out-Null
try {
    Set-Location $phiano
    cargo build --release --bin story
    $story = Join-Path $target "release\story.exe"
    $out = Join-Path $text "story_go.md"
    "# The story going on, before and after asking (docs/130)`n`n## Before`n" | Set-Content $out -Encoding utf8
    foreach ($q in "She suddenly", "He", "The investment") {
        "### $q`n" | Add-Content $out -Encoding utf8
        & $story $text --in text\story --go $q --words 90 --beyond | Select-Object -Skip 6 | Add-Content $out -Encoding utf8
    }
    for ($r = 1; $r -le $Rounds; $r++) {
        $words = Join-Path $text "unknown_$r.tsv"
        & $story $text --in text\story --unknown $words
        if ((Get-Content $words | Measure-Object -Line).Lines -eq 0) { break }
        & $story $text --uses $words ollama run phi4-builder
    }
    "`n## After`n" | Add-Content $out -Encoding utf8
    foreach ($q in "She suddenly", "He", "The investment") {
        "### $q`n" | Add-Content $out -Encoding utf8
        & $story $text --in text\story --go $q --words 90 --beyond | Select-Object -Skip 6 | Add-Content $out -Encoding utf8
    }
    Get-Content $out
}
finally {
    [Phi.Power]::SetThreadExecutionState([uint32]"0x80000000") | Out-Null
}
