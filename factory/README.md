# Software Factory implementation

`factory/` is the executable Rust implementation root for the Software Factory product in this repository.

The product semantics remain governed by the canonical corpus under `docs/canon/` and by the ordered programme/ticket acceptance criteria. Language-neutral interoperability and conformance material belongs under `contracts/factory/`; Rust types implement those semantics but do not redefine their wire meaning by convenience.

The implementation boundary is intentionally narrow:

- AIKit remains a separate control-plane product. Factory may consume its public seams; this crate does not absorb AIKit internals.
- Workcell remains a separate product at `EpiLogos/Workcell`. Factory owns shared semantic demands/bindings, not Workcell provider or materialisation internals.
- QL/MEF remains an optional formalism/provider. Factory does not contain a standalone QL kernel.
- `ql-agent-experiments/` is the separate QL Loop Runtime programme and is not the Factory implementation root.
- historical Factory source islands remain prior art unless current canon and tickets explicitly recover a behavior.

## Quality baseline

The Factory workflow runs the following checks against this crate:

```text
cargo fmt --manifest-path factory/Cargo.toml -- --check
cargo clippy --manifest-path factory/Cargo.toml --all-targets -- -D warnings
cargo test --manifest-path factory/Cargo.toml --all-targets
```

Draft PR #131 is the active integration surface for the Rust implementation sequence. Draft PR #125 is retained only as prior executable contract/semantic exploration and is not the product implementation baseline.
