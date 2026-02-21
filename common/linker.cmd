#!/usr/bin/env -S arg=2>NUL sh
:; # A very quirky solution to make things run cross-platform.

:; # We explicitely specify `+stable` because even if we run client packages with Nightly,
:; # we never want to run the linker on Nightly. Unless we are testing the linker,
:; # in which case we don't go through the shim.

:; # UNIX
:; # Lines starting with `:;` are ignored on Windows but are executed on UNIX.
:; (
:;   cd "$(dirname "$0")/../host/ld/src/js"
:;   tsc --build; exit $?
:; ) || exit $?
:; (
:;   cd "$(dirname "$0")/../host"
:;   cargo +stable run -q -p js-bindgen-ld -- "$@"; exit $?
:; ); exit $?

:: Windows
:: Never reached on UNIX because we execute `exit`.
@echo off
pushd "%~dp0..\host\ld\src\js"
tsc --build || exit /b %ERRORLEVEL%
popd
pushd "%~dp0..\host"
cargo +stable run -q -p js-bindgen-ld -- %*
popd
exit /b %ERRORLEVEL%
