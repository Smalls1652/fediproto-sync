[CmdletBinding()]
param(
    [Parameter(Position = 0)]
    [string]$RootDirectory = (Get-Location).Path,
    [Parameter(Position = 1)]
    [string[]]$OnlyProcess
)

class LexiconItem {
    [string]$Path
    [string[]]$SubPaths
    [string[]]$ExcludedIds
    [string]$OutPath

    LexiconItem() {
    }

    LexiconItem([string]$path, [string[]]$subPaths, [string[]]$excludedIds, [string]$outPath) {
        $this.Path = $path
        $this.SubPaths = $subPaths
        $this.ExcludedIds = $excludedIds
        $this.OutPath = $outPath
    }
}

function Generate-RustifiedName {
    [CmdletBinding()]
    param(
        [Parameter(Position = 0, Mandatory)]
        [string]$Name
    )

    $generatedNameBuilder = [System.Text.StringBuilder]::new()
    $nameChars = $Name.ToCharArray()

    foreach ($charItem in $nameChars) {
        if ([char]::IsUpper($charItem)) {
            $null = $generatedNameBuilder.Append("_")
            $null = $generatedNameBuilder.Append([char]::ToLower($charItem))
        }
        else {
            $null = $generatedNameBuilder.Append($charItem)
        }
    }

    return $generatedNameBuilder.ToString().TrimEnd()
}

function Generate-TypeComment {
    [CmdletBinding()]
    param(
        [Parameter(Position = 0, Mandatory)]
        [hashtable]$DefinitionItem,
        [Parameter(Position = 1, Mandatory)]
        [string]$RootId,
        [Parameter(Position = 2, Mandatory)]
        [string]$TypeName
    )

    $typeCommentBuilder = [System.Text.StringBuilder]::new("/*")

    $null = $typeCommentBuilder.AppendLine("    Type: $($TypeName)")
    $null = $typeCommentBuilder.AppendLine("    Id: $($RootId)#$($TypeName)")
    $null = $typeCommentBuilder.AppendLine("    Kind: $($DefinitionItem["type"])")
    $null = $typeCommentBuilder.AppendLine("    ")
    $null = $typeCommentBuilder.AppendLine("    Properties:")

    foreach ($propertyItemKey in $DefinitionItem["properties"].Keys) {
        $propertyItem = $DefinitionItem["properties"][$propertyItemKey]

        $rustifiedPropertyName = Generate-RustifiedName -Name $propertyItemKey

        $requirementString = switch ($propertyItemKey -in $DefinitionItem["required"]) {
            $true {
                "Required"
                break
            }

            Default {
                "Optional"
                break
            }
        }

        switch ($propertyItem["type"]) {
            "ref" {
                $null = $typeCommentBuilder.AppendLine("    - $($rustifiedPropertyName): $($propertyItem["ref"]) (JsonProperty: $($propertyItemKey)) [$($requirementString)]")
                break
            }

            "string" {
                if ($null -ne $propertyItem["format"] -and $propertyItem["format"] -eq "datetime") {
                    $null = $typeCommentBuilder.AppendLine("    - $($rustifiedPropertyName): datetime (JsonProperty: $($propertyItemKey)) [$($requirementString)]")
                }
                else {
                    $null = $typeCommentBuilder.AppendLine("    - $($rustifiedPropertyName): string (JsonProperty: $($propertyItemKey)) [$($requirementString)]")
                }
                break
            }

            "array" {
                if ($propertyItem["items"]["type"] -eq "ref") {
                    $null = $typeCommentBuilder.AppendLine("    - $($rustifiedPropertyName): $($propertyItem["items"]["ref"])[] (JsonProperty: $($propertyItemKey)) [$($requirementString)]")
                }
                elseif ($propertyItem["items"]["type"] -eq "string" -and $null -ne $propertyItem["items"]["format"] -and $propertyItem["items"]["format"] -eq "datetime") {
                    $null = $typeCommentBuilder.AppendLine("    - $($rustifiedPropertyName): datetime[] (JsonProperty: $($propertyItemKey)) [$($requirementString)]")
                }
                else {
                    $null = $typeCommentBuilder.AppendLine("    - $($rustifiedPropertyName): $($propertyItem["items"]["type"])[] (JsonProperty: $($propertyItemKey)) [$($requirementString)]")
                }
                break
            }

            Default {
                $null = $typeCommentBuilder.AppendLine("    - $($rustifiedPropertyName): $($propertyItem["type"])  (JsonProperty: $($propertyItemKey)) [$($requirementString)]")
                break
            }
        }
    }

    $null = $typeCommentBuilder.AppendLine("*/")

    return $typeCommentBuilder.ToString()
}

$rootDirectoryPathResolved = (Resolve-Path -Path $RootDirectory -ErrorAction "Stop").Path

$writeInfoSplat = @{
    "InformationAction" = "Continue";
}

$atprotoLibPath = Join-Path -Path $rootDirectoryPathResolved -ChildPath "atproto-lib"

$tempClonePath = Join-Path -Path ([System.IO.Path]::GetTempPath()) -ChildPath "atproto"

if (Test-Path -Path $tempClonePath) {
    Write-Warning "Removing existing temp clone at '$($tempClonePath)'."
    Remove-Item -Path $tempClonePath -Recurse -Force
}

git clone "https://github.com/bluesky-social/atproto.git" "$($tempClonePath)"

$lexiconsToGenerate = @(
    [LexiconItem]@{
        Path        = "app/bsky";
        SubPaths    = @(
            "actor",
            "embed",
            "feed",
            "graph",
            "labeler",
            "notification",
            "richtext",
            "video"
        );
        ExcludedIds = @(
            "app.bsky.actor.getPreferences",
            "app.bsky.actor.getProfile",
            "app.bsky.actor.profile",
            "app.bsky.actor.putPreferences",
            "app.bsky.feed.generator",
            "app.bsky.feed.like",
            "app.bsky.feed.repost",
            "app.bsky.graph.block",
            "app.bsky.graph.follow",
            "app.bsky.graph.list",
            "app.bsky.graph.listblock",
            "app.bsky.graph.listitem",
            "app.bsky.labeler.service"
        )
        OutPath     = "app_bsky";
    },
    [LexiconItem]@{
        Path        = "chat/bsky";
        SubPaths    = @(
            "actor",
            "convo",
            "moderation"
        );
        ExcludedIds = @();
        OutPath     = "chat_bsky";
    },
    [LexiconItem]@{
        Path        = "com/atproto";
        SubPaths    = @(
            "admin",
            "identity",
            "label",
            "moderation",
            "repo",
            "server",
            "sync"
        );
        ExcludedIds = @(
            "com.atproto.server.activateAccount",
            "com.atproto.server.deleteSession",
            "com.atproto.server.requestAccountDelete",
            "com.atproto.server.requestEmailConfirmation"
        );
        OutPath     = "com_atproto";
    }
)

if ($OnlyProcess.Count -gt 0) {
    $lexiconsToGenerate = $lexiconsToGenerate | Where-Object { $PSItem.Path -in $OnlyProcess }
}

try {
    $rootLexiconsPath = Join-Path -Path $tempClonePath -ChildPath "lexicons"

    foreach ($lexiconItem in $lexiconsToGenerate) {
        $sourcePath = Join-Path -Path $rootLexiconsPath -ChildPath $lexiconItem.Path
        $generatedPath = Join-Path -Path $atprotoLibPath -ChildPath "src/types/$($lexiconItem.OutPath)"

        foreach ($subPath in $lexiconItem.SubPaths) {
            $lexiconSubPath = Join-Path -Path $sourcePath -ChildPath $subPath
            $generatedLexiconSubPath = Join-Path -Path $generatedPath -ChildPath $subPath

            if (!(Test-Path -Path $generatedLexiconSubPath)) {
                Write-Warning "Creating directory for '$($subPath)' types in the '$($lexiconItem.OutPath)' module..."
                $null = New-Item -Path $generatedLexiconSubPath -ItemType "Directory" -Force
            }

            $rootDefsFile = Join-Path -Path $lexiconSubPath -ChildPath "defs.json"
            $generatedRootDefsFilePath = Join-Path -Path $generatedLexiconSubPath -ChildPath "defs.rs"

            if ((Test-Path -Path $rootDefsFile) -and !(Test-Path -Path $generatedRootDefsFilePath)) {
                $rootDefs = Get-Content -Path $rootDefsFile -Raw | ConvertFrom-Json -AsHashtable

                $generatedRootDefsFileBuilder = [System.Text.StringBuilder]::new()

                $null = $generatedRootDefsFileBuilder.AppendLine("use serde::{Deserialize, Serialize};")
                $null = $generatedRootDefsFileBuilder.AppendLine()

                foreach ($rootDefItemKey in $rootDefs["defs"].Keys) {
                    $rootDefItem = $rootDefs["defs"][$rootDefItemKey]

                    $null = $generatedRootDefsFileBuilder.AppendLine((Generate-TypeComment -DefinitionItem $rootDefItem -RootId $rootDefs["id"] -TypeName $rootDefItemKey))
                }

                $generatedRootDefsFileBuilder.ToString() | Out-File -FilePath $generatedRootDefsFilePath -Encoding "UTF8"
            }

            $defFiles = Get-ChildItem -Path $lexiconSubPath -Filter "*.json" -File | Where-Object { $PSItem.Name -notin @("defs.json") }

            foreach ($defFileItem in $defFiles) {
                $defItem = Get-Content -Path $defFileItem.FullName -Raw | ConvertFrom-Json -AsHashtable

                if ($defItem["id"] -in $lexiconItem.ExcludedIds) {
                    Write-Warning "Skipping '$($defItem["id"])'..."
                    continue
                }

                $generatedDefFileNameBuilder = [System.Text.StringBuilder]::new()
                $baseDefFileNameChars = $defFileItem.BaseName.ToCharArray()

                foreach ($charItem in $baseDefFileNameChars) {
                    if ([char]::IsUpper($charItem)) {
                        $null = $generatedDefFileNameBuilder.Append("_")
                        $null = $generatedDefFileNameBuilder.Append([char]::ToLower($charItem))
                    }
                    else {
                        $null = $generatedDefFileNameBuilder.Append($charItem)
                    }
                }

                $null = $generatedDefFileNameBuilder.Append(".rs")
                $generatedDefFileName = $generatedDefFileNameBuilder.ToString().TrimEnd()

                $generatedDefFilePath = Join-Path -Path $generatedLexiconSubPath -ChildPath $generatedDefFileName

                if (!(Test-Path -Path $generatedDefFilePath)) {
                    Write-Information @writeInfoSplat -MessageData "Generating template type file for '$($defItem.id)'..."

                    $generatedDefFileBuilder = [System.Text.StringBuilder]::new()
                    $null = $generatedDefFileBuilder.AppendLine("use serde::{Deserialize, Serialize};")
                    $null = $generatedDefFileBuilder.AppendLine()
                    $null = $generatedDefFileBuilder.AppendLine("/*")
                    $null = $generatedDefFileBuilder.AppendLine("    $($defItem["id"])")
                    $null = $generatedDefFileBuilder.AppendLine("*/")
                    $null = $generatedDefFileBuilder.AppendLine()

                    $defItemMain = $defItem["defs"]["main"]
                    
                    if ("input" -in $defItemMain.Keys) {
                        if ($null -ne $defItemMain["input"] -and $null -ne $defItemMain["input"]["schema"] -and $defItemMain["input"]["encoding"] -eq "application/json") {
                            $null = $generatedDefFileBuilder.AppendLine((Generate-TypeComment -DefinitionItem $defItemMain["input"]["schema"] -RootId $defItem["id"] -TypeName "request"))
                        }
                    }

                    if ("output" -in $defItemMain.Keys) {
                        if ($null -ne $defItemMain["output"] -and $null -ne $defItemMain["output"]["schema"] -and $defItemMain["output"]["encoding"] -eq "application/json") {
                            $null = $generatedDefFileBuilder.AppendLine((Generate-TypeComment -DefinitionItem $defItemMain["output"]["schema"] -RootId $defItem["id"] -TypeName "response"))
                        }
                    }

                    $nonMainKeys = $defItem["defs"].Keys | Where-Object { $PSItem -ne "main" }
                    foreach ($defKeyItem in $nonMainKeys) {
                        $defItemSub = $defItem["defs"][$defKeyItem]

                        $null = $generatedDefFileBuilder.AppendLine((Generate-TypeComment -DefinitionItem $defItemSub -RootId $defItem["id"] -TypeName $defKeyItem))
                    }
                    
                    $generatedDefFileBuilder.ToString() | Out-File -FilePath $generatedDefFilePath -Encoding "UTF8"
                }
            }
        }
    }
}
finally {
    Write-Warning "Cleaning up temp clone at '$($tempClonePath)'..."
    Remove-Item -Path $tempClonePath -Recurse -Force
}
