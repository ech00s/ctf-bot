use std::{sync::Arc};
use tokio::sync::{Mutex,RwLock};
use crate::{controller::Controller, manager::Manager, models::{Ctf,User,Error}};

type ManMutex = Arc<Mutex<Manager>>;
type Managers = Arc<RwLock<Vec<ManMutex>>>;
pub struct Bot {
    pub ctfs:Arc<Vec<Ctf>>,
    pub managers:Managers,
    pub control:Controller
} // User Bot, which is stored and accessible in all command invocations

impl Bot{
    pub async fn add_manager(&self,guild_id:String){
        println!("Adding manager {}",guild_id);
        let mut managers = self.managers.write().await;
        managers.push(Arc::new(Mutex::new(Manager::new(guild_id,&self.ctfs))));
    }

    pub async fn man_exists(&self,guild_id:&String)->bool{
        println!("Checking manager {}",guild_id);
        let managers = self.managers.read().await;
        for manager in managers.iter(){
            
            let l = manager.lock().await;
            if &l.guild == guild_id{
                return true;
            }
        }
        false
    }


    pub async fn man_mut(&self,guild_id:&String)->ManMutex{
        println!("Providing manager {}",guild_id);
        let managers = self.managers.read().await;
        for manager in managers.iter(){
                let l = manager.lock().await;
                if &l.guild == guild_id{
                    return manager.clone()
                }
        }
        panic!("Manager not found {}", guild_id);
    }

    pub async fn valid_usr(&self,guild_id:&String,user_id:&String)->User{
        println!("Validating user {} from guild {}",user_id,guild_id);
        let m = self.man_mut(guild_id).await;
        let mut man = m.lock().await;
        match man.id_user(user_id){
            Some(usr)=>usr.to_owned(),
            None=>{
                let some = User{id:user_id.to_string(),ongoing:false};
                man.add_user(some.clone());
                some
            }
        }
    }

    
    pub async fn new(ctfs:Vec<Ctf>)->Result<Bot,Error>{
        println!("Creating bot with {} ctfs",ctfs.len());
        let mut control = Controller::new("ctf-jobs");
        control.connect().await?;
        Ok(Bot{
            ctfs:Arc::new(ctfs),
            managers:Arc::new(RwLock::new(vec![])),
            control:control
        })
    }

}
