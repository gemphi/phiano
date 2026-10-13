# Overnight learning from Phi-4 (docs/96).
#
#   powershell -ExecutionPolicy Bypass -File C:\Users\phiac\Workspace\gemphi\phinet\crates\phiano\scripts\overnight.ps1
#
# 1. Keeps the PC awake while it runs.
# 2. Asks Phi-4 one kind at a time from several seed words, one quest each,
#    every answer saved as it arrives (a crash or Ctrl+C loses nothing).
# 3. Merges every table (tonight's and phi4_laws) into all.tsv.
# 4. Trains kind_operator on it with four seeds and writes summary.txt.
# Everything goes to data\quest\night_<date>\ ; the log is night.log.
# Edit $Seeds / $PerSeed to change how long it runs (~30 s a question).

param(
    [string[]]$Seeds = @("bread", "river", "hammer", "doctor", "tree", "car", "computer", "music"),
    [int]$PerSeed = 110,
    [string]$Kinds = "is-a,has,part-of,made-of,used-for,does"
)

$ErrorActionPreference = "Continue"
$env:Path = [Environment]::GetEnvironmentVariable("Path", "Machine") + ";" + [Environment]::GetEnvironmentVariable("Path", "User")
$phiano = Split-Path -Parent (Split-Path -Parent $MyInvocation.MyCommand.Path)
$phiace = Join-Path (Split-Path -Parent (Split-Path -Parent (Split-Path -Parent $phiano))) "phiace"
$night = Join-Path $phiano ("data\quest\night_" + (Get-Date -Format "yyyy-MM-dd"))
New-Item -ItemType Directory -Force -Path $night | Out-Null
Start-Transcript -Path (Join-Path $night "night.log") -Append | Out-Null

# Keep the machine awake (not the screen) until the script ends.
Add-Type -Namespace Phi -Name Power -MemberDefinition '[DllImport("kernel32.dll")] public static extern uint SetThreadExecutionState(uint f);'
[Phi.Power]::SetThreadExecutionState([uint32]"0x80000001") | Out-Null

function Stamp($m) { Write-Host ("[{0}] {1}" -f (Get-Date -Format "HH:mm:ss"), $m) }

try {
    Stamp "night in $night · seeds $($Seeds -join ', ') · $PerSeed questions each · kinds $Kinds"
    ollama --version
    Set-Location $phiace
    $env:PHIQUEST_KINDS = $Kinds
    foreach ($seed in $Seeds) {
        $out = Join-Path $night $seed
        if (Test-Path (Join-Path $out "relations.tsv")) { Stamp "$seed already done, skipping"; continue }
        Stamp "quest from $seed"
        $env:PHIQUEST_OUT = $out
        cargo run -p phiecs --release --example quest -- $seed 2 $PerSeed ollama run phi4
        Stamp "quest from $seed ended (exit $LASTEXITCODE)"
    }

    Stamp "merging tables"
    $tables = @(Get-ChildItem -Path $night -Recurse -Filter relations.tsv) + @(Get-Item (Join-Path $phiano "data\quest\phi4_laws\relations.tsv") -ErrorAction SilentlyContinue)
    $rows = $tables | Where-Object { $_ } | ForEach-Object { Get-Content -Encoding UTF8 $_.FullName } | Where-Object { $_.Trim() -ne "" } | Sort-Object -Unique
    $all = Join-Path $night "all.tsv"
    [IO.File]::WriteAllLines($all, [string[]]$rows)
    Stamp "all.tsv: $($rows.Count) rows from $($tables.Count) tables"

    Set-Location $phiano
    $env:PHIANO_LAW_MIN = "0.3"
    foreach ($s in 7, 8, 9, 10) {
        $env:PHIANO_SEED = "$s"
        Stamp "kind_operator seed $s"
        cargo run --release --example kind_operator -- $all 60 fno,fno+closure 2>&1 | Tee-Object -FilePath (Join-Path $night "operator_seed$s.txt")
    }
    $lines = Get-ChildItem (Join-Path $night "operator_seed*.txt") | ForEach-Object {
        "== $($_.Name)"
        Select-String -Path $_.FullName -Pattern "rows \(|^prior|^fno|  held out" | ForEach-Object { $_.Line }
    }
    $lines | Set-Content (Join-Path $night "summary.txt")
    Stamp "done: $night\summary.txt"
}
finally {
    [Phi.Power]::SetThreadExecutionState([uint32]"0x80000000") | Out-Null
    Stop-Transcript | Out-Null
}
