# The tree of knowing, live (docs/134). No GPU, no Phi-4: everything here
# is told from files on disk and read by counts.
#
#   powershell -ExecutionPolicy Bypass -File C:\Users\phiac\Workspace\gemphi\phinet\crates\phiano\scripts\demo_tree.ps1
#   ... demo_tree.ps1 -Quick     (skips the code student: about 2 minutes)
#
# 1. The story as a tree: two paragraphs on disk, the rows they make, and
#    the student saying the story back under its own scope.
# 2. The language in the tree: Emma (158k words) and the story under one
#    root; the story gone on with the language, past where it ends.
# 3. Our own context tests: a 2,000-turn session as a tree; needles, keys
#    and updated keys read after everything else was said; nothing dropped.
# 4. The code student on the base: a function written from Phi-4's saved
#    answers by movement and proportion, with the compiler as `worse`.
param([switch]$Quick)
$ErrorActionPreference = "Continue"
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8
$env:Path = [Environment]::GetEnvironmentVariable("Path", "Machine") + ";" + [Environment]::GetEnvironmentVariable("Path", "User")
$phiano = Split-Path -Parent (Split-Path -Parent $MyInvocation.MyCommand.Path)
# phiano is a member of the phinet workspace: its binaries land in the workspace's target directory.
$target = (cargo metadata --no-deps --format-version 1 --manifest-path (Join-Path $phiano "Cargo.toml") | ConvertFrom-Json).target_directory
Set-Location $phiano
cargo build --release --bin story --bin context --bin code_student 2>&1 | Select-Object -Last 1
$story = Join-Path $target "release\story.exe"
$context = Join-Path $target "release\context.exe"
$student = Join-Path $target "release\code_student.exe"
$text = Join-Path $phiano "data\text"

function Say($s) { Write-Host ""; Write-Host "== $s" -ForegroundColor Cyan }

Say "1. The story as a tree (data\story\ch1.txt: one chapter, two paragraphs)"
Get-Content (Join-Path $phiano "data\story\ch1.txt")
Say "the rows the reader makes of one word (suddenly, second paragraph)"
Select-String -Path (Join-Path $phiano "data\story\rows.tsv") -Pattern "^suddenly@story/ch1/p2#1" | ForEach-Object { $_.Line }
Say "the student says the story back from two words, and crosses paragraphs where the words draw it"
& $story (Join-Path $phiano "data\story") --go "She suddenly" --words 40 --quiet
& $story (Join-Path $phiano "data\story") --go "He" --words 40 --quiet

Say "2. The language in the tree (data\text: Emma, 55 chapters, and the story under one root)"
& $story $text --in text\story --go "She suddenly" --words 100 --beyond --quiet | Select-Object -First 3
Say "inside Emma's first chapter it says the chapter back"
& $story $text --in text\emma\ch01 --go "Emma Woodhouse" --words 40 --quiet | Select-Object -First 1
Say "what the story does not yet know (said in fewer than 3 paragraphs of the whole tree) - what it would ask Phi-4 for"
& $story $text --in text\story --unknown (Join-Path $env:TEMP "unknown.tsv") --quiet

Say "3. Our own context tests: a session kept as a tree, nothing dropped, nothing summarised"
& $context --turns 400,2000 --seed 7 | Select-Object -Last 4

if (-not $Quick) {
    Say "4. The code student on the base: a new function from Phi-4's saved answers"
    $code = Join-Path $phiano "data\phi4_code"
    Say "a known task in a type it never saw (i128): the type sibling said again"
    & $student --phi4 $code --ask "fn count_odd(v: &[i128]) -> usize"
    Say "the literal exchanged with the type (0 stays 0 for u64; it would be 0.0 for f64)"
    & $student --phi4 $code --ask "fn count_zero(v: &[u64]) -> usize"
    Say "a word it cannot yet read (odd, under max): it compiles and is wrong - the honest limit; the chain of requests is what fixes it"
    & $student --phi4 $code --ask "fn max_odd(v: &[i32]) -> Option<i32>"
}
Say "done"
