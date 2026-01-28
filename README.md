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

There is one environmental dependency on a Veraison test service instance hosted by [Linaro](https://www.linaro.org) at `https://veraison.test.linaro.org:11443`. This instance provides the CoSERV API endpoint, along with the endorsements and reference values required for pre-silicon Arm CCA platforms.

## Okay, So What Did it Do?
The source code directly embeds an example of a CCA attestation token.
The application will cryptographically verify and appraise this attestation token, using trust anchors and reference values obtained from a CoSERV distribution endpoint.
It will then display the result. The output will look something like the following:-

```
Using CoSERV service at https://veraison.test.linaro.org:11443
The reference value query is: ogB4I3RhZzphcm0uY29tLDIwMjM6Y2NhX3BsYXRmb3JtIzEuMC4wAaQAAgGhAIGBoQDZAlhYIH9FTEYCAQEAAAAAAAAAAAADAD4AAQAAAFBYAAAAAAAAAsB0MjAyNi0wMS0yOFQxMzoxMToyMFoDAA
The trust anchor query is: ogB4I3RhZzphcm0uY29tLDIwMjM6Y2NhX3BsYXRmb3JtIzEuMC4wAaQAAQGhAYGB2QImWCEBBwYFBAMCAQAPDg0MCwoJCBcWFRQTEhEQHx4dHBsaGRgCwHQyMDI2LTAxLTI4VDEzOjExOjIwWgMA
EAR: {"eat_profile":"arm-cca","iat":1769605880,"ear.verifier-id":{"developer":"https://veraison-project.org","build":"cover 0.0.1"},"submods":{"platform":{"ear.status":"affirming","ear.trustworthiness-vector":{"instance-identity":2,"configuration":2,"executables":3,"file-system":0,"hardware":2,"runtime-opaque":2,"storage-opaque":2,"sourced-data":0}},"realm":{"ear.status":"warning","ear.trustworthiness-vector":{"instance-identity":2,"configuration":0,"executables":33,"file-system":0,"hardware":0,"runtime-opaque":2,"storage-opaque":0,"sourced-data":0}}},"ear.raw-evidence":"2QGPohmsylkF7tKERKEBOCKgWQWBqRkBCXgjdGFnOmFybS5jb20sMjAyMzpjY2FfcGxhdGZvcm0jMS4wLjAKWCANIuCKmEaQWEhjGCg0ib2zbwnb7-sYZN9DP6blTqLXERkJXFggf0VMRgIBAQAAAAAAAAAAAAMAPgABAAAAUFgAAAAAAAAZAQBYIQEHBgUEAwIBAA8ODQwLCgkIFxYVFBMSERAfHh0cGxoZGBkJYUTPz8_PGQlbGTADGQliZ3NoYS0yNTYZCWB4Omh0dHBzOi8vdmVyYWlzb24uZXhhbXBsZS8ud2VsbC1rbm93bi92ZXJhaXNvbi92ZXJpZmljYXRpb24ZCV-NpAFpUlNFX0JMMV8yBVggU3h5YwdTXfPsjYsVouLcVkFBnD0wYM_jIjjA-pc_eqMCWCCaJx8qkWsLbubOyyQm8LMgbvB0V4vlXZvJT28_46uGqgZnc2hhLTI1NqQBZ1JTRV9CTDIFWCBTeHljB1Nd8-yNixWi4txWQUGcPTBgz-MiOMD6lz96owJYIFPCNOXoRytqxRwa4cqz_gb60FO-uOv9iXewEGVb_dPDBmdzaGEtMjU2pAFlUlNFX1MFWCBTeHljB1Nd8-yNixWi4txWQUGcPTBgz-MiOMD6lz96owJYIBEhz8zVkT8KY_7ECm_9ROpk-dwTXGZjS6AB0QvPQwKiBmdzaGEtMjU2pAFmQVBfQkwxBVggU3h5YwdTXfPsjYsVouLcVkFBnD0wYM_jIjjA-pc_eqMCWCAVcbXseL1oUSv3gwu2oqRLIEfH31e85564ocDlvqClAQZnc2hhLTI1NqQBZkFQX0JMMgVYIFN4eWMHU13z7I2LFaLi3FZBQZw9MGDP4yI4wPqXP3qjAlggEBWbryYrQ6ktldtZ2uH3LGRRJzAWYeCjzk44spWpfFgGZ3NoYS0yNTakAWdTQ1BfQkwxBVggU3h5YwdTXfPsjYsVouLcVkFBnD0wYM_jIjjA-pc_eqMCWCAQEi6Faz_NSfBjY2MXR2FJy3MKGqHPqtgYVSty9W1vaAZnc2hhLTI1NqQBZ1NDUF9CTDIFWCDxS0mHkEvLWBTkRZoFftTSD1imMxUiiKdhIU3NKHgLVgJYIKpnoWmwu6IXqgqoimU0aSDITEJEfDa6X36mX0IsH-XYBmdzaGEtMjU2pAFnQVBfQkwzMQVYIFN4eWMHU13z7I2LFaLi3FZBQZw9MGDP4yI4wPqXP3qjAlggLm0xpZg6kSUb-uWu-hwKGdi6PPYB0OinBrTPqWYaa4oGZ3NoYS0yNTakAWNSTU0FWCBTeHljB1Nd8-yNixWi4txWQUGcPTBgz-MiOMD6lz96owJYIKH7UObIb64Wee8zUSlv1nE0EaCM-N0XkKT9BfroaIFkBmdzaGEtMjU2pAFpSFdfQ09ORklHBVggU3h5YwdTXfPsjYsVouLcVkFBnD0wYM_jIjjA-pc_eqMCWCAaJSQCly9gV_pTzBcrUrn_ymmOGDEfrNDzsG7KrveeFwZnc2hhLTI1NqQBaUZXX0NPTkZJRwVYIFN4eWMHU13z7I2LFaLi3FZBQZw9MGDP4yI4wPqXP3qjAlggmpKtvAzuOO9ljHHOGxv4xlZo8Wa_shNkTIlcyxrQeiUGZ3NoYS0yNTakAWxUQl9GV19DT05GSUcFWCBTeHljB1Nd8-yNixWi4txWQUGcPTBgz-MiOMD6lz96owJYICOJAxgMwQTsLF2LPyDFvGGziewKln34zCCM3HzUVBdPBmdzaGEtMjU2pAFtU09DX0ZXX0NPTkZJRwVYIFN4eWMHU13z7I2LFaLi3FZBQZw9MGDP4yI4wPqXP3qjAlgg5sIejSYP5xiC3r2zOdJAKiynZIUpvCMD9IZJvOA4ABcGZ3NoYS0yNTZYYDHQTVLM3pUsHjLLoYGIWkC4zDjgUoweiViYB2QqpePyvDf5U3RQa_9NLkvnBjxNckGScMci6NTZPui2yfrOO0PJdhpJlBq284_9_0lq1GO0y_oR2D4j4x9_YjKd4wwcyBms0VkCTtKERKEBOCKgWQHhqBkBCXgcdGFnOmFybS5jb20sMjAyMzpyZWFsbSMxLjAuMApYQG6G1tl8xxO8bdQ9vOSRprQDEcAnqL-Fo52mPpzkTBMqihGdKW-uammZ6b8-RHGwzgEkXYiUJMMeiXk7Ox1rFQQZrMxnc2hhLTI1Nhms0GdzaGEtMjU2GazLWEBUaGUgcXVpY2sgYnJvd24gZm94IGp1bXBzIG92ZXIgMTMgbGF6eSBkb2dzLlRoZSBxdWljayBicm93biBmb3ggGazNWGukAQIgAiFYMHb5iAkb5YXtQYAa7Pq4WFSMYwV-FrDmdhILvQ0vnCngVsXUGgEw65whUXiZ3CMUayJYMCjhsGK9PqSzFf0hnxy7Uoy250ykm-Fnc3NPYaHKYQMbK789kY8vlP_EIo5QkZVErhmszlggMRMUq3NiA1DPdYg0rlxl2ejC3H_r5ufZZUu-hk4wDUkZrM-EWCAk1bCilswFy9gGjFBnxb1HO3cN2mrggv47owq-P5pqsVggeI_AkL_GuO2QMVK6hBTnPa9bjHux55rVAqsGmbZZ7RZYINrEalhBXcOgDXp0GFIAjpyuZPUtA7n3bXb0s2RP78QWWCAyxq_GJ-VVhcAxVTWfMxoOIl9oQNuUfdlu-rgb4mcZOVhgWAsd6jLTCsaITIaznL4PywO9AN9RA_m6sBOGpGo7qBQ-J-1tTrDQonJKvflkDAlGL6zm3xhpCd-m6xMeOnkYJ2B3rNq4qL3sprDqr6tm4UOcE3H0-x1qrAR0gbXcdd1G"}
```

## Status and Learning Points
- The code currently only handles _unsigned_ CoSERV - that's just for simplicity, though.
- It's aesthetically displeasing to be implementing a trait called `CorimStore` when we're not actually storing CoRIMs! Perhaps we just live with that, but it's at least worth thinking about modifying `cover` to split the `CorimStore` trait. The trait has two distinct parts: the CoRIM ingestion part, and the TA/RV delivery part that the `Verifier` consumes. If there was a separate trait for just the delivery part, and it had a more neutral name (without "CoRIM" in the name), it would feel architecturally cleaner.

## Next Steps
This repo is throwaway - just a playground for experimentation. But some of what is here could possibly be spun out into a library crate.

One thing we could do, of course, is just put this code into `cover` itself, making it natively CoSERV-aware as well as CoRIM-aware. But since `cover` has been built specifically to ingest CoRIM and to exemplify the verification algorithm in the CoRIM IETF Draft, we might prefer to avoid this. Another argument for keeping it separate is that the logic in `store.rs` may also be the beginnings of a cache management layer for CoSERV results, which would be a useful piece of infrastructure to maintain separately from verification.
