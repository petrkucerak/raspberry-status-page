#[macro_use]
extern crate rocket;
use rocket::response::content::RawHtml;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use sysinfo::System;

#[derive(Serialize)]
struct RaspberryPiData {
    cpu_usage: f32,
    total_memory: u64,
    used_memory: u64,
    total_swap: u64,
    used_swap: u64,
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
                    const response = await fetch('/pi-data');
                    const data = await response.json();

                    document.getElementById('cpu-usage').innerText = data.cpu_usage.toFixed(2) + '%';
                    document.getElementById('total-memory').innerText = data.total_memory + ' KB';
                    document.getElementById('used-memory').innerText = data.used_memory + ' KB';
                    document.getElementById('total-swap').innerText = data.total_swap + ' KB';
                    document.getElementById('used-swap').innerText = data.used_swap + ' KB';
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
                <h2>CPU Usage: <span id="cpu-usage">Loading...</span></h2>
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

    // Calculate average CPU usage across all processors
    let cpu_usage: f32 =
        sys.cpus().iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / sys.cpus().len() as f32;

    Json(RaspberryPiData {
        cpu_usage,
        total_memory: sys.total_memory(),
        used_memory: sys.used_memory(),
        total_swap: sys.total_swap(),
        used_swap: sys.used_swap(),
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, pi_data])
}
