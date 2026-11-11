use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    assert_eq!(env::var("CARGO_CFG_TARGET_OS").unwrap(), "windows");
    assert_eq!(env::var("CARGO_CFG_TARGET_ENV").unwrap(), "gnu");

    let manifest = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    // crates/trainer -> crates -> 仓库根目录。workspace 的 target 固定在根目录。
    let workspace = manifest
        .parent()
        .and_then(|path| path.parent())
        .expect("trainer crate must live under crates/");
    let target_dir = env::var_os("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .map(|path| {
            if path.is_absolute() {
                path
            } else {
                workspace.join(path)
            }
        })
        .unwrap_or_else(|| workspace.join("target"));
    let target = env::var("TARGET").unwrap();
    let profile = env::var("PROFILE").unwrap();
    // 修改器会内嵌 speedhack.dll。Debug 构建时如果还没编 DLL，就退回 release 版本。
    let mut speedhack_dll = target_dir
        .join(&target)
        .join(&profile)
        .join("speedhack.dll");
    if !speedhack_dll.exists() {
        speedhack_dll = target_dir
            .join(&target)
            .join("release")
            .join("speedhack.dll");
    }
    assert!(
        speedhack_dll.exists(),
        "speedhack.dll is missing; build the speedhack crate first (see scripts/build.ps1)"
    );
    fs::copy(&speedhack_dll, out.join("speedhack.dll")).unwrap();
    println!("cargo:rerun-if-changed={}", speedhack_dll.display());
    let icon = manifest
        .join("assets/imouto.ico")
        .to_string_lossy()
        .replace('\\', "/");

    let version = env::var("CARGO_PKG_VERSION").unwrap();
    let major = env::var("CARGO_PKG_VERSION_MAJOR").unwrap();
    let minor = env::var("CARGO_PKG_VERSION_MINOR").unwrap();
    let patch = env::var("CARGO_PKG_VERSION_PATCH").unwrap();

    // 生成资源脚本，把图标和版本信息编进 exe。windres 需要 MinGW 环境。
    let rc_path = out.join("resource.rc");
    let obj_path = out.join("resource.o");
    let rc = format!(
        r#"#pragma code_page(65001)
1 ICON "{icon}"
1 VERSIONINFO
FILEFLAGS 0x0
FILEVERSION {major}, {minor}, {patch}, 0
FILEOS 0x40004
PRODUCTVERSION {major}, {minor}, {patch}, 0
FILESUBTYPE 0x0
FILEFLAGSMASK 0x3f
FILETYPE 0x1
BEGIN
BLOCK "StringFileInfo"
BEGIN
BLOCK "000004b0"
BEGIN
VALUE "FileVersion", "{version}"
VALUE "LegalCopyright", "Copyright © 2025-2026 by Yuki"
VALUE "ProductName", "妹相随修改器"
VALUE "ProductVersion", "{version}"
VALUE "FileDescription", "妹相随～黑白世界的缤纷冒险～修改器"
END
END
BLOCK "VarFileInfo"
BEGIN
VALUE "Translation", 0x0, 0x04b0
END
END
"#
    );
    fs::write(&rc_path, rc).unwrap();

    println!("cargo:rerun-if-changed=assets/imouto.ico");
    println!("cargo:rerun-if-env-changed=WINDRES");
    println!("cargo:rerun-if-env-changed=WINDRES_PREPROCESSOR");

    let windres = env::var("WINDRES").expect("WINDRES must be set");
    let preprocessor = env::var("WINDRES_PREPROCESSOR").expect("WINDRES_PREPROCESSOR must be set");
    let status = Command::new(windres)
        .arg("--preprocessor")
        .arg(preprocessor)
        .arg(&rc_path)
        .arg(&obj_path)
        .status()
        .expect("failed to run windres");
    assert!(status.success(), "windres failed");

    println!(
        "cargo:rustc-link-arg={}",
        obj_path.to_string_lossy().replace('\\', "/")
    );
}
