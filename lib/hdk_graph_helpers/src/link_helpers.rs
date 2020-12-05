/**
 * Link-handling abstractions for Holochain apps
 *
 * Handles common behaviours for linking between data in different hApps,
 * in a way that is predictable, semantically meaningful and easy to reason about.
 *
 * @package HoloREA
 * @since   2019-07-03
 */

use hdk3::prelude::*;
use hdk3::prelude::link::Link;

use crate::{
    GraphAPIResult,
};

// HDK re-exports
pub use hdk3::prelude::create_link;
pub use hdk3::prelude::delete_link;

//--------------------------------[ READ ]--------------------------------------

/// Load a set of addresses of type `T` and automatically coerce them to the
/// provided newtype wrapper.
///
/// The `Address` array is returned wrapped in a copy-on-write smart pointer
/// such that it can be cheaply passed by reference into Serde output
/// structs & functions via `Cow::into_owned`.
///
/// :TODO: propagate errors in link retrieval
///
/// @see `type_aliases.rs`
///
pub fn get_linked_addresses_as_type<'a, T, I>(
    base_address: I,
    link_type: &str,
    link_tag: &str,
) -> Cow<'a, Vec<T>>
    where T: From<Address> + Clone, I: AsRef<Address>
{
    let addrs = get_linked_addresses(base_address.as_ref(), link_type, link_tag);
    if let Err(_get_links_err) = addrs {
        return Cow::Owned(vec![]);  // :TODO: improve error handling
    }

    Cow::Owned(addrs.unwrap().iter()
        .map(|addr| { T::from(addr.to_owned()) })
        .collect())
}

/// Similar to `get_linked_addresses_as_type` except that the returned addresses
/// refer to remote records from external networks.
///
pub fn get_linked_addresses_with_foreign_key_as_type<'a, T, I>(
    base_address: I,
    link_type: &str,
    link_tag: &str,
) -> Cow<'a, Vec<T>>
    where T: From<Address> + Clone, I: AsRef<Address>
{
    let addrs = get_linked_addresses(base_address.as_ref(), link_type, link_tag);
    if let Err(_get_links_err) = addrs {
        return Cow::Owned(vec![]);  // :TODO: improve error handling
    }

    Cow::Owned(addrs.unwrap().iter()
        .filter_map(|addr| {
            let base_addr_request = get_key_index_address(&addr);
            match base_addr_request {
                Ok(remote_addr) => Some(T::from(remote_addr)),
                Err(_) => None,     // :TODO: handle errors gracefully
            }
        })
        .collect())
}

/// Load any set of linked `EntryHash`es being referenced from the
/// provided `base_address` with the given `link_tag`.
///
pub (crate) fn get_linked_addresses(
    base_address: &EntryHash,
    link_tag: LinkTag,
) -> GraphAPIResult<Vec<EntryHash>> {
    pull_links_data(base_address, link_tag, get_link_target_entry)
}

/// Load any set of linked `HeaderHash`es being referenced from the
/// provided `base_address` with the given `link_tag`.
///
/// Required to retrieve link headers for executing deletions.
///
pub (crate) fn get_linked_headers(
    base_address: &EntryHash,
    link_tag: LinkTag,
) -> GraphAPIResult<Vec<HeaderHash>> {
    pull_links_data(base_address, link_tag, get_link_target_header)
}

//-----------------------------------------------------

// :TODO: ensure ordering is latest-first
fn pull_links_data<T, F>(
    base_address: &EntryHash,
    link_tag: LinkTag,
    link_map: F,
) -> GraphAPIResult<Vec<T>>
    where F: Fn(&Link) -> T,
{
    let links_result = get_links((*base_address).clone(), Some(link_tag))?;

    Ok(links_result
        .into_inner()
        .iter()
        .map(link_map)
        .collect()
    )
}

fn get_link_target_entry(l: &Link) -> EntryHash {
    l.target.clone()
}

fn get_link_target_header(l: &Link) -> HeaderHash {
    l.create_link_hash.clone()
}
