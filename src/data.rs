use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "PNY GeForce RTX 5070 ARGB OC 12GB GDDR7 Video Card".to_string(),
            price: 999.99,
            description: "Powered by NVIDIA Blackwell architecture and DLSS 4, this GDDR7 video card boosts gaming and AI performance. With 4th gen ray tracing and 6,144 CUDA cores, it delivers immersive, lag-free experience.".to_string(),
            image: "/catnip.jpg".to_string()
        },
        Product {
            id: 2,
            name: "PNY GeForce RTX5060 Ti OC Dual-Fan16GB GDDR7 Video Card".to_string(),
            price: 579.99,
            description: "Powered by the cutting-edge NVIDIA Blackwell architecture, it features GDDR7 memory and supports resolutions up to 7680 x 4320 for breathtaking visual clarity.".to_string(),
            image: "/squid.jpg".to_string()
        },
        Product {
            id: 3,
            name: "NVIDIA GeForce RTX 5070 12GB GDDR7 Video Card".to_string(),
            price: 799.99,
            description: "Designed for creators, this video card features 12GB of GDDR7 memory and 675GB/sec memory bandwidth for fast data transfer and seamless performance.".to_string(),
            image: "/mermaid.jpg".to_string()
        },
        Product {
            id: 4,
            name: "PNY GeForce RTX 5060 OC 8GB GDDR7 Video Card".to_string(),
            price: 399.99,
            description: "Powered by cutting-edge NVIDIA Blackwell architecture and DLSS 4, it delivers ultra-smooth frame rates and stunning visuals. Accelerate your creative workflow via NVIDIA Studio. Plus, NVIDIA Reflex 2 minimizes system latency for ultra-responsive gameplay.".to_string(),
            image: "/ocean.jpg".to_string()
        },
        Product {
            id: 5,
            name: "ASUS Prime Radeon RX 9070 XT OC 16GB GDDR6 Video Card".to_string(),
            price: 879.99,
            description: "Boost your CPU's power with the ASUS Prime Radeon RX 9070 XT OC graphics card. Equipped with advanced Axial-tech fans and a phase-change GPU thermal pad, it offers optimal cooling and performance. The 2.5-slot design ensures compatibility, while the protective backplate and stainless steel bracket enhance durability.".to_string(),
            image: "/pirate.jpg".to_string()
        },
        Product {
            id: 6,
            name: "MSI Gaming RTX 5060 8G Gaming Trio OC White Graphics Card".to_string(),
            price: 514.99,
            description: "Based on the Blackwell architecture, the MSI GeForce RTX 5060 GAMING TRIO OC WHITE Graphics Card brings the power of real-time ray tracing and AI to your PC games. Play your favorite titles at a high resolution with high frame rates. The GPU features 8GB of GDDR7 VRAM and a 128-bit memory interface".to_string(),
            image: "/tug.jpg".to_string()
        },
        Product {
            id: 7,
            name: "XFX Speedster MERC310 AMD Radeon RX 7900XTX Black Gaming Graphics Card".to_string(),
            price: 1269.99,
            description: "The XFX AMD Radeon RX 7000 Series graphics cards, featuring the groundbreaking AMD RDNA 3 architecture, deliver ultra-high frame rates for your favorite games at 4K max settings.".to_string(),
            image: "/bed.jpg".to_string()
        },
        Product {
            id: 8,
            name: "ASUS Dual NVIDIA GeForce RTX 3050 6GB OC Edition Gaming Graphics Card ".to_string(),
            price: 477.99,
            description: "ASUS Dual NVIDIA GeForce RTX 3050 6GB OC Edition Gaming Graphics Card - PCIe 4.0, 6GB GDDR6 Memory, HDMI 2.1, DisplayPort 1.4a, 2-Slot Design, Axial-tech Fan Design, 0dB Technology, Steel Bracket.".to_string(),
            image: "/knot.jpg".to_string()
        },
        Product {
            id: 9,
            name: "GIGABYTE GeForce RTX 5060 WINDFORCE 8G Graphics Card,".to_string(),
            price: 317.99,
            description: "GIGABYTE WINDFORCE GeForce RTX 5060 8GB GDDR7 PCI Express 5.0 ATX Graphics Card".to_string(),
            image: "/crabby.jpg".to_string()
        },
        Product {
            id: 10,
            name: "EVGA GeForce RTX 3090 FTW3 Gaming 24GB GDDR6X 19500MHz PCIe 4.0 Black Graphic Card ".to_string(),
            price: 1500.99,
            description: "The EVGA GeForce® RTXᐪᔿ 3090 is colossally powerful in every way imaginable, giving you a whole new tier of performance at 8K resolution. It's powered by the NVIDIA Ampere architecture, which doubles down on ray tracing and AI performance with enhanced RT Cores, Tensor Cores, and new streaming multiprocessors.".to_string(),
            image: "/lifejacket.jpg".to_string()
        }
    ]
}