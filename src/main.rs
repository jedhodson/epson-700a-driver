mod epson_driver;
use epson_driver::{Controller, ControllerOptions, Robot };

use std::net::ToSocketAddrs;

/**
    Starting R700-A...
    DEBUG: "$Login,,terminator" => "#Login,0"
    DEBUG: "$GetVariable,VERSION$,String" => "#GetVariable,PRODUCTION-AU.ACMK4.M3.2020.1.0.6"
    DEBUG: "$GetStatus,,terminator" => "#GetStatus,00100000010,0000"
    Starting C4...
    DEBUG: "$Reset,,terminator" => "!Reset,20"
    DEBUG: "$SetMotorsOn,1,terminator" => "!SetMotorsOn,20"
    DEBUG: "$GetStatus,1,terminator" => "#GetStatus,00100000010,0000"
    Shutting down...
    DEBUG: "$SetMotorsOff,1,terminator" => "!SetMotorsOff,20"
    DEBUG: "$GetStatus,1,terminator" => "#GetStatus,00100000010,0000"
    DEBUG: "$Pause,,terminator" => "#Pause,0"
    DEBUG: "$Stop,,terminator" => "#Stop,0"
    DEBUG: "$Logout,,terminator" => "#Logout,0
 */
fn main() {
    let opts = ControllerOptions {
        // TODO: There must be better way
        address: "192.168.0.122:5000".to_socket_addrs().unwrap().next().unwrap() 
        // address: "127.0.0.1:5000".to_socket_addrs().unwrap().next().unwrap() 
    };

    let mut controller = match Controller::connect(opts) {
        Ok(robot) => robot,
        Err(err) => panic!("ERROR: {:?}", err)
    };

    println!("Starting R700-A...");
    controller.login().expect("Failed to log in to controller");
    controller.start();
    std::thread::sleep(std::time::Duration::from_secs(2));

    
    let mut c4 = Robot::new(controller, 1);
    // let mut c4 = controller.create_robot(2);
    println!("Starting C4...");
    c4.reset();
    c4.enable();
    c4.status();

    // c4.reset();
    // c4.home();
    // c4.status();


    std::thread::sleep(std::time::Duration::from_secs(10));
    println!("Shutting down...");
    c4.disable();
    c4.status();
    c4.pause();
    c4.stop();
    c4.logout();
    
    // controller.logout().expect("Error logging out");

    // let cmd = RemoteEthernetCommand {
    //     command: "Home".to_string(),
    //     parameters: Some(Params(vec!["1".to_string()]))
    // };

    // println!("TEST: {}", Controller::build_command(cmd));

    // let mut robot = match Controller::connect(opts) {
    //     Ok(robot) => robot,
    //     Err(err) => panic!("ERROR: {:?}", err)
    // };

    // println!("Connected to robot.");
    // robot.start();
    // robot.status();
    
    // println!("Give life to rombot");
    // robot.motors(true);

    // std::thread::sleep(std::time::Duration::from_millis(2000));
    // println!("Stopping the robot");
    // robot.motors(false);
    // robot.stop();
}
