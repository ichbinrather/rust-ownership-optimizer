<#
.SYNOPSIS
    Installs the rust-ownership-optimizer skill for every supported AI coding agent
    found on this Windows machine.

.DESCRIPTION
    Run from a cloned repository:
        .\install.ps1
    Or run directly from GitHub (downloads the repository archive first):
        & ([scriptblock]::Create((irm https://raw.githubusercontent.com/ichbinrather/rust-ownership-optimizer/main/install.ps1)))

.PARAMETER Link
    Create directory junctions instead of copies, so `git pull` updates the skill.
    Junctions do not need administrator rights or Developer Mode.

.PARAMETER All
    Install for every agent, even if its configuration folder does not exist yet.

.PARAMETER Force
    Replace an existing installation.
#>
[CmdletBinding()]
param(
    [switch]$Link,
    [switch]$All,
    [switch]$Force
)

$ErrorActionPreference = 'Stop'
$Name = 'rust-ownership-optimizer'
$Repo = 'ichbinrather/rust-ownership-optimizer'
$Branch = 'main'

# Locate the skill source: the folder of this script, or a fresh download.
$Source = $null
if ($PSScriptRoot -and (Test-Path (Join-Path $PSScriptRoot 'SKILL.md'))) {
    $Source = $PSScriptRoot
} else {
    if ($Link) {
        throw '-Link needs a cloned repository. Clone it first, then run .\install.ps1 -Link'
    }
    $tmp = Join-Path ([System.IO.Path]::GetTempPath()) ("$Name-" + [guid]::NewGuid().ToString('N'))
    New-Item -ItemType Directory -Path $tmp | Out-Null
    $zip = Join-Path $tmp 'repo.zip'
    $url = "https://github.com/$Repo/archive/refs/heads/$Branch.zip"
    Write-Host "Downloading $url"
    [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
    Invoke-WebRequest -Uri $url -OutFile $zip -UseBasicParsing
    Expand-Archive -Path $zip -DestinationPath $tmp
    $Source = (Get-ChildItem -Path $tmp -Directory | Select-Object -First 1).FullName
}

$HomeDir = $env:USERPROFILE
if (-not $HomeDir) { $HomeDir = $HOME }

# Each entry: agent config folder, skills folder.
$Targets = @(
    @{ Base = (Join-Path $HomeDir '.claude');           Skills = (Join-Path $HomeDir '.claude\skills') },
    @{ Base = (Join-Path $HomeDir '.codex');            Skills = (Join-Path $HomeDir '.codex\skills') },
    @{ Base = (Join-Path $HomeDir '.agents');           Skills = (Join-Path $HomeDir '.agents\skills') },
    @{ Base = (Join-Path $HomeDir '.config\opencode');  Skills = (Join-Path $HomeDir '.config\opencode\skills') },
    @{ Base = (Join-Path $HomeDir '.cursor');           Skills = (Join-Path $HomeDir '.cursor\skills') },
    @{ Base = (Join-Path $HomeDir '.gemini');           Skills = (Join-Path $HomeDir '.gemini\skills') },
    @{ Base = (Join-Path $HomeDir '.copilot');          Skills = (Join-Path $HomeDir '.copilot\skills') }
)

$Items = @('SKILL.md', 'AGENTS.md', 'references', 'examples', 'scripts', 'agents')
$installed = 0

foreach ($t in $Targets) {
    if (-not $All -and -not (Test-Path $t.Base)) { continue }

    New-Item -ItemType Directory -Path $t.Skills -Force | Out-Null
    $dest = Join-Path $t.Skills $Name

    if (Test-Path $dest) {
        if (-not $Force) {
            Write-Host "skip   $dest (already exists; use -Force to replace)"
            continue
        }
        $item = Get-Item $dest -Force
        if ($item.Attributes -band [IO.FileAttributes]::ReparsePoint) {
            # Remove only the junction, never the files it points to.
            $item.Delete()
        } else {
            Remove-Item -Path $dest -Recurse -Force
        }
    }

    if ($Link) {
        New-Item -ItemType Junction -Path $dest -Target $Source | Out-Null
    } else {
        New-Item -ItemType Directory -Path $dest | Out-Null
        foreach ($i in $Items) {
            $p = Join-Path $Source $i
            if (Test-Path $p) { Copy-Item -Path $p -Destination $dest -Recurse }
        }
    }
    Write-Host "ok     $dest"
    $installed++
}

Write-Host "Installed for $installed agent(s). Restart your agent to load the skill."
if ($installed -eq 0) {
    Write-Host 'No agent folders found. Run again with -All to create them.'
}
