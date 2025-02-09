[CmdletBinding()]
param(
    [Parameter(Position = 0)]
    [string]$ArtifactsDirPath = (Get-Location).Path
)

$resolvedPath = (Resolve-Path -Path $ArtifactsDirPath -ErrorAction "Stop").Path

if ([System.IO.FileAttributes]::Directory -notin (Get-Item -Path $resolvedPath).Attributes) {
    $PSCmdlet.ThrowTerminatingError(
        [System.Management.Automation.ErrorRecord]::new(
            [System.IO.IOException]::new("Specified path is not a directory."),
            "PathNotDirectory",
            [System.Management.Automation.ErrorCategory]::InvalidArgument,
            $resolvedPath
        )
    )
}

$platformMap = @(
    [pscustomobject]@{
        "Platform" = "Linux (x64/amd64)";
        "PlatformId" = "linux-amd64";
        "PlatformMarkdownText" = "Linux (``x64/amd64``)";
    },
    [pscustomobject]@{
        "Platform" = "Linux (arm64)";
        "PlatformId" = "linux-arm64";
        "PlatformMarkdownText" = "Linux (``arm64``)";
    },
    [pscustomobject]@{
        "Platform" = "Windows (x64/amd64)";
        "PlatformId" = "windows-amd64";
        "PlatformMarkdownText" = "Windows (``x64/amd64``)";
    },
    [pscustomobject]@{
        "Platform" = "Windows (arm64)";
        "PlatformId" = "windows-arm64";
        "PlatformMarkdownText" = "Windows (``arm64``)";
    },
    [pscustomobject]@{
        "Platform" = "macOS (x64/amd64)";
        "PlatformId" = "macOS-amd64";
        "PlatformMarkdownText" = "macOS (``x64/amd64``)";
    },
    [pscustomobject]@{
        "Platform" = "macOS (arm64)";
        "PlatformId" = "macos-arm64";
        "PlatformMarkdownText" = "macOS (``arm64``)";
    }
)

$artifactFiles = Get-ChildItem -Path $resolvedPath | Where-Object { $PSItem.Extension -eq ".zip" }

$markdownStringBuilder = [System.Text.StringBuilder]::new()

$null = $markdownStringBuilder.AppendLine("| Platform | SHA256 hash |")
$null = $markdownStringBuilder.AppendLine("| --- | --- |")

foreach ($platform in $platformMap) {
    $platformFile = $artifactFiles | Where-Object { $PSItem.Name -like "*_$($platform.PlatformId)_*" }

    if ($null -ne $platformFile) {
        Write-Verbose "Getting file hash for '$($platformFile.Name)'"
        $fileHash = (Get-FileHash -Path $platformFile.FullName -Algorithm "SHA256").Hash.ToLower()

        $null = $markdownStringBuilder.AppendLine("| $($platform.PlatformMarkdownText) | ``$($fileHash)`` |")
    }
}

$markdownStringBuilder.ToString()
