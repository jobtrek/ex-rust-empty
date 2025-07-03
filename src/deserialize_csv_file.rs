use csv::Reader;
use serde::Deserialize;

/// This structure represent a record of 1 month of energy production of 1 type of energy in switzerland.
#[derive(Deserialize)]
pub struct EnergyProduction {
    #[serde(rename = "Datum")]
    pub month: String,
    #[serde(rename = "Energietraeger")]
    pub source: String,
    #[serde(rename = "Produktion_GWh")]
    pub amount: f64,
}

/// This function should load the given file in memory, and deserialize it to the
/// Energy production struct
///
/// ```
/// let energies = rust_ex::deserialize_csv_file::read_given_file_and_return_vec_of_structs(rust_ex::CSV_FILE_LOCATION);
/// assert_eq!(energies.iter().count(), 22190);
/// ```
pub fn read_given_file_and_return_vec_of_structs(filepath: &str) -> Vec<EnergyProduction> {
    // Write your code here
    let mut reader = Reader::from_path(filepath).expect("Cannot find file");
    reader
        .deserialize::<EnergyProduction>()
        .map(|energy| energy.expect("Cannot deserialize"))
        .collect()
}