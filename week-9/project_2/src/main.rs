use std::fs::File;
use std::io::Write;

fn main() {
let records = [
"Student Name, Matric.Number, Department, Level",
"Oluchi Mordi, ACC10211111, Accounting, 300",
"Adamas Aliyu, ECO10110101, Economics, 100",
"Shania Bolade, CSC10328828, Computer, 200",
"Adekunle Gold, EEE11020202, Electrical, 200",
"Blanca Edemah, MEE10202001, Mechanical, 100",
];

let mut file = File::create("SMIS.csv").unwrap();
file.write_all("PAU Student Management Information System\n".as_bytes()).unwrap();
for record in records {
let line = format!("{}\n", record);
file.write_all(line.as_bytes()).unwrap();
}
println!("Data has been sucdessfully saved");
}