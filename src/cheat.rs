use std::sync::{Arc, RwLock};

use winapi::shared::ntdef::NULL;
use winapi::um::winnt::PROCESS_ALL_ACCESS;

use crate::cheat_helper::data::DataSaver;
use crate::cheat_helper::helper::Coord;
use crate::mem::ProcMem;

pub fn tp(coords: Coord) {
    println!("Coords : {}", coords);
}


pub trait Cheat {
    fn start() {}
    fn stop() {}
}

pub struct CheatsManager {
    pub(crate) running: bool,
    pub(crate) data_saver: Arc<RwLock<DataSaver>>,
}

impl CheatsManager {
    pub unsafe fn run(&self) {
        let mut mem = ProcMem {
            h_process: NULL,
            dw_pid: 2940,
            dw_protection: PROCESS_ALL_ACCESS,
            dw_cave_addr: 0,
            b_pOn: false,
            b_iOn: false,
            b_prot: false,
        };
        mem.process("ac_client.exe".to_string());
        let mut result = 0;
        mem.read_mem::<i32>(0x008428B8, &mut result);
        println!("{}", result);
        // ammo::write_ammo(&mut mem, 666, &self.data_saver);
    }

    pub fn stop() {}
}
