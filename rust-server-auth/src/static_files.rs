pub fn send_static_file(req: &mut gondor_io::Request) -> Result<(),std::io::Error> {
    let path = req.path();
    match path {
        b"main" | b"css/main.css" => req.send(200, "text/css", include_bytes!("../public/css/main.css")),
		b"" | b"index" | b"index.html" => req.send(200, "text/html", include_bytes!("../public/index.html")),
		b"ok" | b"ok.txt" => req.send(200, "text/plain", include_bytes!("../public/ok.txt")),
		_ => Ok(())
    }
}