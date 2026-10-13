# Teaching for hours (docs/132). One unattended run with a time budget;
# everything it learns is files, so a stopped run resumes where it was.
#
#   powershell -ExecutionPolicy Bypass -File C:\Users\phiac\Workspace\gemphi\phinet\crates\phiano\scripts\teach.ps1 -Hours 8
#
# Each loop, until the hours are spent:
# 1. the teacher goes on with the story, five paragraphs (`story --teach`):
#    before each, the student's own going-on is held against the teacher's
#    (agreement, D20) and logged to data\text\story\teach.log;
# 2. the story's words told in fewer than three paragraphs are asked for
#    (`story --unknown` / `--uses`), told as data\text\uses\<word>.txt;
# 3. the code student's unknown words likewise (`code_student --unknown` /
#    `code_quest --uses`), first filling any task Phi-4 has not answered;
# 4. every fourth loop, the measurements: the context tests (fast) and the
#    code student by task, appended to data\teach_runs.md with the time.
param([double]$Hours = 8, [int]$Seed = 7)
$ErrorActionPreference = "Continue"
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8
$env:Path = [Environment]::GetEnvironmentVariable("Path", "Machine") + ";" + [Environment]::GetEnvironmentVariable("Path", "User")
$phiano = Split-Path -Parent (Split-Path -Parent $MyInvocation.MyCommand.Path)
# phiano is a member of the phinet workspace: its binaries land in the workspace's target directory.
$target = (cargo metadata --no-deps --format-version 1 --manifest-path (Join-Path $phiano "Cargo.toml") | ConvertFrom-Json).target_directory
$text = Join-Path $phiano "data\text"
$code = Join-Path $phiano "data\phi4_code"
$runs = Join-Path $phiano "data\teach_runs.md"
$modelfile = Join-Path $phiano "data\builders\phi4-builder.Modelfile"
"FROM phi4`nPARAMETER temperature 0`nPARAMETER seed $Seed" | Set-Content $modelfile -Encoding ascii
ollama create phi4-builder -f $modelfile
Add-Type -Namespace Phi -Name Power -MemberDefinition '[DllImport("kernel32.dll")] public static extern uint SetThreadExecutionState(uint f);'
[Phi.Power]::SetThreadExecutionState([uint32]"0x80000001") | Out-Null
try {
    Set-Location $phiano
    cargo build --release --example story --example code_student --example code_quest --example context
    $story = Join-Path $target "release\story.exe"
    $student = Join-Path $target "release\code_student.exe"
    $quest = Join-Path $target "release\code_quest.exe"
    $context = Join-Path $target "release\context.exe"
    $teacher = @("ollama", "run", "phi4-builder")
    $stop = (Get-Date).AddHours($Hours)
    $loop = 0
    if (-not (Test-Path $runs)) { "# Teaching runs (docs/132)`n" | Set-Content $runs -Encoding utf8 }
    "`n## Run started $(Get-Date -Format s), budget $Hours h`n" | Add-Content $runs -Encoding utf8
    while ((Get-Date) -lt $stop) {
        $loop++
        "— loop $loop at $(Get-Date -Format T)"
        & $story $text --in text\story --teach 5 @teacher
        $words = Join-Path $text "unknown_story.tsv"
        & $story $text --in text\story --unknown $words
        if ((Get-Content $words | Measure-Object -Line).Lines -gt 0) { & $story $text --uses $words @teacher }
        & $quest $code --parallel 2 @teacher
        $cwords = Join-Path $code "unknown_code.tsv"
        & $student --phi4 $code --unknown $cwords
        if ((Get-Content $cwords | Measure-Object -Line).Lines -gt 0) { & $quest $code --uses $cwords @teacher }
        if ($loop % 4 -eq 1) {
            "### $(Get-Date -Format s), loop $loop" | Add-Content $runs -Encoding utf8
            $paragraphs = (Get-Content (Join-Path $text "story\teach.log") | Measure-Object -Line).Lines
            $last = Get-Content (Join-Path $text "story\teach.log") | Select-Object -Last 5
            "story: $paragraphs paragraphs taught; last five (file, words, overlap, agreed from start): " | Add-Content $runs -Encoding utf8
            $last | ForEach-Object { "    " + ($_ -split "`t")[0..3] -join " · " } | Add-Content $runs -Encoding utf8
            & $context --turns 400 --seed $Seed | Select-Object -Last 2 | Add-Content $runs -Encoding utf8
            & $student --phi4 $code --by-task --show 0 | Select-Object -First 6 | Add-Content $runs -Encoding utf8
        }
    }
    "`n## Run ended $(Get-Date -Format s) after $loop loops`n" | Add-Content $runs -Encoding utf8
    Get-Content $runs | Select-Object -Last 30
}
finally {
    [Phi.Power]::SetThreadExecutionState([uint32]"0x80000000") | Out-Null
}
