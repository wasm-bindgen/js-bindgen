@echo off
pushd "%~dp0..\host" || exit /b 1
cargo run -p js-bindgen-linker -- %*
popd
exit /b %ERRORLEVEL%
