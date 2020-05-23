extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_json_derive;

use hdk::prelude::*;
use std::convert::TryFrom;

/**
 * Common utility crate that implements default behaviours to make it easy to deal with structs 
 * that are stored in holochain entries
 */
pub trait HolochainEntry: TryFrom<JsonString> + Into<JsonString> + Clone {
    /**
     * Returns the entry_type for this Entry
     * This entry_type is what is used to identify the Entry in the entry definition
     */
    fn entry_type() -> String;

    /**
     * Wraps the struct into the holochain Entry
     */
    fn entry(self) -> Entry {
        Entry::App(Self::entry_type().into(), self.into())
    }

    /**
     * Calculates which address would the Entry have if it was committed
     */
    fn address(&self) -> ZomeApiResult<Address> {
        hdk::entry_address(&self.clone().entry())
    }

    /**
     * Tries to convert an Entry to the deserialized struct
     * Returns None if the Entry is not of type struct
     */
    fn from_entry(entry: &Entry) -> Option<Self> {
        if let Entry::App(entry_type, attestation_entry) = entry {
            if entry_type.to_string() == Self::entry_type() {
                if let Ok(t) = Self::try_from(attestation_entry.clone()) {
                    return Some(t);
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use hdk::holochain_json_api::{json::JsonString, error::JsonError};
    use super::HolochainEntry;

    #[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
    pub struct TestEntry {
        contents: String,
    }

    impl HolochainEntry for TestEntry {
        fn entry_type() -> String {
            String::from("test")
        }
    }

    #[test]
    fn serialize_and_deserialize() {
        let test = TestEntry {
            contents: String::from("testing crate")
        };

        let test_entry = test.clone().entry();

        let maybe_test = TestEntry::from_entry(&test_entry);

        assert!(maybe_test.is_some());
        assert_eq!(maybe_test.unwrap().contents, test.contents);
    }
}
