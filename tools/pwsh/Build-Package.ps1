[CmdletBinding()]
param(
    [Parameter(Position = 0, Mandatory)]
    [ValidateNotNullOrEmpty()]
    [string]$PackageName,
    [Parameter(Position = 1)]
    [ValidateSet(
        "Debug",
        "Release"
    )]
    [string]$BuildProfile = "Debug"
)

$cargoBuildArgs = [System.Collections.Generic.List[string]]::new()

$cargoBuildArgs.Add("build")
$cargoBuildArgs.Add("--package")
$cargoBuildArgs.Add($PackageName)

if ($BuildProfile -eq "Release") {
    $cargoBuildArgs.Add("--release")
}

$cargoBuildProcSplat = @{
    "FilePath" = "cargo";
    "ArgumentList" = $cargoBuildArgs;
    "NoNewWindow" = $true;
    "Wait" = $true;
    "ErrorAction" = "Stop";
}
Start-Process @cargoBuildProcSplat
