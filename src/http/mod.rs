use iron::prelude::*;
use iron::status;
use serde_json;

use super::proc_fs_utils::process::scan_process_entries;
use super::proc_fs_utils::generic_parsers::get_cpu_info;

mod routing;

fn failure_response() -> Response {
    Response::with((status::InternalServerError, "failed to get data"))
}

fn ok_response(s: String) -> Response {
    Response::with((status::Ok, s))
}

fn list_processes(request: &mut Request) -> IronResult<Response> {
    info!("processing request: {}", request.url);

    match scan_process_entries() {
        Ok(processes) => {
            let process_response = serde_json::to_string_pretty(&processes)
                .ok()
                .map(ok_response)
                .unwrap();
            Ok(process_response)
        }
        Err(_) => Ok(failure_response())
    }
}

fn show_cpu_info(request: &mut Request) -> IronResult<Response> {
    info!("processing request: {}", request.url);

    match get_cpu_info() {
        Ok(cpu_info) => {
            let cpu_info_response = serde_json::to_string_pretty(&cpu_info)
                .ok()
                .map(ok_response)
                .unwrap();
            Ok(cpu_info_response)
        }
        Err(_) => Ok(failure_response())
    }
}

pub fn start_server() {
    let mut router = self::routing::Router::new();
    router.add_route("processes".to_owned(), list_processes);
    router.add_route("cpu".to_owned(), show_cpu_info);

    info!("booting up server");
    Iron::new(router).http("localhost:3000").unwrap();
}