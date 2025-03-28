extern crate winres;

fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("src/icon.ico")
           .set("ProductName", "Windows内存优化工具")
           .set("FileDescription", "Windows内存优化工具 - 子非鱼")
           .set("LegalCopyright", "Copyright (c) 2025 子非鱼")
           .set("OriginalFilename", "memclean.exe")
           .set("InternalName", "memclean")
           .set("CompanyName", "子非鱼工作室");
        res.compile().unwrap();
    }
}
