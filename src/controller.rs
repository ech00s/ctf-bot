use std::{time::Duration};

use crate::models::{Session,User,Ctf,Error};
use k8s_openapi::api::{core::v1::{Container,PodSpec,PodTemplateSpec,Pod},batch::v1::{Job,JobSpec}};
use kube::{
    api::{Api, AttachParams, DeleteParams, ListParams, PostParams},
    runtime::{wait::{self,conditions::{Condition}}},
    Client, ResourceExt,
};

#[must_use]
pub fn is_pod_ready() -> impl Condition<Job> {
    |obj: Option<&Job>| {
        if let Some(job) = &obj {
            if let Some(s) = &job.status {
                if let Some(ready) = &s.ready{
                    return *ready > 0;
                }
            }
        }
        false
    }
}

pub struct Resources{
    pods:Api<Pod>,
    jobs:Api<Job>,
}

enum State{
    Nready,
    Ready(Resources)
}

pub struct Controller{
    state:State,
    ns:String
}

impl Controller {
    pub fn new(ns:&str)->Controller{
        Controller{ns:ns.to_string(),state:State::Nready}
    }

    fn get_resources(&self)->Result<&Resources,Error>{
        match &self.state{
            State::Ready(resources)=>Ok(resources),
            State::Nready=>Err("Attempting to use unready controller".into())
        }
    }
    
    pub async fn connect(&mut self)->Result<(),Error>{
        println!("Controller connecting");
        let client = Client::try_default().await?; 
        let resources = Resources{
            pods: Api::namespaced(client.clone(), &self.ns),
            jobs: Api::namespaced(client, &self.ns)
        };
        self.state = State::Ready(resources);
        Ok(())
    }

    async fn pod_ready(&self,user:&User)->Result<String,Error>{
        let selector = format!("job-name={}", user.id);
        let resources = self.get_resources()?;
        tokio::time::timeout(
            Duration::from_secs(60), // Set a clear timeout for waiting
            wait::await_condition(
                resources.jobs.clone(), // Target the specific Pod
                user.id.as_str(), // Condition to wait for
                is_pod_ready(),
            )
        ).await??.unwrap();
        
        let lp = ListParams::default().labels(&selector).limit(1);
        let pod_list = resources.pods.list(&lp).await?;
        let pod_name = pod_list.items.get(0)
            .ok_or("Pod not found after Job creation")?
            .name_any();
        println!("Pod ready {}", pod_name);
        Ok(pod_name)
    }

    fn _job(&self,name:&String,image:&String)->Job{
        Job{
             metadata: k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta {
                name: Some(name.clone()),
                labels: Some([("job-name".to_string(), name.clone())].into_iter().collect()),
                ..Default::default()
             },
        spec: Some(JobSpec {
            template: PodTemplateSpec {
                metadata: Some(k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta {
                    labels: Some([("app.kubernetes.io/job-name".to_string(), name.clone())].into_iter().collect()),
                    ..Default::default()
                }),
                spec: Some(PodSpec {
                    containers: vec![Container {
                        name: name.clone(),
                        image: Some(image.clone()),
                        command: Some(vec!["tail".to_string(), "-f".to_string()]),
                        security_context: Some(k8s_openapi::api::core::v1::SecurityContext {
                            privileged: Some(false),
                            allow_privilege_escalation: Some(false),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }],
                    restart_policy: Some("Never".to_string()),
                    ..Default::default()
                }),
            },
            // The number of successful completions to run (1 in this case)
            completions: Some(1), 
            parallelism: Some(1),
            ..Default::default()
        }),
        ..Default::default()
        }
    }

    pub async fn create_job(&self,user:&User,ctf:&Ctf)->Result<String,Error>{    
            println!("Creating job for user {} with ctf {}",user.id,ctf.id);
            let res = self.get_resources()?;
            let name = user.id.clone();
            let data = self._job(&name, &ctf.image);
            if let Err(e) = res.jobs.create(&PostParams::default(), &data).await{
                println!("{}",e);
                return Err("Could not create job".into());
            }
            Ok(self.pod_ready(&user).await?)
    }

    pub async fn delete_job(&self,user:&User)->Result<(),Error>{
        println!("Deleting job for user {}",user.id);
        let res = self.get_resources()?;
        let name = user.id.clone();
        if let Err(_) =  res.jobs.delete(&name, &DeleteParams::default()).await{
            return Err("Could not delete job".into());
        }
        Ok(())
    }

    pub async fn start_session(&self,user:&User,ctf:&Ctf,pod_name:&String)->Result<Session, Error>{
    println!("Starting session for user {} with ctf {}",user.id,ctf.id);
    let res = self.get_resources()?;
    let params = AttachParams::default()
        .stdin(true)
        .stdout(true)
        .stderr(false)
        .tty(false);
    let mut attached = match res.pods.exec(&pod_name, vec!["/bin/bash"],&params).await{
        Ok(something)=>something,
        Err(e)=>{
            println!("{}",e);
            return Err("Could not attach to process".into());
        }
    };
    let writer = match attached.stdin(){
        Some(w)=>w,
        None=>{
            return Err("Could not create input stream".into());
        }
    };
    let reader = match attached.stdout(){
        Some(r)=>r,
        None=>{
            return Err("Could not create output stream".into())
        }
    };
    Ok(Session{
        user_id:user.id.clone(),
        ctf_id:ctf.id.clone(),
        writer:Box::new(writer),
        reader:Box::new(reader)
    })
}

}