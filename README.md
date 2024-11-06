# Distributed Key Generation

:warning: Under Construction
> This crate is actively being developed. Breaking changes will occur until mainnet when we will start [Semantic Versioning](https://semver.org/).

Distributed key-generator of [Radius Block Building Solution]() written in Rust programming language.

Distributed Key Generator is an implementation of aggregated encryption/decryption mechanism which generates both keys at a regular interval. At each interval, an encryption key and its ID pair is generated and made accessible to other entities such as Secure RPC. A decryption key is generated only after a certain amount of time and can be accessed using the corresponding encryption key ID.

## Contributing
We appreciate your contributions to our project. To get involved, refer to the [Contributing guide](https://github.com/radiusxyz/radius-docs-bbs/blob/main/contributing_guide.md).

## Getting Help
Our developers are willing to answer your questions. If you are first and bewildered, refer to the [Getting Help](https://github.com/radiusxyz/radius-docs-bbs/blob/main/getting_help.md) page.
