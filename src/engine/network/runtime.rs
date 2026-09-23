use std::sync::OnceLock;
use tokio::runtime::Runtime;

pub fn get_network_runtime() -> &'static Runtime {
    static RUNTIME: OnceLock<Runtime> = OnceLock::new();
    RUNTIME.get_or_init(|| {
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(2)
            .thread_name("tokio-net")
            .enable_all()
            .build()
            .expect("Failed to initialize Tokio network runtime")
    })
}
