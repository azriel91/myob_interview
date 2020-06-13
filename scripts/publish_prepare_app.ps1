# Stop on error: <https://stackoverflow.com/a/44810914/1576773>
Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"
$PSDefaultParameterValues["*:ErrorAction"]="Stop"

# Profile to publish
$profile = "release"

# Directories
$script_dir = $PSScriptRoot
$repository_dir = Split-Path -Path "$script_dir"
$target_dir = "${repository_dir}\target"
$target_profile_dir = "${target_dir}\${profile}"
$target_publish_dir = "${target_dir}\publish"

# Application to publish
$app_name = "pett_server"
$app_crate_dir = "${repository_dir}\${app_name}"
$app_publish_artifacts = @(
  "${target_profile_dir}\${app_name}.exe",
  "${app_crate_dir}\health.txt"
)
$target_publish_app_dir = "${target_publish_dir}\${app_name}"

# Ensure the source files exist before transferring
$app_publish_artifacts | ForEach-Object {
  if (!(Test-Path "${PSItem}")) {
    Write-Host "ERROR: Publish artifact does not exist: '${PSItem}'"
    Exit 1
  }
}

# Prepare the publish directory
if (Test-Path "${target_publish_app_dir}") {
  Remove-Item -Recurse -Force "${target_publish_app_dir}"
}
New-Item -ItemType Directory -Force -Path "${target_publish_app_dir}" | Out-Null

$app_publish_artifacts | ForEach-Object {
  Copy-Item -Path "${PSItem}" -Destination "${target_publish_app_dir}" -Recurse
}
