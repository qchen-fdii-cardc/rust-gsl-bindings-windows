$ErrorActionPreference = 'Stop'

# Always run from repository root.
$repoRoot = Split-Path -Parent $PSScriptRoot
Set-Location $repoRoot

# Set this if bindgen can't find libclang on your system. 
# On Windows, the LLVM installer includes a copy of libclang.dll that works with bindgen.
# $env:LIBCLANG_PATH = "C:\Program Files\LLVM\bin" # change this if your LLVM is installed somewhere else
# Set it to the directory of LLVM
#  winget install --id LLVM.LLVM --source winget will install LLVM

$wrapper = 'include/wrapper.h'
$output = 'src/gsl_bindings.rs'
 
# Step 1: Build umbrella wrapper.h from all local GSL headers.
Write-Host 'Step 1/2: Building include/wrapper.h from include/gsl/*.h...'
Get-ChildItem -Path 'include/gsl' -Filter '*.h' |
    Sort-Object Name |
    ForEach-Object { "#include <gsl/$($_.Name)>" } |
    Out-File -Encoding ascii $wrapper
 
# Step 2: Run bindgen-cli from wrapper.h to produce raw bindings.
Write-Host 'Step 2/2: Running bindgen-cli...'
bindgen $wrapper --no-layout-tests --allowlist-function "gsl_.*" --allowlist-type "gsl_.*" --allowlist-var "GSL_.*|gsl_.*" -- -Iinclude | Out-File -Encoding ascii $output

# On Windows MSVC, attach raw-dylib link attributes to every extern block so
# direct unsafe calls from bindgen symbols can resolve without per-function wrappers.
$raw = Get-Content -Raw -Path $output
$inject = @'
#[cfg_attr(all(target_os = "windows", target_env = "msvc"), link(name = "gsl", kind = "raw-dylib"))]
#[cfg_attr(all(target_os = "windows", target_env = "msvc"), link(name = "gslcblas", kind = "raw-dylib"))]
unsafe extern "C" {
'@
$linked = $raw -replace 'unsafe extern "C" \{', $inject
Set-Content -Path $output -Value $linked -Encoding ascii

Write-Host "Generated $output from $wrapper"
