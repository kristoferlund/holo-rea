#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate hdk_graph_helpers;
extern crate vf_planning;
mod satisfaction_requests;

use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
    holochain_persistence_api::{
        cas::content::Address,
    },
    holochain_core_types::{
        dna::entry_types::Sharing,
    },
    holochain_json_api::{
        json::JsonString,
        error::JsonError,
    },
};

use vf_planning::satisfaction::{
    Entry,
    CreateRequest,
    UpdateRequest,
    ResponseData as Response,
};
use satisfaction_requests::{
    QueryParams,
    receive_create_satisfaction,
    receive_get_satisfaction,
    receive_update_satisfaction,
    receive_delete_satisfaction,
    receive_query_satisfactions,
};
use vf_observation::identifiers::{
    EVENT_BASE_ENTRY_TYPE,
};
use vf_planning::identifiers::{
    SATISFACTION_BASE_ENTRY_TYPE,
    SATISFACTION_INITIAL_ENTRY_LINK_TYPE,
    SATISFACTION_ENTRY_TYPE,
    SATISFACTION_SATISFIEDBY_LINK_TYPE,
};

// Entry type definitions

fn satisfaction_entry_def() -> ValidatingEntryType {
    entry!(
        name: SATISFACTION_ENTRY_TYPE,
        description: "Represents many-to-many relationships between intents and economic events that fully or partially satisfy one or more intents.",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |_validation_data: hdk::EntryValidationData<Entry>| {
            Ok(())
        }
    )
}

fn satisfaction_base_entry_def() -> ValidatingEntryType {
    entry!(
        name: SATISFACTION_BASE_ENTRY_TYPE,
        description: "Base anchor for initial satisfaction addresses to provide lookup functionality",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |_validation_data: hdk::EntryValidationData<Address>| {
            Ok(())
        },
        links: [
            to!(
                SATISFACTION_ENTRY_TYPE,
                link_type: SATISFACTION_INITIAL_ENTRY_LINK_TYPE,

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: | _validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                EVENT_BASE_ENTRY_TYPE,
                link_type: SATISFACTION_SATISFIEDBY_LINK_TYPE,

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: | _validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            )
        ]
    )
}

// Zome definition

define_zome! {
    entries: [
       satisfaction_entry_def(),
       satisfaction_base_entry_def()
    ]

    init: || { Ok(()) }

    validate_agent: |validation_data : EntryValidationData::<AgentId>| {
        Ok(())
    }

    receive: |from, payload| {
      format!("Received: {} from {}", payload, from)
    }

    functions: [
        satisfaction_created: {
            inputs: |satisfaction: CreateRequest|,
            outputs: |result: ZomeApiResult<Response>|,
            handler: receive_create_satisfaction
        }
        satisfaction_updated: {
            inputs: |satisfaction: UpdateRequest|,
            outputs: |result: ZomeApiResult<Response>|,
            handler: receive_update_satisfaction
        }
        satisfaction_deleted: {
            inputs: |address: Address|,
            outputs: |result: ZomeApiResult<bool>|,
            handler: receive_delete_satisfaction
        }

        get_satisfaction: {
            inputs: |address: Address|,
            outputs: |result: ZomeApiResult<Response>|,
            handler: receive_get_satisfaction
        }
        query_satisfactions: {
            inputs: |params: QueryParams|,
            outputs: |result: ZomeApiResult<Vec<Response>>|,
            handler: receive_query_satisfactions
        }
    ]

    traits: {
        hc_public [
            satisfaction_created,
            satisfaction_updated,
            satisfaction_deleted,

            get_satisfaction,
            query_satisfactions
        ]
    }
}