# 3.0.0 (2025-03-22)


### Bug Fixes

* Fix partial move in error handling ([f32294d](https://github.com/lzlrd/wsl-hello-sudo/commit/f32294d42125c5c915e5d6e9beb49d56c84b7fa6))
* Fix warning that pointer doesn't outlive temporary CString ([#23](https://github.com/lzlrd/wsl-hello-sudo/issues/23)) ([97289e5](https://github.com/lzlrd/wsl-hello-sudo/commit/97289e55b2e85e4e5994f0e8b9617900e3ae8079))
* **install:** Fix installation failure that occured because .exe files did not have the execution permissions. ([5788c93](https://github.com/lzlrd/wsl-hello-sudo/commit/5788c93f265201134d5afe719b607a715a12e07d))
* quote hello_winpath in uninstall.sh ([#28](https://github.com/lzlrd/wsl-hello-sudo/issues/28)) ([bd7e009](https://github.com/lzlrd/wsl-hello-sudo/commit/bd7e009beb15d7248c590e521eed3ecbaec16515))
* quote PAM_WSL_HELLO_WINPATH to allow spaces ([abad2f8](https://github.com/lzlrd/wsl-hello-sudo/commit/abad2f8c79e02d1436899a1a89af67b2efd7b08d))


### Features

* add pam-config for automatic configuration Install automatic PAM config ([1380e27](https://github.com/lzlrd/wsl-hello-sudo/commit/1380e2749f79bf0dd0255ec7a7ebc360b8b71290)), closes [#14](https://github.com/lzlrd/wsl-hello-sudo/issues/14)
* move hello to foreground once window exists ([#27](https://github.com/lzlrd/wsl-hello-sudo/issues/27)) ([b0b9f54](https://github.com/lzlrd/wsl-hello-sudo/commit/b0b9f543fdef61dc3cb77bb0fea523afcb62fa65))
* Rewrite Windows components in Rust ([#26](https://github.com/lzlrd/wsl-hello-sudo/issues/26)) ([d87ad12](https://github.com/lzlrd/wsl-hello-sudo/commit/d87ad1238e0f64e907f929b84f2bfcaec607e436))
* support custom /c mountpoints ([#13](https://github.com/lzlrd/wsl-hello-sudo/issues/13)) ([5fcc2a1](https://github.com/lzlrd/wsl-hello-sudo/commit/5fcc2a183956f5db27359b3a798d3b77a8e14e8b))


### Performance Improvements

* decrease pam module binary size ([c79a184](https://github.com/lzlrd/wsl-hello-sudo/commit/c79a18458ac71d11d3e99af5eecdb4adb17d34f0))


### BREAKING CHANGES

* The file location and interface of the Windows component are updated, though it was not intended to be used by end users.

Co-authored-by: Takaya Saeki <abc.tkys+pub@gmail.com>



