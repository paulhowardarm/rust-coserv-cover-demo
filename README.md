# rust-coserv-cover-demo
An explorative coding exercise to see how the [coserv-rs](https://github.com/veraison/corim-rs) and [cover](https://github.com/veraison/cover) Rust crates could be used together to verify CCA evidence with endorsements/RVs coming from one or more CoSERV queries, rather than from actual CoRIM resources.

CoSERV and CoRIM have overlapping but distinct data models. The CoMID "triples" are common between the two. While `cover` is specifically designed to use CoRIM as input, this exercise aims to show the feasibility of using CoSERV as the input instead.

## How to Build
This is a Rust project so you need to [install Rust](https://rust-lang.org/tools/install/) first.

Then simply build as follows:

```
cargo build
```

## How to Run
This project is a very minimal piece of playground code and requires no environmental set-up or command-line input. It is entirely self-contained. All of the data objects that it uses are already embedded in the source files. Therefore, the code can be run simply with:

```
cargo run
```

## Okay, So What Did it Do?
At time of writing, it failed with a panic looking something like this:

```
thread 'main' panicked at src/main.rs:354:70:
called `Result::unwrap()` on an `Err` value: Custom("no trust anchor found for EnvironmentMap { class: Some(ClassMap { class_id: Some(Extension(Tag(600, Bytes(Bytes { bytes: [127, 69, 76, 70, 2, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 62, 0, 1, 0, 0, 0, 80, 88, 0, 0, 0, 0, 0, 0] })))), vendor: None, model: None, layer: None, index: None }), instance: Some(Ueid(TaggedUeidType(Accepted(UeidType(Bytes { bytes: [1, 7, 6, 5, 4, 3, 2, 1, 0, 15, 14, 13, 12, 11, 10, 9, 8, 23, 22, 21, 20, 19, 18, 17, 16, 31, 30, 29, 28, 27, 26, 25, 24] }))))), group: None }")
```

Hopefully better times are ahead, but this result is actually not too dreadful. The fact that the code builds and gets as far as this error is still a useful result.

## Okay, So What Did it TRY to Do?
The code attempts to use `cover` to verify an Arm CCA attestation token (in EAT format). The attestation token is embedded as a byte array in the `main.rs` source file. It is identical to the attestation token that would be obtained by a realm running with a pre-silicon Arm CCA platform such as the Fixed Virtual Platform (FVP). Embedding the byte array in the source file means that we don't need to actually spin up an FVP or run a realm in order to get the evidence.

The code also embeds two [CoSERV](https://datatracker.ietf.org/doc/draft-ietf-rats-coserv/01/) data objects (as CBOR byte arrays). These capture the reference values and trust anchors that describe the valid state of an Arm CCA pre-silicon platform. They are identical to the results that would be produced by a live CoSERV-enabled service when queried for CCA trust anchors and reference values for CCA on pre-silicon. Again, the reason they are directly embedded in the code is just to avoid any complex set-up requirements.

The code makes use of the `CorimStore` abstraction/trait published by `cover`. It re-implements the trait in terms of a CoSERV parser rather than a CoRIM parser, but then presents the same fundamental RV/TA relations to the verifier. The main purpose of this code is to demonstrate the feasibility of re-implementing `CorimStore` in this way. It's essentially a throw-away exercise designed to create learning points.

## What Causes the Panic?
Not entirely sure, but it looks like `cover` isn't able to locate a suitable trust anchor for the supplied evidence, and therefore cannot proceed with the evaluation. This may be a relatively trivial issue due to mismatched expectations on CBOR tag numbers - this code is being written at a time where the PSA/CCA CoRIM profile specs have been updated, but not everything has caught up with the change yet. Hopefully it's not a fundamental software-architectural problem, but it needs more investigation.

## Status and Learning Points
- The code doesn't currently produce a complete attestation result due to a `panic` (see above), and it would be nice to have a better understanding of why that happens.
- The code builds successfully, and runs at least as far as populating the store from the CoSERV data. This suggests that it is, overall, feasible to feed `cover` with CoSERV data rather than literal CoRIM, which is the main aim of this demo.
- The code currently only handles _unsigned_ CoSERV - that's just for simplicity, though.
- The code is using a fork/branch of `coserv-rs` from Fujitsu's [MonakaResearch](https://github.com/MonakaResearch/coserv-rs/tree/fjcoserv) repo. This is only because their Rust implementation of the CoSERV data model has not yet been merged to mainline. It's not because any specific changes are required in support of this demo.
- The code is using a fork/branch of `cover` from [paulhowardarm](https://github.com/paulhowardarm/cover/tree/phCoserv). Again, this is not because any specific functional code changes are required. It's just to skirt around an irritating cargo dependency problem with the shared `corim-rs` crate. The aim with this demo is to show that we can, potentially, use `cover` as-is with no changes, just making use of the traits/abstractions that are already there (most notably `CorimStore`).
- It's aesthetically displeasing to be implementing a trait called `CorimStore` when we're not actually storing CoRIMs! Perhaps we just live with that, but it's at least worth thinking about modifying `cover` to split the `CorimStore` trait. The trait has two distinct parts: the CoRIM ingestion part, and the TA/RV delivery part that the `Verifier` consumes. If there was a separate trait for just the delivery part, and it had a more neutral name (without "CoRIM" in the name), it would feel architecturally cleaner.
- There is a fair amount of boilerplate code associated with creating the `RvRelation`, `EvRelation` and `EvsRelation` objects from the CoSERV result data. Much of this boilerplate is extremely similar (almost copy/paste) from the code that does the same job in `cover`, but it's not packaged for re-use. Also, although it's _almost_ copy/paste, it's not quite: CoSERV has quads rather than triples, and the quads provide the authority information for the relations. But there's definitely a bad "code smell" in `store.rs` with the amount of verbosity and near-duplication. Perhaps there's a way to export some helper functions from `cover` to improve the sharing.

## Next Steps
This repo is throwaway - just a playground for experimentation. But some of what is here could possibly be spun out into a library crate.

One thing we could do, of course, is just put this code into `cover` itself, making it natively CoSERV-aware as well as CoRIM-aware. But since `cover` has been built specifically to ingest CoRIM and to exemplify the verification algorithm in the CoRIM IETF Draft, we might prefer to avoid this. Another argument for keeping it separate is that the logic in `store.rs` may also be the beginnings of a cache management layer for CoSERV results, which would be a useful piece of infrastructure to maintain separately from verification.
