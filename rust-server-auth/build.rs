use std::{io::Write, path::{Path, PathBuf}};

pub const PUBLIC_DIR: &'static str = env!("PUBLIC_DIR");
pub const COMPILE_TO: &'static str = env!("COMPILE_TO");
pub const ROOT_FILE: &'static str = env!("ROOT_FILE");

fn main() {
    let public_dir = Path::new(PUBLIC_DIR);
    std::fs::create_dir_all(public_dir).unwrap();
    
    let mut paths = Vec::new();
    pathloop(public_dir, &mut paths);
    let mut file = std::fs::OpenOptions::new()
        .create(true).truncate(true).write(true)
        .open(Path::new(COMPILE_TO).join("static_files.rs")).unwrap();
    
    let pathdiff = pathdiff::diff_paths(PUBLIC_DIR, COMPILE_TO).unwrap();
    let prefix = pathdiff.to_string_lossy().replace('\\',"/");
    let mut routes = String::new();
    for path in &paths {
        let path = path.strip_prefix(PUBLIC_DIR).unwrap();
        let with_ext = path.with_extension("");
        let name = with_ext.file_name().unwrap().to_string_lossy();
        let filetype = file_type(path.extension().unwrap().to_str().unwrap());
        let path = path.to_string_lossy().replace('\\', "/");
        if name == ROOT_FILE {
            routes.push_str(&format!(
                "b\"\" | b\"{name}\" | b\"{path}\" => req.send(200, \"{filetype}\", include_bytes!(\"{prefix}/{path}\")),\n\t\t"))
        }else {
            routes.push_str(&format!(
                "b\"{name}\" | b\"{path}\" => req.send(200, \"{filetype}\", include_bytes!(\"{prefix}/{path}\")),\n\t\t"))
        }
    }

write!(file, "pub fn send_static_file(req: &mut gondor_io::Request) -> Result<(),std::io::Error> {{
    let path = req.path();
    match path {{
        {routes}_ => Ok(())
    }}
}}").unwrap();
}
fn pathloop(path: &std::path::Path, paths: &mut Vec<PathBuf>) {
    for path in std::fs::read_dir(path).unwrap() {
        let path = path.unwrap().path();
        if path.is_dir() {
            pathloop(path.as_path(), paths)
        } else if path.is_file() {
            paths.push(path.clone());
            println!("cargo:debug=Static file: {}", path.display())
        }
    }
}

/// See more at: https://www.iana.org/assignments/media-types/media-types.xhtml
pub fn file_type(v: &str) -> &str {
    match v {
        "aac" => "audio/aac",
        "abw" => "application/x-abiword",
        "arc" => "application/x-freearc",
        "avi" => "video/x-msvideo",
        "azw" => "application/vnd.amazon.ebook",
        "bin" => "application/octet-stream",
        "bmp" => "image/bmp",
        "bz" => "application/x-bzip",
        "bz2" => "application/x-bzip2",
        "csh" => "application/x-csh",
        "css" => "text/css",
        "csv" => "text/csv",
        "doc" => "application/msword",
        "docx" => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        "eot" => "application/vnd.ms-fontobject",
        "epub" => "application/epub+zip",
        "gz" => "application/gzip",
        "gif" => "image/gif",
        "htm" => "text/html",
        "html" => "text/html",
        "ico" => "image/vnd.microsoft.icon",
        "ics" => "text/calendar",
        "jar" => "application/java-archive",
        "jpeg" => "image/jpeg",
        "jpg" => "image/jpeg",
        "js" => "text/javascript",
        "json" => "application/json",
        "jsonld" => "application/ld+json",
        "mid" => "audio/midi audio/x-midi",
        "midi" => "audio/midi audio/x-midi",
        "mjs" => "text/javascript",
        "mp3" => "audio/mpeg",
        "mpeg" => "video/mpeg",
        "mpkg" => "application/vnd.apple.installer+xml",
        "oga" => "audio/ogg",
        "ogv" => "video/ogg",
        "ogx" => "application/ogg",
        "opus" => "audio/opus",
        "otf" => "font/otf",
        "png" => "image/png",
        "pdf" => "application/pdf",
        "php" => "application/x-httpd-php",
        "ppt" => "application/vnd.ms-powerpoint",
        "rar" => "application/vnd.rar",
        "rtf" => "application/rtf",
        "sh" => "application/x-sh",
        "svg" => "image/svg+xml",
        "tar" => "application/x-tar",
        "tiftiff" => "image/tiff",
        "tiff" => "image/tiff",
        "ts" => "video/mp2t",
        "ttf" => "font/ttf",
        "txt" => "text/plain",
        "wav" => "audio/wav",
        "weba" => "audio/webm",
        "webm" => "video/webm",
        "webp" => "image/webp",
        "woff" => "font/woff",
        "woff2" => "font/woff2",
        "xhtml" => "application/xhtml+xml",
        "xls" => "application/vnd.ms-excel",
        "xlsx" => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        "xml" => "application/xml",
        "xul" => "application/vnd.mozilla.xul+xml",
        "zip" => "application/zip",
        "7z" => "application/x-7z-compressed",
        _ => "unknow"
    }
}