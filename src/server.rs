use crate::qr::show_qr;
use crate::recv;
use crate::recv::{upload, upload_html};
use actix_web::{web, App, CustomizeResponder, HttpRequest, HttpResponse, HttpServer, Responder};
use rand::Rng;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub enum Action {
    Download { file_path: PathBuf },
    Upload,
}
pub async fn download(
    req: HttpRequest,
    action: web::Data<Action>,
) -> CustomizeResponder<HttpResponse> {
    let file_path = match &**action {
        Action::Download { file_path } => file_path.clone(),
        _ => {
            panic!("file_path not found.");
        }
    };

    let file_name = file_path.file_name().unwrap().to_str().unwrap();
    println!(
        "Sending file {:?}... to {}",
        file_path,
        req.connection_info().host()
    );
    let file = actix_files::NamedFile::open_async(&file_path)
        .await
        .unwrap();

    file.into_response(&req).customize().insert_header((
        "Content-Disposition",
        format!("attachment; filename=\"{}\"", file_name),
    ))
}

pub async fn start(action: Action) -> std::io::Result<()> {
    let mut rng = rand::thread_rng();
    let port = rng.gen_range(49152..=65535);
    let action_clone = web::Data::new(action.clone());
    let action_clone2 = action.clone();
    let srv = HttpServer::new({
        move || {
            let mut app = App::new().app_data(action_clone.clone());
            match action.clone() {
                Action::Download { file_path: _ } => {
                    app = app.service(web::resource("/download").route(web::get().to(download)))
                }
                Action::Upload {} => {
                    app = app
                        .route("/", web::get().to(upload_html))
                        // Configure route for file upload
                        .route("/upload", web::post().to(upload))
                }
            }
            app
        }
    })
    .bind(("0.0.0.0", port.clone()))
    .unwrap()
    .run();
    show_qr(action_clone2, port);
    println!("[(*) HTTP] TCP Server is listening on 0.0.0.0:{}", port);
    srv.await
}
