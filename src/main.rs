// ---------------------------------------
// IOracle server
// ---------------------------------------
use rocket::{form::Form, response::Redirect, State};
use rocket_dyn_templates::Template;
use std::{env, process};

#[macro_use]
extern crate rocket;

pub mod iching;
pub mod wires;

// #[derive(Debug)]
struct Config {
    multy: f32,
    bias: f32,
    threshold: f32,
    port: std::sync::Arc<std::sync::Mutex<Box<dyn serialport::SerialPort>>>,
}

impl Config {
    pub fn new() -> Result<Config, env::VarError> {
        let multy: f32 = env::var("MULTY")
            .map(|m| m.parse())
            .map_err(|_| env::VarError::NotPresent)?
            .map_err(|_| env::VarError::NotPresent)?;

        let bias: f32 = env::var("BIAS")
            .map(|b| b.parse())
            .map_err(|_| env::VarError::NotPresent)?
            .map_err(|_| env::VarError::NotPresent)?;

        let threshold: f32 = env::var("THRESHOLD")
            .map(|t| t.parse())
            .map_err(|_| env::VarError::NotPresent)?
            .map_err(|_| env::VarError::NotPresent)?;

        let port = serialport::new(wires::SERIAL_DEV, wires::SERIAL_RATE)
            .timeout(std::time::Duration::from_millis(100))
            .open()
            .map_err(|_| env::VarError::NotPresent)?;

        // let port = serialport::new(wires::SERIAL_DEV, wires::SERIAL_RATE)
        //     .open_native()
        //     .map_err(|_| env::VarError::NotPresent)?;

        // {
        //     println!("serial: ok");
        //     let mut serial_buf: Vec<u8> = vec![0; 32];
        //     if let Err(e) = port.read(serial_buf.as_mut_slice()) {
        //         println!("Found no data! {:?}", e);
        //     } else {
        //         println!("ok2 {:?}", serial_buf);
        //     }
        // };

        let port = std::sync::Arc::new(std::sync::Mutex::new(port));

        Ok(Self {
            multy,
            bias,
            threshold,
            port,
        })
    }
}

#[derive(Debug, FromForm)]
struct FormData {
    question: String,
}

#[get("/")]
fn home() -> Template {
    Template::render("home", rocket_dyn_templates::context! {})
}

#[post("/question", data = "<form_data>")]
async fn question(form_data: Form<FormData>, config: &State<Config>) -> Redirect {
    // Read data from serial port with config
    let first_reading = wires::read(
        config.port.clone(),
        config.multy,
        config.bias,
        config.threshold,
    );
    wires::react(first_reading);

    // Read data from serial port with config
    let second_reading = wires::read(
        config.port.clone(),
        config.multy,
        config.bias,
        config.threshold,
    );
    wires::react(second_reading);

    let hexagram = iching::Hexagram::new(first_reading, second_reading);
    let new_answer = iching::Answer::new(hexagram, form_data.question.to_owned());
    let new_answer_id = new_answer.save();
    Redirect::to(format!("/answer/{}", new_answer_id))
}

#[get("/answer/<id>")]
fn answer(id: u64) -> Template {
    let answer = iching::Answer::get_by_id(id);
    Template::render(
        "answer",
        rocket_dyn_templates::context! { answer: answer.answer },
    )
}

#[catch(404)]
pub fn not_found() -> Redirect {
    Redirect::to("/")
}

#[catch(500)]
pub fn internal_error() -> Redirect {
    Redirect::to("/")
}

#[launch]
fn rocket() -> _ {
    let config = Config::new().unwrap_or_else(|_| {
        println!("Environment variable not found!");
        process::exit(1);
    });

    rocket::build()
        .mount("/", routes![home, question, answer])
        .register("/", catchers![not_found, internal_error])
        .attach(Template::fairing())
        .manage(config)
}
