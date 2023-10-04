extern crate libc;

use libc::{c_ushort, c_int, c_uchar, c_uint};
use std::time::Duration;
use std::thread;

// Representation of PCAN_PCIBUS1 to PCAN_PCIBUS16
// Refer TPCANHandle definition in PCAN-Basic documentation for more information
const PCAN_HANDLE: [u16; 16] = [65, 66, 67, 68, 69, 70, 71, 72, 1033, 1034, 1035, 1036, 1037, 1038, 1039, 1040];
// Only for CAN. N/A for CANFD
const PCAN_BAUD_500K: u16 = 0x001C; 

#[repr(C)]
pub struct TPCANMsg {
    pub ID: c_uint,
    pub MSGTYPE: c_uchar,
    pub LEN: c_uchar,
    pub DATA: [c_uchar; 8]
}

#[repr(C)]
pub struct TPCANMsgFD {
    pub ID: c_uint,
    pub MSGTYPE: c_uchar,
    pub LEN: c_uchar,
    pub DATA: [c_uchar; 64]
}

#[link(name = "pcanbasic", kind = "dylib")]
extern {
    fn CAN_Initialize(Channel: c_ushort, Btr0Btr1: c_ushort);
    fn CAN_GetStatus(Channel: c_ushort) -> c_int;
    fn CAN_Write(Channel: c_ushort, MessageBuffer: &TPCANMsg);
}

fn send_message_1() {
    let data = [1,2,3,4,5,6,7,8];
    let can_message = TPCANMsg { ID: 123, MSGTYPE: 0, LEN: 8, DATA: data};
    for _ in 1..100 {
        unsafe {
            CAN_Write(PCAN_HANDLE[0], &can_message);
        }
        thread::sleep(Duration::from_millis(10));
    }
}

fn send_message_2() {
    let data = [1,2,3,4,5,6,7,8];
    let can_message = TPCANMsg { ID: 124, MSGTYPE: 0, LEN: 8, DATA: data};
    for _ in 1..100 {
        unsafe {
            CAN_Write(PCAN_HANDLE[1], &can_message);
        }
        thread::sleep(Duration::from_millis(10));
    }
}

fn main() {
    unsafe {
        CAN_Initialize(PCAN_HANDLE[0], PCAN_BAUD_500K);
        CAN_Initialize(PCAN_HANDLE[1], PCAN_BAUD_500K);
        let status = CAN_GetStatus(PCAN_HANDLE[0]);
        println!("{status}");
    }
    println!("Waiting 10 seconds for pcanview to start");
    thread::sleep(Duration::from_millis(10000));
    println!("Start sending messages");
    let thd1 = thread::spawn(|| {
        send_message_1();
    });
    let thd2 = thread::spawn(|| {
        send_message_2();
    });
    thd1.join().unwrap();
    thd2.join().unwrap();
}