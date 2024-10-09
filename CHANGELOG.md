# Changelog

All notable changes to this project will be documented in this file.

## [unreleased]

### 🚜 Refactor

- Simplify Adaptor instantiation - ([b5e241e](https://github.com/vainjoker/realme/commit/b5e241e0cc1bd5f967f43393308d1da96f2a72a7))

### 📚 Documentation

- Update for badges - ([8f9d86e](https://github.com/vainjoker/realme/commit/8f9d86e03475e03b4280771d84fa39496e1577c3))

### ⚙️ Miscellaneous Tasks

- Remove Travis CI configuration - ([e04ba5d](https://github.com/vainjoker/realme/commit/e04ba5dcbc79b2617e00e69f28ea5c52269fdeec))
- Update codecov-action to v4 - ([ed6f4f2](https://github.com/vainjoker/realme/commit/ed6f4f20a92994c148cb0fde773307da0aac9219))
- Integrate coverage reporting - ([eae11f3](https://github.com/vainjoker/realme/commit/eae11f3b7aec034aec9327f7961336614be3c803))

## [0.1.3](https://github.com/vainjoker/realme/compare/v0.1.2..v0.1.3) - 2024-09-28

### ✨  Features

- Bump version to 0.1.3    - ([d7f5ef9](https://github.com/vainjoker/realme/commit/d7f5ef9bea5ededca4573e4da516e05c43ee329d))
- Refactor: Improve API ergonomics with key traits    - ([152be6b](https://github.com/vainjoker/realme/commit/152be6b7b3889210ec9ddd116ac4eb50b9c0a6e4))
- Update: Migrate to new API for accessing values - ([5646a54](https://github.com/vainjoker/realme/commit/5646a54e7be67714042acfc0656a31deac4a6d14))

### 🐛 Bug Fixes

- Fix: Address potential panic in env var access - ([f70cb17](https://github.com/vainjoker/realme/commit/f70cb171d3c5287a74bf4543f82734ca8eb41cc0))

### ⚙️ Miscellaneous Tasks

- Refactor dependency management and code structure    - ([de9b098](https://github.com/vainjoker/realme/commit/de9b0983eba030aa6231995b29e4564a422efe61))

### Build

- Delete release action - ([8923fe1](https://github.com/vainjoker/realme/commit/8923fe1f793613d766c0b91764cc1f13584b55c9))

## [0.1.2](https://github.com/vainjoker/realme/compare/v0.1.1..v0.1.2) - 2024-09-18

### ✨  Features

- Add example for global config setting - ([28f66f3](https://github.com/vainjoker/realme/commit/28f66f3e1bdc084d72ab2b04443b5fdb377e57c2))

### 🐛 Bug Fixes

- Fix env variable replacement in nested structures - ([0a8a4e1](https://github.com/vainjoker/realme/commit/0a8a4e1275ee177790ffd07d4fe944b5f02ededc))

### Build

- Update for v0.1.2 - ([a3cf6af](https://github.com/vainjoker/realme/commit/a3cf6af2d82a224ed351047cd8f677a17001d00f))

## [0.1.1] - 2024-09-16

### ✨  Features

- Add expration for set and get value - ([98cc46c](https://github.com/vainjoker/realme/commit/98cc46c8cdbebbcc5e183a0ae2a73fa8176a4882))
- Add support for more parser - ([7bedf7a](https://github.com/vainjoker/realme/commit/7bedf7a1d331df0e3adb5e2079e2b5d09eb97bf6))
- Add tracing support - ([24d1af1](https://github.com/vainjoker/realme/commit/24d1af191bb94bbc5d39cd1b2321fbc60c6b8378))
- Support load adaptor from cmd input - ([2c403d3](https://github.com/vainjoker/realme/commit/2c403d33aac99ace16bacbf6a8fd7d484371de9f))
- Implement parser and cast error - ([89f2685](https://github.com/vainjoker/realme/commit/89f26853af813aa824e21a8163fd276a9319689b))
- Implement better error handling - ([1e1490d](https://github.com/vainjoker/realme/commit/1e1490ddf2851bfa9f317345961bf1ad5fc8a2e1))
- Add support for env and file adaptor - ([78e57dd](https://github.com/vainjoker/realme/commit/78e57dd7270c7531d34eba7fba6caf22511a6694))
- Make Parser into features - ([79dc063](https://github.com/vainjoker/realme/commit/79dc0634f8e95813529882c2e8727368b767b81d))
- Finish Serialize&Deserialize for Value - ([c41e6c7](https://github.com/vainjoker/realme/commit/c41e6c7f0c5ca528f544876747dc0a557a23328d))
- Use PhantomData wrap Parser - ([98ea304](https://github.com/vainjoker/realme/commit/98ea304c1f432e8c551611417604dddc0ac16934))
- Support custom parser - ([35fb75d](https://github.com/vainjoker/realme/commit/35fb75dabc46ac9ec0108aa3cca81567f52e87c8))
- Serialize Realm works - ([5cf578e](https://github.com/vainjoker/realme/commit/5cf578e7e684d2796a74dfc79a0ed9e01e66918b))
- Finish basic deserialize for Realm - ([a6f9dc4](https://github.com/vainjoker/realme/commit/a6f9dc416a74933b2a507d4ed568178df1234c92))
- Complete basic TOML parsing functionality - ([875275d](https://github.com/vainjoker/realme/commit/875275d9341c98118a92dc24aec228ec9ea71bae))
- Implement the basic structure - ([6c6e3aa](https://github.com/vainjoker/realme/commit/6c6e3aadbd250b41f8096ae7cae91dd742f89d4a))
- Initial commit - ([95f05db](https://github.com/vainjoker/realme/commit/95f05dbfafc61fabece1ed0b86f16c2f83fc0d67))

### 🐛 Bug Fixes

- Try serialize toml datetime - ([de6668c](https://github.com/vainjoker/realme/commit/de6668c309af9a796aed3800b61f3c24c9117c86))

### 🚜 Refactor

- Refactor code structure - ([e446217](https://github.com/vainjoker/realme/commit/e446217f570ad1ef2bc79f831d7c529665f7531a))
- Source and Parser now use generics parameter - ([29f6333](https://github.com/vainjoker/realme/commit/29f6333b5330e4c1a5ff49e21aa87ce4c84d7cbd))
- Repub all your needs - ([f2daff4](https://github.com/vainjoker/realme/commit/f2daff44ab618b41ad42081fc14c651f75ff9c7b))

### 📚 Documentation

- Add rust docs - ([ef9aeba](https://github.com/vainjoker/realme/commit/ef9aebadf245a35fe4088a14ee579d8ef25f8565))
- Add TODO.md - ([cc93326](https://github.com/vainjoker/realme/commit/cc93326bfb462b71c2dd34551d61b213fd29f5b0))
- Add readme file for the project - ([cf57b8b](https://github.com/vainjoker/realme/commit/cf57b8b1a3aa2eb4f491d2bc97dccb19fa887a3d))

### 🎨 Styling

- Format the code - ([977d90b](https://github.com/vainjoker/realme/commit/977d90b5d073e5613f5d68faec1788223b87a98f))

### ✅ Testing

- Add value module tests - ([97e28c3](https://github.com/vainjoker/realme/commit/97e28c307a440d6ee55c28c02841328e2d6f16d0))

### ⚙️ Miscellaneous Tasks

- Clean commit before merge - ([3d8a9c6](https://github.com/vainjoker/realme/commit/3d8a9c6ac5e13831229534ca740ba8dc325a49f5))

### Build

- Update crate name for publish - ([f19c58a](https://github.com/vainjoker/realme/commit/f19c58a824a204aa300399187d69eae6107c3cdd))

<!-- generated by git-cliff -->
