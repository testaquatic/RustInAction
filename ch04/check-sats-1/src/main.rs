use check_sats_1::{
    cube_sat::fetch_sat_ids,
    ground_station::GroundStation,
    mailbox::{MailBox, Message},
};

fn main() {
    let mut mail = MailBox::new();

    let base = GroundStation;

    let sat_ids = fetch_sat_ids();

    sat_ids
        .into_iter()
        .map(|sat_id| {
            base.connect(sat_id);
            Message::new(sat_id, String::from("hello"))
        })
        .for_each(|msg| base.send(&mut mail, msg));

    let sat_ids = fetch_sat_ids();

    sat_ids.into_iter().for_each(|sat_id| {
        let sat = base.connect(sat_id);
        let msg = sat.recv(&mut mail);

        println!("{:?}: {:?}", sat, msg)
    })
}
