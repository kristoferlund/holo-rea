/**
 * Holo-REA agreement zome internal data structures
 *
 * Required by the zome itself, and for any DNA-local zomes interacting with its
 * storage API directly.
 *
 * @package Holo-REA
 */
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use holochain_json_api::{ json::JsonString, error::JsonError };
use holochain_json_derive::{ DefaultJson };

use hdk_graph_helpers::{
    record_interface::Updateable,
};

use vf_core::type_aliases::{
    Timestamp,
};

use hc_zome_rea_agreement_rpc::{ CreateRequest, UpdateRequest };

//---------------- RECORD INTERNALS & VALIDATION ----------------

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Entry {
    pub name: Option<String>,
    pub created: Option<Timestamp>,
    pub note: Option<String>,
}

//---------------- CREATE ----------------

/// Pick relevant fields out of I/O record into underlying DHT entry
impl From<CreateRequest> for Entry {
    fn from(e: CreateRequest) -> Entry {
        Entry {
            name: e.name.into(),
            created: e.created.into(),
            note: e.note.into(),
        }
    }
}

//---------------- UPDATE ----------------

/// Handles update operations by merging any newly provided fields
impl Updateable<UpdateRequest> for Entry {
    fn update_with(&self, e: &UpdateRequest) -> Entry {
        Entry {
            name: if !e.name.is_some() { self.name.to_owned() } else { e.name.to_owned().into() },
            created: if !e.created.is_some() { self.created.to_owned() } else { e.created.to_owned().into() },
            note: if !e.note.is_some() { self.note.to_owned() } else { e.note.to_owned().into() },
        }
    }
}
