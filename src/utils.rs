use comfy_table::Table;

use crate::Weather;

pub fn set_table_style(weather: &Weather) {
    let mut table = Table::new();
    table
        .set_header(vec![
            "City",
            "Temperature",
            "Feels like",
            "Temp min",
            "Temp max",
            "Wind Speed",
        ])
        .add_row(vec![
            weather.name.clone(),
            convert_f_to_c(weather.main.temp).to_string(),
            convert_f_to_c(weather.main.feels_like).to_string(),
            convert_f_to_c(weather.main.temp_min).to_string(),
            convert_f_to_c(weather.main.temp_max).to_string(),
            weather.wind.speed.to_string(),
        ]);

    println!("{table}");
}

fn convert_f_to_c(f: f64) -> f64 {
    f64::trunc((f - 273.15) * 100.0) / 100.0
}
