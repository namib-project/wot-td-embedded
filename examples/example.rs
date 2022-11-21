use core::str::from_utf8;

use serde_json_core::ser::to_slice;
use serde_json_core::ser::to_string;
use serde_json_core::ser::Error;
use wot_td_embedded::data_structures::Map;
use wot_td_embedded::models::action::Action;
use wot_td_embedded::models::form::Form;
use wot_td_embedded::models::thing_description::ThingDescription;

fn main() -> Result<(), Error> {
    let thing_description = ThingDescription::builder()
        .title("Test")
        .description("Test Description")
        .actions(Map::new(vec![(
            "test",
            Action::builder(vec![Form::builder("hello").build()])
                .title("Test-Action")
                .build(),
        )]))
        .build();

    let mut buf: [u8; 200] = [0; 200];

    let length = to_slice::<ThingDescription>(&thing_description, &mut buf)?;

    const CHUNK_SIZE: usize = 10;

    let string_output = to_string::<ThingDescription, 200>(&thing_description)?;
    println!("{}", string_output);
    let chunks = buf[0..length].chunks(CHUNK_SIZE);

    for chunk in chunks {
        println!("{}", from_utf8(chunk).unwrap());
    }

    println!(
        "{}",
        to_string::<ThingDescription, 200>(&thing_description)?
    );

    println!(
        "{}",
        &thing_description
            .actions
            .unwrap()
            .find_value("test")
            .unwrap()
            .title
            .unwrap()
    );

    Ok(())
}
