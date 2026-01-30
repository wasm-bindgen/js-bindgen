#!/usr/bin/env -S arg=2>NUL sh
:; # A very quirky solution to make things run cross-platform.

:; # UNIX
:; # Lines starting with `:;` are ignored on Windows but are executed on UNIX.
:; (
:;   cd "$(dirname "$0")/../host" || exit 1
:;   cargo +stable run -q -p js-bindgen-ld -- "$@"
:; )
:; exit $?

:: Windows
:: Never reached on UNIX because we execute `exit`.
@echo off
pushd "%~dp0..\host" || exit /b 1
cargo +stable run -q -p js-bindgen-ld -- %*
popd
exit /b %ERRORLEVEL%
