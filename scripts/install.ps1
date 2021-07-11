#!/usr/bin/env pwsh

$ErrorActionPreference = 'Stop'

$ZipUrl = "https://github.com/shreyashsaitwal/recast/releases/latest/download/recast-x86_64-pc-windows-msvc.zip"

# Required by GitHub
[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12

$DataDir = "$HOME/.recast"
if (!(Test-Path $DataDir)) {
    New-Item $DataDir -ItemType Directory | Out-Null
}

$ZipFile = "$DataDir/recast-x86_64-pc-windows-msvc.zip"

# Download ZIP
Invoke-WebRequest -OutFile "$ZipFile" $ZipUrl -UseBasicParsing

# Extract it
if (Get-Command Expand-Archive -ErrorAction SilentlyContinue) {
    Expand-Archive "$ZipFile" -DestinationPath "$DataDir" -Force
}
else {
    Add-Type -AssemblyName System.IO.Compression.FileSystem
    [IO.Compression.ZipFile]::ExtractToDirectory("$ZipFile", $DataDir)
}
Remove-Item "$ZipFile"

$User = [EnvironmentVariableTarget]::User
$Path = [Environment]::GetEnvironmentVariable("Path", $User)

# Update PATH
if (!(";$Path;".ToLower() -like "*;$DataDir/bin;*".ToLower())) {
    [Environment]::SetEnvironmentVariable("Path", "$Path;$DataDir/bin", $User)
    $Env:Path += ";$DataDir/bin"
}

Write-Output "Success! Installed Recast at $DataDir\bin\recast.exe!"
Write-Output "Run 'recast --help' to get started.`n"
