use std::io::{self, Write};
use std::process::Command;
use std::path::PathBuf;

fn main() {
    // 提示用户输入要转换的类型（图片/视频）
    print!("请输入要转换的类型（图片/视频）：");
    io::stdout().flush().unwrap();
    let mut input_type = String::new();
    io::stdin().read_line(&mut input_type).unwrap();
    let input_type = input_type.trim();

    // 判断输入类型
    let input_type = match input_type {
        "图片" => "image",
        "视频" => "video",
        _ => panic!("无效的输入类型！"),
    };

    // 提示用户输入要转换的文件名
    print!("请输入要转换的文件名：");
    io::stdout().flush().unwrap();
    let mut input_filename = String::new();
    io::stdin().read_line(&mut input_filename).unwrap();

    // 提示用户输入转换后的文件名
    print!("请输入转换后的文件名：");
    io::stdout().flush().unwrap();
    let mut output_filename = String::new();
    io::stdin().read_line(&mut output_filename).unwrap();

    // 提示用户输入转换后文件保存位置
    print!("请输入转换后文件的保存位置（留空则保存至本程序运行目录）：");
    io::stdout().flush().unwrap();
    let mut output_path = String::new();
    io::stdin().read_line(&mut output_path).unwrap();
    let mut output_path = output_path.trim().to_owned();

    // 使用 ffmpeg 进行格式转换
    let mut args = vec![];
    match input_type {
        "image" => {
            args.push("-i");
            args.push(input_filename.trim());
            args.push(output_filename.trim());
        }
        "video" => {
            args.push("-i");
            args.push(input_filename.trim());
            args.push("-c:v");
            args.push("libx264");
            args.push("-c:a");
            args.push("aac");
            args.push(output_filename.trim());
        }
        _ => panic!("无效的输入类型！"),
    };

    if output_path.is_empty() {
        output_path = std::env::current_dir().unwrap().to_str().unwrap().to_owned();
    }
    args.push("-y"); // 覆盖输出文件
    let mut output_file_path = PathBuf::from(&output_path);
    output_file_path.push(output_filename.trim());
    args.push(output_file_path.to_str().unwrap());

    let output = Command::new("ffmpeg")
        .args(&args)
        .output()
        .expect("无法执行 ffmpeg 命令");

    // 检查转换是否成功
    if output.status.success() {
        println!("转换成功！");
    } else {
        let error_message = String::from_utf8_lossy(&output.stderr);
        println!("转换失败：{}", error_message);
    }
}
