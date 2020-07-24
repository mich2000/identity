use std::sync::atomic::{AtomicUsize, Ordering};

use rocket::{Request, Data};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Method;

/**
 * Struct used to count all the method request
*/
#[derive(Default)]
pub struct HitCount {
    get : AtomicUsize,
    post : AtomicUsize,
    put : AtomicUsize,
    delete : AtomicUsize
}

impl Clone for HitCount {
    fn clone(&self) -> Self {
        HitCount {
            get: AtomicUsize::new(self.get.load(Ordering::SeqCst)),
            post: AtomicUsize::new(self.post.load(Ordering::SeqCst)),
            put: AtomicUsize::new(self.put.load(Ordering::SeqCst)),
            delete: AtomicUsize::new(self.delete.load(Ordering::SeqCst)),
        }
    }
}

impl Fairing for HitCount {
    /**
     * Function used to give a description and give the methods that are used for this middleware.
     */
    fn info(&self) -> Info {
        Info {
            name: "Method requests Counter",
            kind: Kind::Request
        }
    }

    /**
     * When a request is coming in based on the request type the counter will go up at specific property of the counter that matches with the request type.
     * 
     * ex.
     * Post request => counter.post will go up by 1
     */
    fn on_request(&self, request: &mut Request, _: &Data) {
        match request.method() {
            Method::Get => self.get.fetch_add(1, Ordering::Relaxed),
            Method::Post => self.post.fetch_add(1, Ordering::Relaxed),
            Method::Put => self.put.fetch_add(1,Ordering::Relaxed),
            Method::Delete => self.delete.fetch_add(1,Ordering::Relaxed),
            _ => 1
        };
    }
}