use crate::models::{User, Session,Ctf,Error};
use std::sync::Arc;
use tokio::io::{AsyncWriteExt,AsyncReadExt};
pub struct Manager{
    pub guild:String,
    sessions:Vec<Session>,
    users:Vec<User>,
    ctfs:Arc<Vec<Ctf>>
}

impl Manager{
    pub fn new(
        guild:String,
        registry:&Arc<Vec<Ctf>>
    )->Manager{
        Manager{
            guild:guild,
            sessions:vec![],
            users:vec![],
            ctfs:registry.clone()
        }
    }

    pub fn id_user(&self,id:&str)->Option<&User>{
        println!("Identifying user {}",id);
        self.users.iter().find(|user|{
            user.id == id
        })
    }

    pub fn id_session(&self,user:&User)->Option<&Session>{
        println!("Identifying session for user {}",user.id);
        self.sessions.iter().find(|session|{
            session.user_id == user.id
        })
    }

    fn sess_mut(&mut self,user:&User)->Option<&mut Session>{
        println!("Providing session for user {}",user.id);
        self.sessions.iter_mut().find(|session|{
            session.user_id == user.id
        })
    }

    pub fn id_ctf(&self, sess:&Session)->Option<&Ctf>{
        println!("Identifying ctf for id {}",sess.ctf_id);
        self.ctfs.iter().find(|ctf|{
            ctf.id == sess.ctf_id
        })
    }

    pub fn list_ctf(&self)->String{
        println!("Listing ctf");
        self.ctfs.iter()
            .map(|ctf| format!("{}: {}\n",ctf.id,ctf.objective))
            .collect()
    }

    pub fn add_session(&mut self,session:Session){
        println!("Adding session for user {} with ctf {}",session.user_id,session.ctf_id);
        let user:& mut User = self.users.iter_mut().find(|usr| usr.id == session.user_id).unwrap();
        user.ongoing = true;
        self.sessions.push(session);
    }

    pub fn remove_session(&mut self, user:&User){
        println!("Removing session for user {}",user.id);
        self.sessions.retain(|s| s.user_id != user.id);
        let user:& mut User = self.users.iter_mut().find(|usr| usr.id == user.id).unwrap();
        user.ongoing = false;
    }

    pub fn check_flag(&self,flag:&String,user:&User)->bool{
        println!("Checking flag for user {}",user.id);
        let sess = self.id_session(user).unwrap();
        let ctf = self.id_ctf(sess).unwrap();
        flag == &ctf.flag
    }

    pub fn add_user(&mut self,user:User){
        println!("Adding user {}",user.id);
        self.users.push(user);
    }
    

    pub async fn write_to_session(&mut self,user:&User,command:&str)->Result<String, Error>{
        println!("Writing command {} for user {}",command,user.id);
        let cmd = format!("{}\n",command);
        let session:&mut Session = self.sess_mut(user).unwrap();
        if let Err(e) = session.writer.write_all(cmd.as_bytes()).await{
            println!("{}",e);
            return Err("Could not write to session".into());
        };
        
       if let Err(e) =  session.writer.flush().await{
            println!("{}",e);
            return Err("Could not flush stdin".into());
        }

        let mut buffer = Vec::new();
        loop{
            let mut chunk = vec![0;4096];
            match tokio::time::timeout(
                std::time::Duration::from_millis(500),
                session.reader.read(&mut chunk)
            ).await{
                Ok(Ok(n))=>{
                    if n>0{
                        buffer.extend_from_slice(&chunk[..n]);
                    }else{
                        break;
                    }
                },
                Ok(Err(e))=>return Err(e.into()),
                Err(_)=>{
                    break;
                }
            };
        }
        let output = String::from_utf8_lossy(&buffer);
        Ok(output.trim_start().to_string())
    }
}
