# Changelog

## 1.0.0 (2024-07-26)


### Features

* improve prompt to better identify invoices ([7ddf0c4](https://github.com/lippoliv/paperlessngx-ollama/commit/7ddf0c4fcd5b091a8c3c4ac3dffdac62e4af24d5))
* load documents to process by paperless ngx url ([8d12c36](https://github.com/lippoliv/paperlessngx-ollama/commit/8d12c3665a8f887d573844ef3bdca0a8d2555db6))
* run in a loop as long as documents are processed ([6c653c0](https://github.com/lippoliv/paperlessngx-ollama/commit/6c653c096eb64521c5ce03f1de839c4e2be2762e))
* send documents to ollama to generate a summary ([03d7b00](https://github.com/lippoliv/paperlessngx-ollama/commit/03d7b00c952ea3989b0973614b7131a7bea089e8))
* update document via paperless api ([1444619](https://github.com/lippoliv/paperlessngx-ollama/commit/1444619c3b32d3d13214f2eb2e74405a1a1251d4))
* users do define model to use via ENV OLLAMA_MODEL ([d12922a](https://github.com/lippoliv/paperlessngx-ollama/commit/d12922abdc07fafb7b0a61473915fa4d162ec160))


### Bug Fixes

* crash if string limits at special character (e.g. €) ([c011267](https://github.com/lippoliv/paperlessngx-ollama/commit/c011267ed8c7d28ef4d043bfb0601ba4ab7a78bb))
* ensure not to send titles longer than 125 chars ([08224ee](https://github.com/lippoliv/paperlessngx-ollama/commit/08224eec5a12f163709eb7a2b205617580f31a8a))
* load environment variables at runtime ([e2e40e5](https://github.com/lippoliv/paperlessngx-ollama/commit/e2e40e575182f5eccd825cffc28156d58de8df48))

## Changelog
