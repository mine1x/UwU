pub mod app;
pub mod core;
pub mod engine;
pub mod input;
pub mod network;
pub mod render;
pub mod world;


fn main() {
    env_logger::init();
    println!("============================================================");
    println!("  PROJECT ZOMBOID STYLE VOXEL ENGINE (Vulkan + Modular)");
    println!("============================================================");
    println!("- Controls:");
    println!("  * W / A / S / D    : Di chuyển tự do theo góc nhìn Isometric");
    println!("  * SPACE            : Nhảy / Bơi");
    println!("  * E                : Bật / Tắt túi đồ (Inventory)");
    println!("  * Chuột trong HUD  : Click tương tác thanh Hotbar & Túi đồ");
    println!("  * Con trỏ chuột    : Khung viền Overlay vàng đậm, không flicker");
    println!("  * Chuột Trái      : Đào / Phá khối ngoài HUD");
    println!("  * Chuột Phải     : Đặt khối / Đặt nước từ Hotbar");
    println!("  * 1 - 9 / Z, X     : Chọn ô Hotbar");
    println!("  * Giữ chuột giữa   : Xoay camera Isometric 360 độ");
    println!("  * Cuộn chuột       : Zoom In / Zoom Out camera");
    println!("  * F3               : Bật / Tắt Pie Chart Profiler (Minecraft Shift+F3)");
    println!("  * G (F3+G)         : Bật / Tắt Chunk Wireframe (Minecraft Chunk Borders)");
    println!("  * 0 / Shift + 1-9  : Điều hướng Pie Chart (0: Lên cấp / 1-9: Vào nhánh)");
    println!("  * ESC              : Thoát game");
    println!("============================================================");

    app::run_bevy_engine();
}