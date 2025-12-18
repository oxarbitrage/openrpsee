# Openrpsee

**Openrpsee** is a helper library for building **OpenRPC documents** and implementing the `rpc.discover` endpoint.

Its main goal is to centralize common functionality used across the Zcash **Z3 stack** (**Zebra**, **Zallet**, and **Zaino**) while remaining generally useful for any project that builds JSON-RPC servers using the `jsonrpsee` ecosystem.

## Overview

Openrpsee provides utilities to:

- Extract RPC method definitions from Rust traits
- Generate a structured representation of those methods at build time
- Produce standards-compliant OpenRPC documents at runtime
- Expose those documents through a `rpc.discover` RPC endpoint

## Generating OpenRPC Metadata

The `generate_openrpc` function parses the given RPC traits, extracts all method definitions, and builds a map containing the metadata required to generate an OpenRPC document.
This map is then written as a Rust source file into the specified output directory.

### Examples

- Zallet:
- Zebra:

## Including the Generated File

The generated Rust file is intended to be included from your project’s `methods.rs` (or equivalent) module.

### Examples:

- Zallet:
- Zebra:

## Generating the OpenRPC Document

The `openrpc` module provided by this crate consumes the generated method map and produces an OpenRPC document.
This document is typically returned by an implementation of the `rpc.discover` RPC method.

### Examples:

- Zallet:
- Zebra:

## Argument Documentation

All RPC arguments are expected to be described using constants. These constants are used when generating the OpenRPC schema.

Examples:

- Zallet:
- Zebra:

## Produced OpenRPC Documents

Live OpenRPC documents generated using this library can be found at:

- Zallet:
- Zebra:
- Z3:

## Playground

Interactive playgrounds exposing the generated OpenRPC schemas:

- Zallet:
- Zebra:
- Z3:
