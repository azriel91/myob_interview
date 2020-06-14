#!/bin/bash
set -euo pipefail

# Profile to publish
profile=release

# Directories
self_dir="$(dirname "$(readlink -f "${BASH_SOURCE}")")"
repository_dir="$(dirname "${self_dir}")"
target_dir="${repository_dir}/target"
target_profile_dir="${target_dir}/${profile}"
target_publish_dir="${target_dir}/publish"

# Application to publish
app_name=pett_server
app_crate_dir="${repository_dir}/${app_name}"
app_publish_artifacts=(
  "${target_profile_dir}/${app_name}"
  "${app_crate_dir}/health.txt"
)
target_publish_app_dir="${target_publish_dir}/${app_name}"

# Ensure the source files exist before transferring
for f in "${app_publish_artifacts[@]}"; do
  if ! test -e "${f}"; then
    echo "ERROR: Publish artifact does not exist: '${f}'"
    exit 1
  fi
done

# Prepare the publish directory
test -d "${target_publish_app_dir}" || mkdir -p "${target_publish_app_dir}"

# To remove extraneous files from the destination directory, we need to use a temporary directory.
#
# * Clean destination directory: <https://stackoverflow.com/a/15383897/1576773>
# * Temporary directory: <https://unix.stackexchange.com/a/84980>
case "${OSTYPE}" in
  linux*          ) cmd_mktmp="mktemp -d";;
  darwin | Darwin ) cmd_mktmp="mktemp -d -t "${target_publish_app_dir}.rsync"";;
  msys            ) echo "Error: Publish app script only usable on *nix systems" 1>&2; exit 1;;
  *               ) echo "Error: Unknown OSTYPE: '${OSTYPE}'" 1>&2; exit 1;;
esac

target_publish_app_temp_dir="$($cmd_mktmp)"

# Deletes the temp directory
function cleanup {
  rm -rf "${target_publish_app_temp_dir}"
}

# Register the cleanup function to be called on the EXIT signal
trap cleanup EXIT

# First rsync from src to dest, as well as hard link the transferred files to a temporary directory.
# Then rsync delete from the temporary directory to the intended dest directory.
rsync -rLE --link-dest="${target_publish_app_dir}" "${app_publish_artifacts[@]}" "${target_publish_app_temp_dir}"
rsync -raLE --delete "${target_publish_app_temp_dir}/" "${target_publish_app_dir}"

exit 0
