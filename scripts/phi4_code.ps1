# The student of code learns from Phi-4 and asks what it does not know (docs/124).
#
#   powershell -ExecutionPolicy Bypass -File C:\Users\phiac\Workspace\gemphi\phinet\crates\phiano\scripts\phi4_code.ps1
#   ... phi4_code.ps1 -Parallel 2 -Rounds 3
#
# One unattended run:
# 1. `code_quest`: Phi-4 (temperature 0) answers the 253 tasks of 83 families
#    (resumes; answers already in data\phi4_code are kept).
# 2. The chain of requests, `-Rounds` times: `code_student --unknown` lists
#    the words of the task names told in fewer than 3 functions; `code_quest
#    --uses` has Phi-4 show each in three functions (uses\<word>\<fn>.rs);
#    the student learns them and asks again for what is still unknown.
# 3. `code_student` writes every task three ways — the task held out alone,
#    the family held out, the family held out with one answer shown — each
#    scored by rustc and the family's asserts beside Phi-4's own score.
# 4. `code_relations --held-out family`: the relation boards of docs/124.
param([int]$Parallel = 3, [int]$Seed = 7, [int]$Rounds = 3)
$ErrorActionPreference = "Continue"
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8
$env:Path = [Environment]::GetEnvironmentVariable("Path", "Machine") + ";" + [Environment]::GetEnvironmentVariable("Path", "User")
$phiano = Split-Path -Parent (Split-Path -Parent $MyInvocation.MyCommand.Path)
# phiano is a member of the phinet workspace: its binaries land in the workspace's target directory.
$target = (cargo metadata --no-deps --format-version 1 --manifest-path (Join-Path $phiano "Cargo.toml") | ConvertFrom-Json).target_directory
$out = Join-Path $phiano "data\phi4_code"
New-Item -ItemType Directory -Force $out | Out-Null
$modelfile = Join-Path $phiano "data\builders\phi4-builder.Modelfile"
"FROM phi4`nPARAMETER temperature 0`nPARAMETER seed $Seed" | Set-Content $modelfile -Encoding ascii
ollama create phi4-builder -f $modelfile
Add-Type -Namespace Phi -Name Power -MemberDefinition '[DllImport("kernel32.dll")] public static extern uint SetThreadExecutionState(uint f);'
[Phi.Power]::SetThreadExecutionState([uint32]"0x80000001") | Out-Null
try {
    Set-Location $phiano
    cargo build --release --bin code_quest --bin code_student --bin code_relations
    $quest = Join-Path $target "release\code_quest.exe"
    $student = Join-Path $target "release\code_student.exe"
    & $quest $out --parallel $Parallel ollama run phi4-builder
    for ($r = 1; $r -le $Rounds; $r++) {
        $words = Join-Path $out "unknown_$r.tsv"
        & $student --phi4 $out --unknown $words
        if ((Get-Content $words | Measure-Object -Line).Lines -eq 0) { break }
        & $quest $out --uses $words ollama run phi4-builder
    }
    & $student --phi4 $out --by-task --show 6 | Set-Content (Join-Path $out "student_by_task.md") -Encoding utf8
    & $student --phi4 $out --show 6 | Set-Content (Join-Path $out "student_by_family.md") -Encoding utf8
    & $student --phi4 $out --one-pointing --show 6 | Set-Content (Join-Path $out "student_one_pointing.md") -Encoding utf8
    $relations = Join-Path $target "release\code_relations.exe"
    & $relations $out --held-out family --r 8,16,32,64 --epochs 60 --seed $Seed --md (Join-Path $out "relations_by_family.md") | Out-Null
    Get-ChildItem $out -Filter "*.md" | ForEach-Object { "== $($_.Name)"; Get-Content $_.FullName | Select-Object -First 8 }
}
finally {
    [Phi.Power]::SetThreadExecutionState([uint32]"0x80000000") | Out-Null
}
