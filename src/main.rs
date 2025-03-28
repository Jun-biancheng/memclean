// Windows内存优化工具 - 子非鱼 © 2025
use clipboard_win::{empty, Clipboard};
use windows::{
    core::w,
    Win32::{
        Foundation::GetLastError,
        System::{
            Console::{
                GetStdHandle, SetConsoleOutputCP, SetConsoleTextAttribute,
                STD_OUTPUT_HANDLE, FOREGROUND_GREEN, FOREGROUND_INTENSITY,
            },
            Memory::SetProcessWorkingSetSizeEx,
            ProcessStatus::EnumProcesses,
            SystemInformation::{GlobalMemoryStatusEx, MEMORYSTATUSEX},
            Threading::{
                GetCurrentProcessId, OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_SET_QUOTA,
            },
        },
    },
};
/// 初始化控制台设置
fn init_console() -> Result<(), String> {
    unsafe {
        // 设置控制台编码为UTF-8
        SetConsoleOutputCP(65001)
            .as_bool()
            .then_some(())
            .ok_or_else(|| format!("设置控制台编码失败: {}", GetLastError().0))?;
        
        // 设置控制台标题
        windows::Win32::System::Console::SetConsoleTitleW(w!("Windows内存优化工具"))
            .as_bool()
            .then_some(())
            .ok_or_else(|| format!("设置控制台标题失败: {}", GetLastError().0))?;
            
        // 设置控制台颜色
        let handle = GetStdHandle(STD_OUTPUT_HANDLE)
            .map_err(|e| format!("获取控制台句柄失败: {:?}", e))?;
            
        SetConsoleTextAttribute(handle, (FOREGROUND_GREEN | FOREGROUND_INTENSITY).into())
            .as_bool()
            .then_some(())
            .ok_or_else(|| format!("设置控制台颜色失败: {}", GetLastError().0))?;
            
        Ok(())
    }
}





/// 优化指定进程的内存使用
fn optimize_process_memory(pid: u32) -> Result<(), String> {
    let access = PROCESS_QUERY_INFORMATION | PROCESS_SET_QUOTA;
    
    unsafe {
        let handle = OpenProcess(access, false, pid)
            .map_err(|e| format!("无法打开进程{}: {:?}", pid, e))?;
            
        SetProcessWorkingSetSizeEx(handle, usize::MAX, usize::MAX, 0)
            .as_bool()
            .then_some(())
            .ok_or_else(|| format!("设置工作集大小失败: {}", GetLastError().0))
    }
}

/// 优化系统内存
fn optimize_memory() -> Result<(), String> {
    const MAX_PIDS: usize = 1024;
    let mut pids = [0u32; MAX_PIDS];
    let mut needed = 0;
    
    unsafe {
        EnumProcesses(pids.as_mut_ptr(), (MAX_PIDS * 4) as u32, &mut needed)
            .as_bool()
            .then_some(())
            .ok_or("无法枚举进程")?;
    }

    let count = (needed / 4) as usize;
    println!("正在优化 {} 个进程的内存...", count);
    
    let current_pid = unsafe { GetCurrentProcessId() };
    
    for &pid in &pids[..count] {
        if pid != 0 && pid != current_pid {
            match optimize_process_memory(pid) {
                Ok(_) => println!("进程 {}: ✓ 优化成功", pid),
                Err(e) => println!("进程 {}: ✗ 优化失败 - {}", pid, e),
            }
        }
    }
    
    Ok(())
}

/// 获取系统内存使用率
fn get_memory_usage() -> Result<f64, String> {
    let mut mem_status = MEMORYSTATUSEX {
        dwLength: std::mem::size_of::<MEMORYSTATUSEX>() as u32,
        ..Default::default()
    };
    
    unsafe {
        GlobalMemoryStatusEx(&mut mem_status)
            .as_bool()
            .then_some(())
            .ok_or_else(|| format!("无法获取内存状态 (错误代码: {})", GetLastError().0))?;
    }
    
    let used = mem_status.ullTotalPhys - mem_status.ullAvailPhys;
    Ok((used as f64 / mem_status.ullTotalPhys as f64) * 100.0)
}

fn main() {
    if let Err(e) = init_console() {
        eprintln!("初始化控制台失败: {}", e);
        return;
    }
    
    println!("=== Windows内存优化工具 ===");
    println!("子非鱼 © 2025");
    
    match get_memory_usage() {
        Ok(usage) => {
            println!("当前内存占用率: {:.1}%", usage);
            if usage >= 50.0 {
                println!("[1/2] 准备优化内存...");
                println!("正在优化内存...");
                match optimize_memory() {
                    Ok(_) => println!(" ✓ 内存优化完成"),git init


                    Err(e) => println!(" ✗ 内存优化失败: {}", e),
                };
            } else {
                println!("内存占用低于50%，跳过优化");
            }
        }
        Err(e) => {
            println!(" ✗ 内存检测失败: {}", e);
        }
    }

    println!("[2/2] 正在清理剪贴板...");
    
    match Clipboard::new() {
        Ok(_clipboard) => match empty() {
            Ok(_) => println!("剪贴板内容已成功清除"),
            Err(e) => println!("剪贴板清除失败: {}", e),
        },
        Err(e) => println!("无法访问剪贴板: {}", e),
    };


    
    // 确保窗口保持足够时间
    println!("\n程序将在3秒后退出...");
    let start = std::time::Instant::now();
    while start.elapsed() < std::time::Duration::from_secs(3) {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
