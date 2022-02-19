use std::time::Duration;
use std::thread::sleep;
use std::process::Command;

struct Cli{
    pomo: u64,
    pomo_break: u64,
    is_break: bool,
}

fn main() {
    let pomo = std::env::args().nth(1).expect("no pattern given");
    let pomo_break = std::env::args().nth(2).expect("no pattern given");
    let mut args = Cli{
        pomo: pomo.parse().expect("Not a number!"),
        pomo_break : pomo_break.parse().expect("Not a number"),
        is_break: false,
    };
    loop{
        // TODO: Read the notify_path from a config file
        if args.is_break {
            pomo_timer(args.pomo_break);
            notify("Get back to work!!!!".to_string(), "/home/drmoscovium/Music/notification/get_back_to_work.ogg".to_string());
            args.is_break = false;
        }
        else{
            pomo_timer(args.pomo);
            notify("Take a break !!!".to_string(), "/home/drmoscovium/Music/notification/take_a_break.ogg".to_string());
            args.is_break = true;
        }
    }

}

fn pomo_timer(time: u64){
    let mut seconds: u64 = time * 60;
    loop{
        sleep(Duration::new(1,0));
        println!("{}:{}", seconds/60, seconds%60);
        seconds = seconds - 1;
        if seconds <= 0 {
            break;
        }
    }
}

fn notify(message: String, notify_path: String){
    Command::new("notify-send").arg("DrPomo").arg(message).status().expect("command Failed");
    Command::new("paplay").arg(notify_path).status().expect("command Failed");
}
