#![feature(try_from)]
use serde_derive::{Serialize, Deserialize};
use holochain_core_types_derive::DefaultJson;
use serde_json::json;

use hdk::{
    define_zome,
    entry,
    load_json,
    from,
    link,
    to,
    AGENT_ADDRESS,
    // error::ZomeApiError,
    holochain_core_types::{
        dna::entry_types::Sharing,
        json::JsonString,
        // json::DefaultJson,
        entry::Entry,
        error::HolochainError,
        cas::content::Address
    },
};

use std::convert::TryFrom;

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
               ),
               to!(
                   "message",
                    tag: "received",

                    validation_package: || {
                        hdk::ValidationPackageDefinition::ChainFull
                    },

                    validation: |base: Address, target: Address, _ctx: hdk::ValidationData| {
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
            receive_message: {
                inputs: |message_address: Address|,
                outputs: |result: JsonString|,
                handler: handle_receive_message
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
    let entry = Entry::App("user".into(), user.into());
    let agent_address = &Address::from(AGENT_ADDRESS.to_string());
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
    let agent_address = &Address::from(hdk::AGENT_ADDRESS.to_string());
    let res = match hdk::get_links(&agent_address, "user_data") {
        Ok(result) => {
            let user_address = &result.addresses()[0];
            let result = hdk::get_entry(&user_address);
            match result {
                Ok(Some(Entry::App(_, value))) => json!({
                    "success": true,
                    "user": User::try_from(value).unwrap(),
                    "address": user_address.clone()
                }),
                Ok(_) =>  json!({"success": false, "err": "No entry found"}),
                Err(err) => json!({"success": false, "error": err}),
            }
        },
        Err(hdk_error) => json!({"success": false, "err": hdk_error}),
    };

    res.into()
}

fn handle_receive_message(message_address: Address) -> JsonString {
    
    let agent_address = &Address::from(hdk::AGENT_ADDRESS.to_string());
    let res = match hdk::get_links(&agent_address, "user_data") {
        Ok(result) => {
            let user_address = result.addresses()[0].clone();
            match hdk::link_entries(
                &user_address,
                &message_address,
                "received"
            ) {
                Ok(_) => json!({"success": true}),
                Err(err) => json!({ "success": false, "error": err})
            }
        },
        Err(hdk_error) => json!({"success": false, "err": hdk_error}),
    };

    res.into()

}