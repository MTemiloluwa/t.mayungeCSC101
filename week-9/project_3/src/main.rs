use std::fs::File;
use std::io::Write;

fn main (){
    let header = ["Commissioner","Ministry","Zone"];
    let min1 = ["Algbogun Alamba Daudu","Internal Affairs","South west"];
    let min2 = ["Murtala Afeez Bendu","Justice","North East"];
    let min3 = ["Okoruocha Calistus Ogbona","Defense","South South"];
    let min4 = ["Adewale Jimoh Akanbi","Power & Steel","South West"];
    let min5 = ["Osazuwa Faith Etieye","Petroleum","South East"];

    println!("{:?}", header);
    println!("{:?}", min1);
    println!("{:?}", min2);
    println!("{:?}", min3);
    println!("{:?}", min4);
    println!("{:?}", min5);

    let mut file = File::create("EFCC.csv").unwrap();
    file.write_all("Convicted Ministers\n".as_bytes()).unwrap();
    let rows = [header, min1, min2, min3, min4, min5];
    for row in rows{
        let dataset = format!("{}, {}, {},\n", row[0], row[1], row[2]);
        file.write_all(dataset.as_bytes()).unwrap();
    }
    println!("Data has been successfully saved");
}