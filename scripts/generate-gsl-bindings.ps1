$ErrorActionPreference = 'Stop'

# Always run from repository root.
$repoRoot = Split-Path -Parent $PSScriptRoot
Set-Location $repoRoot

if (-not $env:LIBCLANG_PATH) {
    # Fallback path for local development environment.
    $defaultLibclangPath = 'C:\Program Files\ANSYS Inc\v222\optiSLang\scripting\algorithms\stochos_env\Lib\site-packages\clang\native'
    if (Test-Path (Join-Path $defaultLibclangPath 'libclang.dll')) {
        $env:LIBCLANG_PATH = $defaultLibclangPath
    }
}

if (-not $env:LIBCLANG_PATH) {
    throw 'LIBCLANG_PATH is not set and no default libclang.dll was found.'
}

$wrapper = 'include/wrapper.h'
$output = 'src/gsl_bindings.rs'

# Build a single umbrella header including all local GSL headers.
Get-ChildItem -Path 'include/gsl' -Filter 'gsl_*.h' |
    Sort-Object Name |
    ForEach-Object { "#include <gsl/$($_.Name)>" } |
    Out-File -Encoding ascii $wrapper

# Generate raw bindings.
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
