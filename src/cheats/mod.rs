use std::sync::atomic::Ordering;
use std::thread::sleep;
use std::time::Duration;
use crate::cheat::Cheat;
use crate::cheat_helper::data::DataSaver;
use crate::mem::ProcMem;

struct InfiniteAmmo {

}

impl Cheat for InfiniteAmmo {

}

impl InfiniteAmmo {
    unsafe fn write_ammo(mem: &mut ProcMem, amount: i32, data_saver: &mut DataSaver) {
        let mut value = 20;

        loop {
            if !data_saver.is_running {
                break;
            }

            if data_saver.infinite_ammo {
                println!("writing");
                mem.write_mem::<i32>(0x00802910, &mut value);
                sleep(Duration::from_millis(300));
            }
        }
    }
}