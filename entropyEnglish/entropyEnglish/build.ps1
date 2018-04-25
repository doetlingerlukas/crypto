$v14 = 'C:\Program Files (x86)\MSBuild\14.0\Bin\Msbuild.exe'
$v15 = 'C:\Program Files (x86)\MSBuild\15.0\Bin\Msbuild.exe'

if(Test-Path -Path $v14) {
  & $v14 entropyEnglish.csproj /t:Build /p:Configuration=Release
} elseif (Test-Path -Path $v15) {
  & $v15 entropyEnglish.csproj /t:Build /p:Configuration=Release
} else {
  Write-Host 'MSBuild not installed.'
}

$binary = '.\bin\Release\entropyEnglish.exe'

if (Test-Path -Path $binary) {
  Invoke-Expression -Command $binary
}
