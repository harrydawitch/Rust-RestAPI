// Khai báo các thư viện cần thiết
use serde::Serialize;
use serde_json::to_string_pretty;

// 1. Dùng #[derive(Serialize)] trên struct.
// Điều này tự động tạo mã để biến cấu trúc này thành dữ liệu JSON.
#[derive(Debug, Serialize)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub price: u64,
    pub in_stock: bool,
}

fn main() {
    // 2. Tạo một thể hiện (instance) của struct
    let laptop = Product {
        id: 101,
        name: "Laptop Ultrabook".to_string(),
        price: 25_000_000,
        in_stock: true,
    };

    // 3. Sử dụng hàm của serde_json để Serialize (chuyển đổi)
    // to_string_pretty tạo JSON có định dạng dễ đọc hơn.
    match to_string_pretty(&laptop) {
        Ok(json_string) => {
            println!("✅ Serialization thành công!");
            println!("\nDữ liệu Struct ban đầu:");
            println!("{:#?}", laptop); // Dùng Debug trait

            println!("\nChuỗi JSON kết quả:");
            println!("{}", json_string);
        }
        Err(e) => {
            eprintln!("❌ Lỗi Serialization: {}", e);
        }
    }
}
