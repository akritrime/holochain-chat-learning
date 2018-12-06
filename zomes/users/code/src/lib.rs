#![feature(try_from)]
#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate holochain_core_types;
#[macro_use]
extern crate holochain_core_types_derive;

use std::convert::TryFrom;

use hdk::{
    holochain_core_types::{
        dna::zome::entry_types::Sharing,
        hash::HashString,
        json::JsonString,
        entry::Entry,
        entry::entry_type::EntryType,
        error::HolochainError,
        cas::content::Address,
    },
    AGENT_ADDRESS
};
#[derive(Serialize, Deserialize, Debug, DefaultJson)]
struct User {
    handle: String,
    email: String,
}

define_zome! {
    entries: [
       entry!(
           name: "user",
           description: "Stores info about a user",
           sharing: Sharing::Public,
           native_type: User,
           validation_package: || hdk::ValidationPackageDefinition::Entry,
           validation: |user: User, validation_data: hdk::ValidationData| {
               Ok(())
           },
           links: [
               from!(
                    "%agent_id",
                    tag: "user_data",
                    validation_package: || {
                        hdk::ValidationPackageDefinition::ChainFull
                    },
                    validation: |_source: Address, _target: Address, _ctx: hdk::ValidationData| {
                        Ok(())
                    }
               )
           ]
       )
]
    genesis: || {
        Ok(())
    }

    functions: {
        main (Public) {
            create_user: {
                inputs: |user: User|,
                outputs: |result: JsonString|,
                handler: handle_create_user
            }
            get_current_user: {
                inputs: | |,
                outputs: |result: JsonString|,
                handler: handle_get_current_user
            }

            // get_all_user: {
            //     inputs: | |,
            //     outputs: |result: JsonString|,
            //     handler: handle_get_all_user
            // }
            // update user (on hold while core dev team implements update_entry)
        }
    }
}

fn handle_create_user(user: User) -> JsonString {
    let entry = Entry::new(EntryType::App("user".into()), user);
    let agent_address = &HashString::from(AGENT_ADDRESS.to_string());
    match hdk::commit_entry(&entry) {
        Ok(address) => {
            match hdk::link_entries(&agent_address, &address, "user_data") {
                Ok(_) => json!({"success": true, "address": address}).into(),
                Err(hdk_err) => json!({"success": false, "error": hdk_err}).into()
            }
            
        }

        Err(hdk_err) => json!({"success": false, "error": hdk_err, "commit": true}).into(),
    }
}

fn handle_get_current_user() -> JsonString {
    let agent_address = &HashString::from(hdk::AGENT_ADDRESS.to_string());
    let res = match hdk::get_links(&agent_address, "user_data") {
        Ok(result) => {
            let user_address = result.addresses()[0].clone();
            let result = hdk::get_entry(user_address.clone());
            match result {
                Ok(Some(user)) => json!({
                    "success": true,
                    "user": User::try_from(user.value().clone()).unwrap(),
                    "address": user_address
                }),
                Ok(None) =>  json!({"success": false, "err": "No entry found"}),
                Err(err) => json!({"success": false, "error": err}),
            }
        },
        Err(hdk_error) => json!({"success": false, "err": hdk_error}),
    };

    res.into()
}