#!/usr/bin/env bash
set -euo pipefail

base_sha="${1:-}"
head_sha="${2:-HEAD}"

full_workspace() {
  printf 'test_scope=workspace\n'
  printf 'test_packages=\n'
}

if [[ -z "${base_sha}" ]] \
  || ! git cat-file -e "${base_sha}^{commit}" 2>/dev/null \
  || ! git cat-file -e "${head_sha}^{commit}" 2>/dev/null; then
  full_workspace
  exit 0
fi

declare -A packages=()
full_workspace_required=false

add_packages() {
  local package
  for package in "$@"; do
    packages["${package}"]=1
  done
}

while IFS= read -r path; do
  case "${path}" in
    Cargo.toml|Cargo.lock|*/Cargo.toml|config/*|.cargo/*|rust-toolchain|rust-toolchain.toml|rustfmt.toml|clippy.toml|Makefile|.github/workflows/*|ci/*)
      full_workspace_required=true
      break
      ;;
    crates/config/*)
      full_workspace_required=true
      break
      ;;
    crates/lb/*)
      add_packages impulse-lb impulse-errors impulse-edge impulse
      ;;
    crates/errors/*)
      add_packages impulse-errors impulse-bridge impulse-transport impulse-edge impulse
      ;;
    crates/bridge/*)
      add_packages impulse-bridge impulse-edge impulse
      ;;
    crates/transport/*)
      add_packages impulse-transport impulse-edge impulse
      ;;
    crates/utils/*)
      add_packages impulse-utils impulse-edge impulse
      ;;
    crates/edge/*)
      add_packages impulse-edge impulse
      ;;
    impulse/*)
      add_packages impulse
      ;;
    *)
      full_workspace_required=true
      break
      ;;
  esac
done < <(git diff --name-only "${base_sha}" "${head_sha}")

if [[ "${full_workspace_required}" == true || ${#packages[@]} -eq 0 ]]; then
  full_workspace
  exit 0
fi

printf 'test_scope=packages\n'
printf 'test_packages='
printf '%s\n' "${!packages[@]}" | sort | paste -sd ' ' -
