use std::sync::atomic::{AtomicUsize, Ordering};

pub struct Counter {
    get: AtomicUsize,
    post: AtomicUsize,
    put: AtomicUsize,
    delete: AtomicUsize,
}

impl Default for Counter {
    fn default() -> Self {
        info!("GET/POST/PUT/DELETE default counter has been created.");
        Self {
            get: AtomicUsize::default(),
            post: AtomicUsize::default(),
            put: AtomicUsize::default(),
            delete: AtomicUsize::default()
        }
    }
}

impl Counter {
    pub fn increment_get(&mut self) { self.get.fetch_add(1, Ordering::SeqCst); }
    
    pub fn increment_post(&mut self) { self.post.fetch_add(1, Ordering::SeqCst); }
    
    pub fn increment_put(&mut self) { self.put.fetch_add(1, Ordering::SeqCst); }
    
    pub fn increment_delete(&mut self) { self.delete.fetch_add(1, Ordering::SeqCst); }

    pub fn get(&self) -> usize { self.get.load(Ordering::SeqCst) }

    pub fn post(&self) -> usize { self.post.load(Ordering::SeqCst) }
    
    pub fn put(&self) -> usize { self.put.load(Ordering::SeqCst) }

    pub fn delete(&self) -> usize { self.delete.load(Ordering::SeqCst) }
}