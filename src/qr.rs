use local_ip_address::local_ip;
use qrcode::QrCode;
use qrcode::render::unicode::Dense1x2;
use crate::server;
use crate::server::Action;

fn get_endpoint(action: Action) -> &'static str {
    match action {
        Action::Download {file_path: _} => {
            "download"
        }
    }
}

pub fn show_qr(action: Action, port: u16) {
    let ip: Option<String> = match local_ip() {
        Ok(ip) => Some(ip.to_string()),
        Err(local_ip_address::Error::LocalIpAddressNotFound) => None,
        Err(e) => panic!("{}",e)
    };
    if let Some(ip) = ip {
        let endp = get_endpoint(action);
        let url = format!("http://{}:{}/{}",ip,port,endp);
        let qr_code = QrCode::new(&url).expect("Couldn't create QRCode.");
        let qrcode_str = qr_code.render::<Dense1x2>().quiet_zone(false).module_dimensions(1,1).build();
        println!("Scan the qrcode: \n\n{}\n\nOr manually open the URL in browser {}",qrcode_str,url);
    }else {
        eprintln!("Failed to resolve local IP.");
    }
}