$OutputEncoding = [Console]::OutputEncoding = [Text.UTF8Encoding]::new($false)
Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$repoRoot = Split-Path -Parent $scriptDir
$devStateDir = Join-Path $repoRoot ".local-dev"
$watchPidFile = Join-Path $devStateDir "leptos-watch.pid"

function Get-LocalServerProcesses {
    return @(
        Get-Process server -ErrorAction SilentlyContinue |
            ForEach-Object {
                [PSCustomObject]@{
                    Id = $_.Id
                    Path = $_.Path
                }
            }
    )
}

function Get-LeptosWatchProcesses {
    return @(
        Get-CimInstance Win32_Process -Filter "Name = 'cargo.exe'" -ErrorAction SilentlyContinue |
            Where-Object {
                $_.CommandLine -and
                $_.CommandLine -like "*leptos*watch*" -and
                $_.CommandLine -like "*$repoRoot*"
            } |
            ForEach-Object {
                [PSCustomObject]@{
                    Id = $_.ProcessId
                    CommandLine = $_.CommandLine
                }
            }
    )
}

function Stop-IfRunning {
    param(
        [int]$ProcessId,
        [string]$Label
    )

    $process = Get-Process -Id $ProcessId -ErrorAction SilentlyContinue
    if ($null -ne $process) {
        Write-Host " - PID $ProcessId $Label" -ForegroundColor Yellow
        Stop-Process -Id $ProcessId -Force
        return $true
    }

    return $false
}

$stoppedAny = $false

if (Test-Path -LiteralPath $watchPidFile) {
    $watchPidText = Get-Content -LiteralPath $watchPidFile -Raw -Encoding UTF8
    $watchPidValue = 0
    if ([int]::TryParse($watchPidText.Trim(), [ref]$watchPidValue)) {
        Write-Host "Stopping cargo leptos watch from PID file:" -ForegroundColor Yellow
        if (Stop-IfRunning -ProcessId $watchPidValue -Label "(from pid file)") {
            $stoppedAny = $true
        }
    }

    Remove-Item -LiteralPath $watchPidFile -Force
}

$runningWatchers = @(Get-LeptosWatchProcesses)
if ($runningWatchers.Count -gt 0) {
    Write-Host "Stopping cargo leptos watch process(es):" -ForegroundColor Yellow
    $runningWatchers | ForEach-Object {
        Write-Host " - PID $($_.Id) $($_.CommandLine)" -ForegroundColor Yellow
        Stop-Process -Id $_.Id -Force
        $stoppedAny = $true
    }
}

$runningServers = @(Get-LocalServerProcesses)

if ($runningServers.Count -gt 0) {
    Write-Host "Stopping local server process(es):" -ForegroundColor Yellow
    $runningServers | ForEach-Object {
        Write-Host " - PID $($_.Id) $($_.Path)" -ForegroundColor Yellow
        Stop-Process -Id $_.Id -Force
        $stoppedAny = $true
    }
}

if (-not $stoppedAny) {
    Write-Host "No local cargo leptos watch or server.exe process is running."
    exit 0
}

Write-Host "Local site stack has been stopped." -ForegroundColor Green
