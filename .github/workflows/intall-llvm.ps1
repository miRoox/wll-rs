# based on https://www.powershellgallery.com/packages/AppVeyorBYOC/1.0.170/Content/scripts%5CWindows%5Cinstall_llvm.ps1

$llvmVersion = "10.0.0"
Write-Host "Installing LLVM $llvmVersion ..." -ForegroundColor Cyan
Write-Host "Downloading..."
$exePath = "$env:temp\LLVM-$llvmVersion-win64.exe"
(New-Object Net.WebClient).DownloadFile("https://github.com/llvm/llvm-project/releases/download/llvmorg-$llvmVersion/LLVM-$llvmVersion-win64.exe", $exePath)
Write-Host "Installing..."
cmd /c start /wait $exePath /S

$llvmPath = "$env:ProgramFiles\LLVM\bin"
$env:Path = (($env:Path -split ';') + $llvmPath) -join ';'
[System.Environment]::SetEnvironmentVariable("LIBCLANG_PATH", $llvmPath, [System.EnvironmentVariableTarget]::Machine)

cmd /c clang --version

Write-Host "Installed" -ForegroundColor Green