use proto_non_optional::basic::{
    City,
    PlaneTicket,
    PlaneTicket_Airport,
};

fn main() {
    // Make our ticket, which has required fields!
    let ticket = PlaneTicket {
        passenger: "Parker".to_owned(),
        city: City::NEW_YORK,
        airport: PlaneTicket_Airport::Id(1),
    };

    println!("Our ticket: {:#?}", ticket);
}
