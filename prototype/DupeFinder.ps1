
param (
    [string][Parameter(Mandatory=$true)]$Path,
    [string]$NamePattern = "*"
)


function PrintFileGroup {
    param (
        $gr
    )
    Write-Output " $($gr.FileName) $($gr.Files.Count) "
    $gr.Files `
        | ForEach-Object { Write-Output "   $($_.FullName) $($_.Length)" } 
}

function CreateFileNameGroupWithUniqFileLengths {
    param (
        [Microsoft.PowerShell.Commands.GroupInfo]$gr
    )

    $fileList =  @()

    $gr.Group | Group-Object -Property Length `
              | ForEach-Object { 
                  $_.Group | Select-Object -First 1 | ForEach-Object { $fileList += $_ } 
              }

    [PSCustomObject]@{
        FileName = $gr.Name
        Count = $fileList.Count
        Files = $fileList
    }
}

Get-ChildItem $Path -Recurse `
    | Where-Object { $_.Name -like $NamePattern } `
    | Group-Object -Property Name `
    | Where-Object { $_.Count -gt 1 } `
    | ForEach-Object { CreateFileNameGroupWithUniqFileLengths $_ } `
    | Where-Object { $_.Count -gt 1 } `
    | Sort-Object -Property Count -Descending `
    | ForEach-Object { PrintFileGroup $_ }

Write-Output ""