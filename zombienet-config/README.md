# Zombienet Configuration

Zombienet aims to be a testing framework for Substrate based blockchains, providing a simple cli tool that allows users to spawn and test **local** ephemeral networks. You can use the [`pop-cli`](https://github.com/r0gue-io/pop-cli?tab=readme-ov-file#install) to run this configuration.

## Running a local test network

Once [`pop-cli`](https://github.com/r0gue-io/pop-cli?tab=readme-ov-file#install) is installed, you can run it as follows:
```sh
pop up parachain -f ./zombienet-config/devnet.toml
```