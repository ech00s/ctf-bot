

use std::error::Error as E;
pub type Error = Box<dyn E + Send + Sync>;
use tokio::io::{AsyncRead, AsyncWrite};

//immutable in principle, provide closure lookup
#[derive(Clone)]
pub struct Ctf{
    pub image:String,
    pub id:String,
    pub objective:String,
    pub flag:String
}

//immutable after first assignment
#[derive(Clone)]
pub struct User{
    pub id:String,
    pub ongoing:bool
}
pub struct Session{
    pub user_id:String,
    pub ctf_id:String,
    pub writer:Box<dyn AsyncWrite+ Send+Unpin>,
    pub reader:Box<dyn AsyncRead + Send+Unpin>
}
