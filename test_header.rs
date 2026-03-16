fn main() {
    let val = actix_web::http::header::HeaderValue::from_static("nginx/1.24.0 (Ubuntu)");
    println!("Value: {:?}", val);
}
