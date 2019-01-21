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
    // to,
    // AGENT_ADDRESS,
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
struct Message {
    content: String
}

fn handle_create_message(message: Message, user_address: Address) -> JsonString {
    let entry = Entry::App("message".into(), message.into());
    let res = match hdk::commit_entry(&entry) {
        Ok(address) => {
            match hdk::link_entries(
                &user_address,
                &address,
                "sent"
            ) {
                Ok(_) => json!({"success": true, "address": address}),
                Err(err) => json!({ "success": false, "error": err})
            }
        },
        Err(hdk_err) => json!({"success": false, "error": hdk_err}),
    };
    res.into()
}

// fn handle_send_message(message_address: Address, user_address: Address) -> JsonString {
//     let res = match hdk::link_entries(
//         &user_address,
//         &message_address,
//         "sent_to"
//     ) {
//         Ok(_) => json!({"success": true}),
//         Err(err) => json!({ "success": false, "error": err})
//     };
//     res.into()
// }

fn handle_get_all_messages_by(user_address: Address) -> JsonString {
    let res = match hdk::get_links(&user_address, "sent") {
        Ok(result) => {
            let messages: Vec<Message> = result
                .addresses()
                .iter()
                .map(|address| hdk::get_entry(&address))
                .filter_map(|i| i.ok())
                .map(|r| match r {
                    Some(Entry::App(_, value)) => Some(value),
                    _ => None
                })
                .filter_map(|i| i)
                .map(|i| Message::try_from(i))
                .filter_map(|i| i.ok())
                .collect();
            // let result : Result<Option<Metric>,_> = hdk::get_entry(user_address.clone());
            json!({
                "success": true,
                "messages": messages
            })
        }
        Err(hdk_error) => json!({
            "success": false,
            "error": hdk_error
        }),
    };
    res.into()
}


// fn handle_get_all_messages_to(user_address: Address) -> JsonString {
//     let res = match hdk::get_links(&user_address, "sent_to") {
//         Ok(result) => {
//             let messages: Vec<Message> = result
//                 .addresses()
//                 .iter()
//                 .map(|address| hdk::get_entry(address.clone()))
//                 .filter_map(|i| i.ok())
//                 .filter_map(|i| i)
//                 .map(|i| Message::try_from(i.value()))
//                 .filter_map(|i| i.ok())
//                 .collect();
//             // let result : Result<Option<Metric>,_> = hdk::get_entry(user_address.clone());
//             json!({
//                 "success": true,
//                 "messages": messages
//             })
//         }
//         Err(hdk_error) => json!({
//             "success": false,
//             "error": hdk_error
//         }),
//     };

//     res.into()
// }

define_zome! {
    entries: [
       entry!(
           name: "message",
           description: "This will be a single commitment an individual makes.",
           sharing: Sharing::Public,
           native_type: Message,
           validation_package: || hdk::ValidationPackageDefinition::Entry,
           validation: |message: Message, validation_data: hdk::ValidationData| {
               Ok(())
           },
           links: [
            //    from!(
            //        "user",
            //         tag: "sent_by",

            //         validation_package: || {
            //             hdk::ValidationPackageDefinition::ChainFull
            //         },

            //         validation: |base: Address, target: Address, _ctx: hdk::ValidationData| {
            //             Ok(())
            //         }
            //    ),
               from!(
                   "user",
                    tag: "sent",

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
            // create user
            create_message: {
                inputs: |message: Message, user_address: Address|,
                outputs: |result: JsonString|,
                handler: handle_create_message
            }

            // send_message: {
            //     inputs: |message_address: Address, user_address: Address|,
            //     outputs: |result: JsonString|,
            //     handler: handle_send_message
            // }

            get_all_messages_by: {
                inputs: |user_address: Address|,
                outputs: |result: JsonString|,
                handler: handle_get_all_messages_by
            }

            // get_all_messages_to: {
            //     inputs: |user_address: Address|,
            //     outputs: |result: JsonString|,
            //     handler: handle_get_all_messages_to
            // }

            // fetch user

            // fetch all users

            // update user (on hold while core dev team implements update_entry)

        }
    }
}

// see https://holochain.github.io/rust-api/0.0.1/hdk/ for info on using the hdk library
