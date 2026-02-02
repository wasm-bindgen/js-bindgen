#!/usr/bin/env -S DUMMY=2>NUL sh
:; # A very quirky solution to make things run cross-platform.

:; # UNIX
:; # Lines starting with `:;` are ignored on Windows but are executed on UNIX.
:; (
:;   cd "$(dirname "$0")/../host/js-bindgen-runner/src/js"
:;   npm install -s --prefer-offline --no-audit --no-fund || exit $?
:;   tsc --build; exit $?
:; ) || exit $?
:; (
:;   cd "$(dirname "$0")/../host"
:;   cargo +stable run -q -p js-bindgen-runner -- "$@"; exit $?
:; ); exit $?

:: Windows
:: Never reached on UNIX because we execute `exit`.
@echo off
pushd "%~dp0..\host\js-bindgen-runner\src\js"
npm install -s --prefer-offline --no-audit --no-fund || exit /b %ERRORLEVEL%
tsc --build || exit /b %ERRORLEVEL%
popd
pushd "%~dp0..\host"
cargo +stable run -q -p js-bindgen-runner -- %*
popd
exit /b %ERRORLEVEL%
