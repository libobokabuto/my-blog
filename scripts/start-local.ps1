param(
    [string]$SiteUrl,
    [string]$DatabaseUrl,
    [string]$RedisUrl,
    [string]$RustLog,
    [switch]$CheckOnly
)

$OutputEncoding = [Console]::OutputEncoding = [Text.UTF8Encoding]::new($false)
Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$repoRoot = Split-Path -Parent $scriptDir
$envCandidates = @(
    (Join-Path $repoRoot ".env.local"),
    (Join-Path $repoRoot ".env")
)

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

function Import-EnvFile {
    param([string]$Path)

    if (-not (Test-Path -LiteralPath $Path)) {
        return $false
    }

    Get-Content -LiteralPath $Path -Encoding UTF8 | ForEach-Object {
        $line = $_.Trim()
        if (-not $line -or $line.StartsWith("#")) {
            return
        }

        $parts = $line -split "=", 2
        if ($parts.Count -ne 2) {
            return
        }

        $name = $parts[0].Trim()
        $value = $parts[1].Trim()

        if (
            ($value.StartsWith('"') -and $value.EndsWith('"')) -or
            ($value.StartsWith("'") -and $value.EndsWith("'"))
        ) {
            $value = $value.Substring(1, $value.Length - 2)
        }

        Set-Item -Path "Env:$name" -Value $value
    }

    return $true
}

$loadedEnvFile = $null
foreach ($candidate in $envCandidates) {
    if (Import-EnvFile -Path $candidate) {
        $loadedEnvFile = $candidate
        break
    }
}

if ($PSBoundParameters.ContainsKey("SiteUrl")) {
    $env:SITE_URL = $SiteUrl
}

if ($PSBoundParameters.ContainsKey("DatabaseUrl")) {
    $env:BLOG_DATABASE_URL = $DatabaseUrl
}

if ($PSBoundParameters.ContainsKey("RedisUrl")) {
    $env:BLOG_REDIS_URL = $RedisUrl
}

if ($PSBoundParameters.ContainsKey("RustLog")) {
    $env:RUST_LOG = $RustLog
} elseif (-not $env:RUST_LOG) {
    $env:RUST_LOG = "server=info,tower_http=info"
}

$requiredVars = @("SITE_URL", "BLOG_DATABASE_URL", "BLOG_REDIS_URL")
$missingVars = @(
    $requiredVars | Where-Object { -not (Get-Item -Path "Env:$_" -ErrorAction SilentlyContinue) }
)

if ($missingVars.Count -gt 0) {
    Write-Host "Missing required environment variables: $($missingVars -join ', ')" -ForegroundColor Red
    Write-Host "Options:" -ForegroundColor Yellow
    Write-Host "1. Create .env.local in the repo root" -ForegroundColor Yellow
    Write-Host "2. Or pass values directly, for example:" -ForegroundColor Yellow
    Write-Host "   .\\scripts\\start-local.ps1 -DatabaseUrl 'mysql://root:your-password@127.0.0.1:3306/my-blog' -RedisUrl 'redis://127.0.0.1:6379/' -SiteUrl 'http://127.0.0.1:3000'" -ForegroundColor Yellow
    exit 1
}

if ($env:BLOG_DATABASE_URL -match "your-password") {
    Write-Host "BLOG_DATABASE_URL still contains the example placeholder. Please set a real value." -ForegroundColor Red
    exit 1
}

$runningServers = @(Get-LocalServerProcesses)

if ($runningServers.Count -gt 0) {
    Write-Host "Stopping existing local server process(es):" -ForegroundColor Yellow
    $runningServers | ForEach-Object {
        Write-Host " - PID $($_.Id) $($_.Path)" -ForegroundColor Yellow
        Stop-Process -Id $_.Id -Force
    }
}

Write-Host "Repo root: $repoRoot"
if ($loadedEnvFile) {
    Write-Host "Loaded env file: $loadedEnvFile"
} else {
    Write-Host "No .env.local or .env found. Using current shell or command arguments."
}

Write-Host "SITE_URL=$env:SITE_URL"
Write-Host "BLOG_DATABASE_URL is set"
Write-Host "BLOG_REDIS_URL=$env:BLOG_REDIS_URL"
Write-Host "RUST_LOG=$env:RUST_LOG"

if ($CheckOnly) {
    Write-Host "Environment check completed. Server was not started." -ForegroundColor Green
    exit 0
}

Set-Location -LiteralPath $repoRoot
& cargo run -p server --bin server
