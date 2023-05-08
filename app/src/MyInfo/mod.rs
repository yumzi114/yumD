
use std::{path::{Path, PathBuf}, ops::Add};
pub struct MyInfo{
    pub list : Vec<System>,
    
}
pub struct System{
    pub sysname:String,
    pub used:bool,
    pub path:String,
    opendoc:bool,
}
impl System{
    fn new(filepath:&str, sysname:&str)->System{
        System{
            sysname:sysname.to_string(),
            used:Path::new(filepath).is_file(),
            path:filepath.to_string(),
            opendoc:true,
        }
    }
    pub fn menu(&self)->String{
        let name = self.sysname.clone().add(" : ");
        // let name = [temp," : ".to_string()].join("");
        name
    }
}
impl MyInfo{
    pub fn new()->MyInfo{
        MyInfo { 
            list:vec![
                System::new("/etc/systemd/system.conf","system.conf"),
                System::new("/etc/pacman.conf","pacman"),
                System::new("/bin/yay","yay"),
                System::new("/etc/modprobe.d/vfio.conf","vfio")
                ]
        }
    }
    
}