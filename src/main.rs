#[macro_use]
extern crate rocket;
use rocket::fs::NamedFile;
use rocket::response::content::RawHtml;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;
use sysinfo::System;

#[derive(Serialize)]
struct RaspberryPiData {
    cpu_usage: f32,
    total_memory: u64,
    total_memory_size: String,
    used_memory: u64,
    used_memory_size: String,
    total_swap: u64,
    total_swap_size: String,
    used_swap: u64,
    used_swap_size: String,
    cpu_temperature: f32,
    uptime: String,
    name: Option<String>,
}

fn read_cpu_temperature() -> io::Result<f32> {
    let mut file = File::open("/sys/class/thermal/thermal_zone0/temp")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Convert the temperature from millidegrees Celsius to degrees Celsius
    let temp: f32 = contents.trim().parse::<f32>().unwrap_or(0.0) / 1000.0;
    Ok(temp)
}

fn formate_memory(value: u64) -> (u64, String) {
    let result = if value < 1000 {
        (value, "B".to_string())
    } else if value < 1000000 {
        (value / 1000, "KB".to_string())
    } else {
        (value / 1000000, "MB".to_string())
    };
    result
}

fn formate_time(value: u64) -> String {
    let days = value / 86400;
    let hours = (value % 86400) / 3600;
    let minutes = (value % 3600) / 60;
    let seconds = value % 60;

    let mut parts = Vec::new();

    if days > 0 {
        parts.push(format!("{} days", days));
    }
    if hours > 0 {
        parts.push(format!("{} hours", hours));
    }
    if minutes > 0 {
        parts.push(format!("{} minutes", minutes));
    }
    if seconds > 0 {
        parts.push(format!("{} seconds", seconds));
    }

    parts.join(", ")
}

#[get("/")]
fn index() -> RawHtml<String> {
    let html = r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Raspberry Pi Info</title>
            <link rel="icon" href="/status/favicon.ico" type="image/x-icon">
            <style>
                body {
                    font-family: Arial, sans-serif;
                    margin: 40px;
                }
                .data {
                    padding: 10px;
                    background-color: #f0f0f0;
                    margin-bottom: 20px;
                }
            </style>
            <script>
                async function fetchPiData() {
                    const response = await fetch('/status/pi-data');
                    const data = await response.json();

                    document.getElementById('cpu-usage').innerText = data.cpu_usage.toFixed(2) + '%';
                    document.getElementById('cpu-temperature').innerText = data.cpu_temperature.toFixed(2) + ' °C';
                    document.getElementById('uptime').innerText = data.uptime;
                    
                    document.getElementById('total-memory').innerText = data.total_memory + ' ' + data.total_memory_size;
                    document.getElementById('used-memory').innerText = data.used_memory + ' ' + data.used_memory_size;
                    document.getElementById('total-swap').innerText = data.total_swap + ' ' + data.total_swap_size;
                    document.getElementById('used-swap').innerText = data.used_swap + ' ' + data.used_swap_size;
                    
                    document.getElementById('name').innerText = data.name;
                }

                // Fetch data every 1 second
                setInterval(fetchPiData, 1000);

                // Fetch data when the page loads
                window.onload = fetchPiData;
            </script>
        </head>
        <body>
            <h1>Raspberry Pi System Information</h1>
            <div class="data">
                <h2>CPU Information</h2>
                <p>Uptime: <span id="uptime">Loading...</span></p>
                <p>CPU Usage: <span id="cpu-usage">Loading...</span></p>
                <p>CPU Temperature: <span id="cpu-temperature">Loading...</span></p>
            </div>
            <div class="data">
                <h2>Memory Information</h2>
                <p>Total Memory: <span id="total-memory">Loading...</span></p>
                <p>Used Memory: <span id="used-memory">Loading...</span></p>
                <p>Total Swap: <span id="total-swap">Loading...</span></p>
                <p>Used Swap: <span id="used-swap">Loading...</span></p>
            </div>
        </body>
        </html>
    "#;

    RawHtml(html.to_string())
}

#[get("/pi-data")]
fn pi_data() -> Json<RaspberryPiData> {
    let mut sys = System::new_all();
    sys.refresh_all();

    // Read CPU temperature
    let cpu_temperature = read_cpu_temperature().unwrap_or(0.0);

    let (total_memory, total_memory_size) = formate_memory(sys.total_memory());
    let (used_memory, used_memory_size) = formate_memory(sys.used_memory());
    let (total_swap, total_swap_size) = formate_memory(sys.total_swap());
    let (used_swap, used_swap_size) = formate_memory(sys.used_swap());

    Json(RaspberryPiData {
        cpu_usage: sys.global_cpu_usage(),
        total_memory: total_memory,
        total_memory_size: total_memory_size,
        used_memory: used_memory,
        used_memory_size: used_memory_size,
        total_swap: total_swap,
        total_swap_size: total_swap_size,
        used_swap: used_swap,
        used_swap_size: used_swap_size,
        cpu_temperature, // Include CPU temperature in the response
        uptime: formate_time(System::uptime()),
        name: System::host_name(),
    })
}

#[get("/favicon.ico")]
async fn favicon() -> Option<NamedFile> {
    NamedFile::open(PathBuf::from("../static/favicon.ico"))
        .await
        .ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/status", routes![index, pi_data, favicon])
}
