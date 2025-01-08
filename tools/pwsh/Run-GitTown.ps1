[CmdletBinding()]
param(
    [Parameter(Position = 0)]
    [ValidateSet(
        "switch",
        "sync",
        "sync-all",
        "hack",
        "hack-prototype"
    )]
    [string]$ActionName = "switch"
)

switch ($ActionName) {
    "switch" {
        git town switch
        break
    }

    "sync" {
        git town sync
        break
    }

    "sync-all" {
        git town sync --all
        break
    }

    "hack" {
        $branchName = Read-Host -Prompt "Enter the name of the new branch"

        if ($null -eq $branchName -or [string]::IsNullOrWhiteSpace($branchName)) {
            $PSCmdlet.ThrowTerminatingError(
                [System.Management.Automation.ErrorRecord]::new(
                    [System.ArgumentException]::new("Branch name cannot be empty"),
                    "BranchNameEmpty",
                    [System.Management.Automation.ErrorCategory]::InvalidArgument,
                    $null
                )
            )
        }
        
        git town hack "$($branchName)"
        break
    }

    "hack-prototype" {
        $branchName = Read-Host -Prompt "Enter the name of the new branch"

        if ($null -eq $branchName -or [string]::IsNullOrWhiteSpace($branchName)) {
            $PSCmdlet.ThrowTerminatingError(
                [System.Management.Automation.ErrorRecord]::new(
                    [System.ArgumentException]::new("Branch name cannot be empty"),
                    "BranchNameEmpty",
                    [System.Management.Automation.ErrorCategory]::InvalidArgument,
                    $null
                )
            )
        }

        git town hack --prototype "$($branchName)"
        break
    }

    default {
        $PSCmdlet.ThrowTerminatingError(
            [System.Management.Automation.ErrorRecord]::new(
                [System.ArgumentException]::new("Invalid action name: $($ActionName)"),
                "InvalidActionName",
                [System.Management.Automation.ErrorCategory]::InvalidArgument,
                $ActionName
            )
        )
    }
}
