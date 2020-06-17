#![allow(non_snake_case)]
use std::process::Command;
use std::env;



struct ConnectionStatus {
    mode: String,
    status: String,
    SSID: String,
}


struct ConnectionData {
    ip: String,
    mac: String,
}


fn makeString(stdout: &Vec<u8>) -> String {
    // executing a command returns an array of u8 instead of chars, this simply turns them into
    // chars and then returns all of them as a string
    let string =  String::from_utf8_lossy(stdout).to_string();
    return string
}


fn separateTitle(string: &str) -> Vec<&str> {
    let mut elements: Vec<&str> = string.split(":").collect();
    elements.remove(0);
    return elements;
}


fn getConnectionData(device: &String) -> ConnectionData {
    let command = Command::new("nmcli").arg("-t").arg("device").arg("show").arg(device).output().expect("Failed to execute command");
    let command = makeString(&command.stdout);

    let lines: Vec<&str> = command.split("\n").collect();

    let mac = separateTitle(lines[findIndex(&lines, &String::from("HWADDR")).unwrap()]).join(":");

    let ipAddress: String = separateTitle(lines[findIndex(&lines, &String::from("IP4.ADDRESS")).unwrap()]).join(".");
    let mut ipAddress: Vec<&str> = ipAddress.split("/").collect();
    ipAddress.pop();

    let connectionData = ConnectionData {
        ip: ipAddress[0].to_string(),
        mac: mac,
    };

    return connectionData;
}


// this returns an index if the specified interface is found
fn findIndex(list: &Vec<&str>, substring: &String) -> Result<usize, String> {
    for i in 0..list.len() {
        if list[i].contains(substring) {
            return Ok(i);
        }
    }
    return Err(String::from("Could not find interface"));
}


fn getStatus(device: &String) -> ConnectionStatus {
    let command = Command::new("nmcli").arg("-t").arg("device").arg("status").output().expect("Usage: nmPolyWidget <interface>\nExample: nmPolyWidget wlan1");
    let command = makeString(&command.stdout);

    let lines: Vec<&str> = command.split("\n").collect();
    let statusAll = lines[findIndex(&lines, device).unwrap()];

    let interfaceStatus: Vec<&str> = statusAll.split(":").collect();

    let mode = interfaceStatus[1].to_string();
    let status = interfaceStatus[2].to_string();
    let SSID = interfaceStatus[3].to_string();

    let status = ConnectionStatus {
        mode: mode,
        status: status,
        SSID: SSID
    };

    return status;
}




fn main() {
    let args: Vec<String> = env::args().collect();
    let device = &args[1];

    let connectionStatus = getStatus(&device);
    let connectionData = getConnectionData(&device);

    println!("▂▄▆█ {} ↑↓ {}", connectionStatus.SSID, connectionData.ip)
}
