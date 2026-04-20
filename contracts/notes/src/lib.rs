#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// Struktur data yang akan menyimpan pesanan layanan
#[contracttype]
#[derive(Clone, Debug)]
pub struct PetService {
    pub id: u64,
    pub owner_name: String,   
    pub pet_name: String,     
    pub service_type: String, 
    pub address: String,      
    pub status: String,       
}

// Storage key untuk data pesanan layanan
const SERVICE_DATA: Symbol = symbol_short!("SERVICES");

#[contract]
pub struct PetShopContract;

#[contractimpl]
impl PetShopContract {
    // Fungsi untuk melihat semua daftar pesanan
    pub fn get_services(env: Env) -> Vec<PetService> {
        // 1. ambil data layanan dari storage
        return env.storage().instance().get(&SERVICE_DATA).unwrap_or(Vec::new(&env));
    }

    // Fungsi untuk membuat pesanan layanan baru
    pub fn request_service(
        env: Env, 
        owner_name: String, 
        pet_name: String, 
        service_type: String, 
        address: String
    ) -> String {
        // 1. ambil data layanan dari storage
        let mut services: Vec<PetService> = env.storage().instance().get(&SERVICE_DATA).unwrap_or(Vec::new(&env));
        
        // 2. Buat object pesanan baru dengan status default "Menunggu Jemputan"
        let new_service = PetService {
            id: env.prng().gen::<u64>(),
            owner_name: owner_name,
            pet_name: pet_name,
            service_type: service_type,
            address: address,
            status: String::from_str(&env, "Menunggu Jemputan"),
        };
        
        // 3. tambahkan pesanan baru ke antrean lama
        services.push_back(new_service);
        
        // 4. simpan data terbaru ke storage
        env.storage().instance().set(&SERVICE_DATA, &services);
        
        return String::from_str(&env, "Pesanan layanan berhasil ditambahkan");
    }

    // Fungsi tambahan: Untuk mengupdate status pesanan (misal: "Selesai Grooming")
    pub fn update_status(env: Env, id: u64, new_status: String) -> String {
        let mut services: Vec<PetService> = env.storage().instance().get(&SERVICE_DATA).unwrap_or(Vec::new(&env));

        for i in 0..services.len() {
            let mut service = services.get(i).unwrap();
            if service.id == id {
                // Update statusnya
                service.status = new_status;
                services.set(i, service);
                
                env.storage().instance().set(&SERVICE_DATA, &services);
                return String::from_str(&env, "Status pesanan berhasil diperbarui");
            }
        }

        return String::from_str(&env, "Pesanan tidak ditemukan");
    }

    // Fungsi untuk membatalkan/menghapus pesanan berdasarkan id
    pub fn cancel_service(env: Env, id: u64) -> String {
        // 1. ambil data layanan dari storage 
        let mut services: Vec<PetService> = env.storage().instance().get(&SERVICE_DATA).unwrap_or(Vec::new(&env));

        // 2. cari index pesanan yang akan dihapus menggunakan perulangan
        for i in 0..services.len() {
            if services.get(i).unwrap().id == id {
                services.remove(i);

                env.storage().instance().set(&SERVICE_DATA, &services);
                return String::from_str(&env, "Berhasil membatalkan pesanan");
            }
        }

        return String::from_str(&env, "Pesanan tidak ditemukan");
    }
}

mod test;