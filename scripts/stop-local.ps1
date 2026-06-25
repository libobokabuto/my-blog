$OutputEncoding = [Console]::OutputEncoding = [Text.UTF8Encoding]::new($false)
Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$repoRoot = Split-Path -Parent $scriptDir

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

$runningServers = @(Get-LocalServerProcesses)

if ($runningServers.Count -eq 0) {
    Write-Host "No local server.exe process is running."
    exit 0
}

Write-Host "Stopping local server process(es):" -ForegroundColor Yellow
$runningServers | ForEach-Object {
    Write-Host " - PID $($_.Id) $($_.Path)" -ForegroundColor Yellow
    Stop-Process -Id $_.Id -Force
}

Write-Host "Local server has been stopped." -ForegroundColor Green
