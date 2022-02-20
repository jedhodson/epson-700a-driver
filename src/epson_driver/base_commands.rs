// use super;
use crate::epson_driver::{Controller, RemoteEthernetCommand};

impl Controller {
    pub fn start(&mut self) {
        self.send_cmds(vec![
            "$Start,0,terminator",
            "$GetVariable,VERSION$,String",
            "$GetStatus,,terminator"
        ]);

        // self.build_command(RemoteEthernetCommand {
        //     command: "".to_string(),
        //     parameters: vec!["".to_string()]
        // });
    }
    
    pub fn stop(&mut self) {
        self.send_cmds(vec![
            "$Login,,terminator",
            "$Stop,,terminator"
        ]);
    }

    // Test/Teach/Auto/Warning/SError/Safeguard/EStop/Error/Paused/Running/Ready

    pub fn status(&mut self) {
        self.send_cmds(vec![
            "$Login,,terminator",
            "$GetStatus,,terminator"
        ])
    }

}