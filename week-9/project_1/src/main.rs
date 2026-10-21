use std::io::Write;


fn main() {
    let header = "Lager, Stout, Non-Alcoholic";
    let row1 = "\n33 Export, Legend, Maltina";
    let row2 = "\nDesperados, Turbo King, Amsel Malta";
    let row3 = "\nGoldberg, Williams, Malta Gold";
    let row4 = "\nGulder, , Fayrouz";
    let row5 = "\n Heineken, , ";
    let row6 ="\nStar, , ";



    let mut file = std::fs::File::create("naija_Brew.csv").expect("create failed");
    file.write_all("These are the categories of Drinks for Nigerian Breweries Limited\n"
        .as_bytes()).expect("write failed");
    file.write_all(header.as_bytes()).expect("write failed");
    file.write_all( row1.as_bytes()).expect("write failed");
    file.write_all( row2.as_bytes()).expect("write failed");
    file.write_all( row3.as_bytes()).expect("write failed");
    file.write_all( row4.as_bytes()).expect("write failed");
    file.write_all( row5.as_bytes()).expect("write failed");
    file.write_all( row6.as_bytes()).expect("write failed");

    println!("\nData has been successfully written to file" );


}

