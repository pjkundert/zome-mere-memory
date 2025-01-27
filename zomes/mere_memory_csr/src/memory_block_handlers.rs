use crate::{
    MemoryBlockEntry,
    ToInput,
};

use hdk::prelude::*;


#[hdk_extern]
pub fn create_memory_block_entry(block: MemoryBlockEntry) -> ExternResult<EntryHash> {
    debug!("Creating 'MemoryBlockEntry' ({}/{}): {}", block.sequence.position, block.sequence.length, block.bytes.len() );

    let action_hash = create_entry( block.to_input() )?;
    let details = get_details(action_hash, GetOptions::default())?.ok_or(
        wasm_error!(WasmErrorInner::Guest("Could not get action details".into()))
    )?;
    
    match details {
        Details::Record(record_details) => Ok(record_details.record.action().entry_hash().ok_or(
            wasm_error!(WasmErrorInner::Guest("Expected entry hash".into()))
        )?.clone()),
        _ => Err(wasm_error!(WasmErrorInner::Guest("Expected record".into())))
    }
}



#[hdk_extern]
pub fn get_memory_block_entry(addr: EntryHash) -> ExternResult<MemoryBlockEntry> {
    debug!("Get 'MemoryBlockEntry': {}", addr );
    let record = get( addr.clone(), GetOptions::network() )?
	.ok_or(wasm_error!(WasmErrorInner::Guest(format!("Entry not found for address: {}", addr ))))?;
    let block = MemoryBlockEntry::try_from( &record )?;

    Ok(	block )
}
