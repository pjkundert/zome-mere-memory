use crate::{
    MemoryBlockEntry,
    ToInput,
};

use hdk::prelude::*;

fn get_entry_hash_for_action_hash_local(action_hash: &ActionHash, options: GetOptions) -> ExternResult<Option<EntryHash>> {
    Ok(get_details(action_hash.to_owned(), options)?
       .and_then(|details| match details {
	   Details::Record(record_details) => record_details.record.action().entry_hash().cloned(),
	   _ => None,
       }))
}


#[hdk_extern]
pub fn create_memory_block_entry(block: MemoryBlockEntry) -> ExternResult<EntryHash> {
    debug!("Creating 'MemoryBlockEntry' ({}/{}): {}", block.sequence.position, block.sequence.length, block.bytes.len() );
    let action_hash = create_entry( block.to_input() )?;
    let entry_hash = get_entry_hash_for_action_hash_local( &action_hash, GetOptions::local() )?
        .ok_or(wasm_error!(WasmErrorInner::Guest(format!("Entry hash not found for Action: {}", action_hash ))))?;

    Ok(entry_hash)
}


#[hdk_extern]
pub fn get_memory_block_entry(addr: EntryHash) -> ExternResult<MemoryBlockEntry> {
    debug!("Get 'MemoryBlockEntry': {}", addr );
    let record = get( addr.clone(), GetOptions::network() )?
	.ok_or(wasm_error!(WasmErrorInner::Guest(format!("Entry not found for address: {}", addr ))))?;
    let block = MemoryBlockEntry::try_from( &record )?;

    Ok(	block )
}
