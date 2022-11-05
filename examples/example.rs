use core::str::from_utf8;

use serde_json_core::ser::to_slice;
use serde_json_core::ser::to_string;
use serde_json_core::ser::Error;
use wot_td_embedded::models::thing_description::ThingDescription;

fn main() -> Result<(), Error> {
    let thing_description = ThingDescription::builder()
        .title("Test")
        .description("Test Description")
        .build();

    // let mut buf: [u8; 200] = [0; 200];

    // let length = to_slice::<ThingDescription>(&thing_description, &mut buf)?;
    // println!("{:?}", &buf[0..length]);

    // const CHUNK_SIZE: usize = 10;

    // if let Ok(length) = thing_description.to_json(&mut buf) {
    //     println!("{:?}", &buf[0..length]);
    // };

    // let blah = to_string::<ThingDescription, 100>(&thing_description)?;
    // let mut chunks = buf[0..length].chunks(CHUNK_SIZE);

    // let mut blah = "Test";

    // let chunk = chunks.nth(0);
    // println!("{:?}", chunk);

    // for chunk in chunks {
    //     println!("{:?}", chunk);
    //     blah = from_utf8(chunk).unwrap();
    //     println!("{:?}", blah);
    // }

    // println!(
    //     "{}",
    //     to_string::<ThingDescription, 100>(&thing_description)?
    // );

    println!("{}", &thing_description.title);
    println!("{:?}", &thing_description.description);

    Ok(())
}
