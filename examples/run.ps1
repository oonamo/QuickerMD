param(
    [Parameter(Mandatory)]
    [string]$path
)

$extension = [System.IO.Path]::GetExtension($path).SubString(1)

if ($extension -like "py") {
    $extension = "python"
}

cat $path | quicker_md run $extension --show-input
