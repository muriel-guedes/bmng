pub fn send_static_file(req: &mut gondor_io::Request) -> Result<(),std::io::Error> {
    let path = req.path();
    match path {
        b"asset-manifest" | b"asset-manifest.json" => req.send(200, "application/json", include_bytes!("../../react-app/build/asset-manifest.json")),
		b"favicon" | b"favicon.ico" => req.send(200, "image/vnd.microsoft.icon", include_bytes!("../../react-app/build/favicon.ico")),
		b"" | b"index" | b"index.html" => req.send(200, "text/html", include_bytes!("../../react-app/build/index.html")),
		b"logo192" | b"logo192.png" => req.send(200, "image/png", include_bytes!("../../react-app/build/logo192.png")),
		b"logo512" | b"logo512.png" => req.send(200, "image/png", include_bytes!("../../react-app/build/logo512.png")),
		b"manifest" | b"manifest.json" => req.send(200, "application/json", include_bytes!("../../react-app/build/manifest.json")),
		b"robots" | b"robots.txt" => req.send(200, "text/plain", include_bytes!("../../react-app/build/robots.txt")),
		b"main.ba17183d" | b"static/css/main.ba17183d.css" => req.send(200, "text/css", include_bytes!("../../react-app/build/static/css/main.ba17183d.css")),
		b"main.db9e5a25" | b"static/js/main.db9e5a25.js" => req.send(200, "text/javascript", include_bytes!("../../react-app/build/static/js/main.db9e5a25.js")),
		b"main.db9e5a25.js.LICENSE" | b"static/js/main.db9e5a25.js.LICENSE.txt" => req.send(200, "text/plain", include_bytes!("../../react-app/build/static/js/main.db9e5a25.js.LICENSE.txt")),
		b"logo.6ce24c58023cc2f8fd88fe9d219db6c6" | b"static/media/logo.6ce24c58023cc2f8fd88fe9d219db6c6.svg" => req.send(200, "image/svg+xml", include_bytes!("../../react-app/build/static/media/logo.6ce24c58023cc2f8fd88fe9d219db6c6.svg")),
		_ => Ok(())
    }
}