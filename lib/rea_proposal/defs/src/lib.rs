/**
 * Holo-REA proposal zome entry type definitions
 *
 * For use in the standard Holo-REA proposal zome,
 * or in zomes wishing to embed additional attributes & logic alongside the
 * standard `Proposal` data model.
 *
 * @package Holo-REA
 */
use hdk::prelude::*;

use hc_zome_rea_proposal_storage_consts::*;
use hc_zome_rea_proposal_storage::Entry;

pub fn entry_def() -> ValidatingEntryType {
    entry!(
        name: PROPOSAL_ENTRY_TYPE,
        description: "Published requests or offers, sometimes with what is expected in return.",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |_validation_data: hdk::EntryValidationData<Entry>| {
            Ok(())
        }
    )
}

pub fn base_entry_def() -> ValidatingEntryType {
    entry!(
        name: PROPOSAL_BASE_ENTRY_TYPE,
        description: "Base anchor for initial proposal addresses to provide lookup functionality",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |_validation_data: hdk::EntryValidationData<Address>| {
            Ok(())
        },
        links: [
            // :TODO: replace with final link definitions
            to!(
                PROPOSAL_ENTRY_TYPE,
                link_type: PROPOSAL_INITIAL_ENTRY_LINK_TYPE,
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