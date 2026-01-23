@echo off
pushd "%~dp0..\host" || exit /b 1
cargo run -q -p js-bindgen-test-runner -- %*
popd
exit /b %ERRORLEVEL%
