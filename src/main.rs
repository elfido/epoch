extern crate chrono;

use std::env;
use std::time;
use chrono::prelude::*;

struct Response {
    local_date: String,
    gmt_date: String,
}

impl Response {
    fn new(local: &str, gmt: &str) -> Response {
        return Response {
            local_date: local.to_string(),
            gmt_date: gmt.to_string()
        }
    }
}

fn get_date(from: &str) -> Result<Response, bool> {
    let requested_time : i64 ;
    let actual_time = if from.len() == 10 {
        format!("{}000", from)
    } else {
        from.to_string()
    };
    if let Ok(value) = actual_time.parse::<i64>() {
        requested_time = value;
    } else {
        return Err(false);
    }
    let seconds = requested_time / 1000;
    let nanos_factor = 1000000;
    let nanos_value = requested_time % 1000;
    let nanos = nanos_value as u32 * nanos_factor;
    let universal = Utc.timestamp(seconds, nanos);
    let local = Local.timestamp(seconds, nanos);
    Ok(Response::new(local.to_string().as_ref(), &universal.to_string().as_ref()))
}

fn main() {
    let args = env::args();
    if args.len() < 2 {
        match time::SystemTime::now().duration_since(time::SystemTime::UNIX_EPOCH) {
            Ok(n) => {
                println!("Current: {}", n.as_millis());
            },
            Err(err) => {
                println!("Cannot obtain current timestamp {}", err);
            }
        }
        return;
    }
    let input = args.last().unwrap();
    match get_date(&input) {
        Ok(resp) => {
            println!("
Requested: {}    Len({})
Local:     {}
UTC:       {}
            ", input, input.len(), resp.local_date, resp.gmt_date);
        },
        Err(_) => {
            println!("Cannot process {} as valid date", input);
        }
    }
}
