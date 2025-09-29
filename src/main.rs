use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};

#[get("/")]
async fn info() -> impl Responder {
    let html = format!(
        r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
      <meta charset="UTF-8" />
      <meta name="viewport" content="width=device-width, initial-scale=1.0" />
      <title>Resumes Counter</title>
      <style>
        body {{
          margin: 0;
          background-color: purple;
          color: white;
          display: flex;
          flex-direction: column;
          justify-content: center;
          align-items: center;
          height: 100vh;
          font-family: Arial, sans-serif;
        }}

        .label {{
          font-size: 2rem;
          margin-bottom: 0.5rem;
        }}

        .number {{
          font-size: 4rem;
          font-weight: bold;
        }}
      </style>
    </head>
    <body>
      <div class="label">resumes created:</div>
      <div class="number">{}</div>
    </body>
    </html>
    "#,
        1
    );

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(info))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
