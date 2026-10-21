use std::io::Write;

fn main() {
    let lager_drink = "Lager: 33 Export, Desperados, Goldberg, Gulder, Heineken, Star\n";
    let stout_drink = "Stout: Legend, Turbo King, Williams";
    let non_alcoholic_drink = "Non-alcoholic: Maltina, Amsel Malta, Malta Gold, Fayrouz";



    let mut file = std::fs::File::create("NaijaBrew.csv").expect("create failed");
    file.write_all("These are the categories of Drinks for Nigerian Breweries Limited\n"
        .as_bytes()).expect("write failed");
    file.write_all(stout_drink.as_bytes()).expect("write failed");
    file.write_all( lager_drink.as_bytes()).expect("write failed");
    file.write_all( non_alcoholic_drink.as_bytes()).expect("write failed");
    println!("\nData has been successfully written to file" );


}

