use std::net::{ToSocketAddrs, SocketAddr};
use std::io::prelude::*;
use std::net::TcpStream;
use std::str;
use std::io::BufReader;

mod epson_constants;
pub mod base_commands;

pub struct ControllerOptions {
    pub address: SocketAddr
}

pub struct Controller {
    opts: ControllerOptions,
    socket: TcpStream
}

pub struct Robot {
    pub robot: u8,
    controller: Controller
}

use std::fmt::{Display, Formatter, Error};
pub struct Params(pub Vec<String>);

pub struct RemoteEthernetCommand {
    pub command: String,
    pub parameters: Option<Params>
}

impl Display for Params {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut result = String::new();

        for num in &self.0 {
            result.push_str(",");
            result.push_str(&num.to_string());
        }

        write!(f, "{}", result)
    }
}

/**
 * See: EPSON RC+ 7.0 (Ver.7.3) User's Guide Rev.4 - 12.2.5 Remote Ethernet Command
 * for the connection docs.
 * Format: $remote_command {, parameter...} terminator
 */
impl Controller {
    pub fn connect(opts: ControllerOptions) -> Result<Self, std::io::Error> {
        let mut stream = TcpStream::connect(opts.address)?;
        stream.set_read_timeout(None);

        Ok(Self {
            socket: stream,
            opts
        })
    }

    // pub fn create_robot(&self, robot: u8) -> Robot {
    //     Robot::new(ref self, robot)
    // }

    fn send_cmds(&mut self, cmds: Vec<&str>) {
        for ref cmd in cmds {
            self.send_cmd(cmd.to_string()).expect("Failed");
            // match self.send_cmd(cmd.to_string()) {
            //     Ok(res) => {
            //         println!("{:?} => {:?}", cmd, res);
            //     },
            //     Err(err) => {
            //         println!("{:?} => ERROR({:?})", cmd, err);
            //     }
            // }
        }
    }

    pub fn build_command(cmd: RemoteEthernetCommand) -> String {
        match cmd.parameters {
            Some(params) => format!("${}{},terminator", cmd.command, params),
            None => format!("${},,terminator", cmd.command)
        }
    }

    pub fn login(&mut self) -> Result<String, std::io::Error> {
        Ok(self.send_cmd("$Login,,terminator".to_string())?)
    }

    pub fn logout(&mut self) -> Result<String, std::io::Error> {
        Ok(self.send_cmd("$Logout,,terminator".to_string())?)
    }

    pub fn execute(&mut self, cmd: RemoteEthernetCommand) -> Result<String, std::io::Error> {
        // self.login();
        self.send_cmd(Self::build_command(cmd))
    }
    
    fn send_cmd(&mut self, cmd: String) -> Result<String, std::io::Error> {
        print!("DEBUG: {:x?} => ", cmd);
        self.socket.write(cmd.as_bytes())?;
        
        let mut result = String::with_capacity(epson_constants::NET_BUFFER_MAX_SIZE);
        let mut reader = BufReader::new(&self.socket);
        let _size = reader.read_line(&mut result);
        // println!("DEBUG: size={:?} buffer={:?}", _size, buffer);
        result = result.trim().to_string();
        println!("{:?}", result);

        Ok(result.to_string())
    }

}

impl Robot {
    pub fn new(controller: Controller, robot: u8) -> Self {
        Self {
            controller, robot
        }
    }

    pub fn home(&mut self) {
        self.controller.execute(RemoteEthernetCommand {
            command: "Home".to_string(),
            parameters: Some(Params(vec![self.robot.to_string()]))
        }).expect("Failed to home");
    }

    pub fn enable(&mut self) {
        self.controller.execute(RemoteEthernetCommand {
            command: "SetMotorsOn".to_string(),
            parameters: Some(Params(vec![self.robot.to_string()]))
        }).expect("Failed to home");

        std::thread::sleep(std::time::Duration::from_secs(5));
    }

    pub fn disable(&mut self) {
        self.controller.execute(RemoteEthernetCommand {
            command: "SetMotorsOff".to_string(),
            parameters: Some(Params(vec![self.robot.to_string()]))
        }).expect("Failed to home");
    }

    pub fn status(&mut self) {
        self.controller.execute(RemoteEthernetCommand {
            command: "GetStatus".to_string(),
            parameters: Some(Params(vec![self.robot.to_string()]))
        }).expect("Failed to get status");
    }

    pub fn reset(&mut self) {
        self.controller.execute(RemoteEthernetCommand {
            command: "Reset".to_string(),
            parameters: None
        }).expect("Failed to get reset");
    }

    pub fn stop(&mut self) {
        self.controller.execute(RemoteEthernetCommand {
            command: "Stop".to_string(),
            parameters: None
        }).expect("Failed to get reset");
    }

    pub fn logout(&mut self) {
        self.controller.execute(RemoteEthernetCommand {
            command: "Logout".to_string(),
            parameters: None
        }).expect("Failed to get reset");
    }

    pub fn pause(&mut self) {
        self.controller.execute(RemoteEthernetCommand {
            command: "Pause".to_string(),
            parameters: None
        }).expect("Failed to get reset");
    }
}