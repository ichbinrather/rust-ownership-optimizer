# Run Clippy with the ownership and allocation lints used by the rust-ownership-optimizer skill.
# Usage: run inside a Cargo project:  powershell -ExecutionPolicy Bypass -File path\to\scripts\audit.ps1
$ErrorActionPreference = 'Stop'
$lints = @(
    'clippy::redundant_clone',
    'clippy::implicit_clone',
    'clippy::unnecessary_to_owned',
    'clippy::clone_on_ref_ptr',
    'clippy::needless_pass_by_value',
    'clippy::trivially_copy_pass_by_ref',
    'clippy::large_types_passed_by_value',
    'clippy::box_collection',
    'clippy::boxed_local',
    'clippy::needless_collect',
    'clippy::ptr_arg',
    'clippy::rc_buffer'
)
$cargoArgs = @('clippy', '--all-targets', '--')
foreach ($l in $lints) { $cargoArgs += @('-W', $l) }
& cargo @args
exit $LASTEXITCODE
