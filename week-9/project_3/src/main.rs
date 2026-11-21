use std::fs::File;
use std::io::Write;

fn main (){
    let commissioners = [
    "Algbogun Alamba Daudu",
    "Murtala Afeez Bendu",
    "Okorocha Calistus Ogbona",
    "Adewale Jimoh Akanbi",
    "Osazuwa Faith Etieye",
    ];

    let ministry =[
    "Internal Affairs",
    "Justice",
    "Defense",
    "Power and Steel",
    "Petroleum",
    ];

    let zone = [
    "South West",
    "North East",
    "South South",
    "South West",
    "South East",
    ];

    let mut com_1 = File::create("commissioners.csv").unwrap();
    let mut min_1 = File::create("ministry.csv").unwrap();
    let mut zon_1 = File::create("zone.csv").unwrap();

    com_1.write_all(commissioners.as_bytes()).unwrap();
    min_1.write_all(ministry.as_bytes()).unwrap();
    zon_1.write_all(zone.as_bytes()).unwrap();



}