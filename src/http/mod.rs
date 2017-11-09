use iron::prelude::*;
use iron::status;
use serde_json;

use super::proc_fs_utils::process::scan_process_entries;

mod routing;

fn list_processes(request: &mut Request) -> IronResult<Response> {

    info!("processing request: {}", request.url);

    match scan_process_entries() {
        Ok(processes) => {
            let processes_string =
                serde_json::to_string_pretty(&processes).unwrap();
            let response = Response::with((status::Ok, processes_string));
            Ok(response)
        }
        Err(_) => {
            let response =
                Response::with((status::InternalServerError, "failed to get data"));
            Ok(response)
        }
    }
}

pub fn start_server() {

    let mut router = self::routing::Router::new();
    router.add_route("processes".to_owned(), list_processes);

    info!("booting up server");
    Iron::new(router).http("localhost:3000").unwrap();
}