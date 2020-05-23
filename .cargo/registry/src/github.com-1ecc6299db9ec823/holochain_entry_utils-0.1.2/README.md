# holochain_entry_utils

Holochain utilities crate around entries, implementing common behaviour and functions.

## Usage

Add this dependency to your `Cargo.toml`:

```toml
[dependencies]
holochain_entry_utils = "0.1.1"
```

Implement the `HolochainEntry` trait:

```rust
    use holochain_entry_utils::HolochainEntry;

    #[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
    pub struct TestEntry {
        contents: String,
    }

    impl HolochainEntry for TestEntry {
        fn entry_type() -> String {
            String::from("test_type")
        }
    }

    pub fn entry_def() -> ValidatingEntryType {
        entry!(
            name: TestEntry::entry_type(),
        ...
    }
```

And now the functions defined in the Trait (see [documentation](https://docs.rs/holochain_entry_utils)) are available for you to use:

```rust
pub fn test_function(test_entry: TestEntry) -> ZomeApiResult<Address> {
    hdk::commit_entry(&test_entry.entry())
} 
```
