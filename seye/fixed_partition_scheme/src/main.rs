
#[derive(Clone)]
struct Job {
    name: String,
    job_size: i32,
}


struct Partition {
    partition_size: i32,
    status: String,
    access: Option<Job>,
}

impl Partition{
    fn set_access_job(&mut self,job:Job,order:i32){
        self.access = Some(job);
        self.status = String::from("busy");
        println!("Partition {} has been occupied by {}",order,self.access.as_ref().unwrap().name);
    }

    fn free_partition_space(&mut self){
        self.status = String::from("free");
        self.access = None;
    }
}

struct MainMemory{
    size: i32,
    partitions: Vec<Partition>
}
impl MainMemory{
    
    //get partition state of each memory oartition
    fn get_partition_status(&self) {
        let mut count = 0;
        for obj in  &self.partitions{
            count += 1;
            if obj.status.eq("busy"){
                println!("Partition {} has been assigned {}",count,obj.access.as_ref().unwrap().name);
            }
            else{
                println!("Partition {} is currently free",count);
            }
        }
    }

    fn load_job_into_partition(&mut self,job:Job){
        let  size:i32 = job.job_size;
        let new_job = Job{
            name:job.name,
            job_size:job.job_size
        };
        if job.job_size > self.size {
            println!("No partition space available for this job.");
        
        }else{
            let mut count = 0;
            for obj in &mut  self.partitions{
                count += 1;
                if obj.partition_size >  size{
                    if obj.status.eq("busy"){
                        println!("Partition {} is in a {} state and currently processing {}",count,obj.status,obj.access.as_ref().unwrap().name);
                    }else{
                        obj.set_access_job(new_job.clone(),count);
                        break;
                    }
                }
                else{
                    println!("Partition {} doesn't have enough space to process this job",count);
                }
            }
            
        }
    }

    fn clear_partition(&mut self,index:usize){
        self.partitions[index].free_partition_space();
        println!("Partition {} has been freed",index);
    }
}



fn main(){
    let mut main_memory = MainMemory {
        size: 200,
        partitions: vec![Partition{
            partition_size: 100,
            status: String::from("Free"),
            access: None
        },Partition{
            partition_size: 25,
            status: String::from("Free"),
            access:None
        },Partition{
            partition_size: 25,
            status: String::from("Free"),
            access: None
        },Partition{
            partition_size: 50,
            status: String::from("Free"),
            access: None
        }]
    };

    let  qued_jobs = vec![
        Job{
            name: String::from("Job 1"),
            job_size: 80
        },Job{
            name: String::from("Job 2"),
            job_size: 110
        },Job{
            name: String::from("Job 3"),
            job_size: 10
        },Job{
            name: String::from("Job 4"),
            job_size: 15
        },Job{
            name: String::from("Job 5"),
            job_size: 30
        },Job{
            name: String::from("Job 6"),
            job_size: 5
        },
    ];
    

    main_memory.load_job_into_partition(qued_jobs[0].clone());    
    main_memory.load_job_into_partition(qued_jobs[1].clone());
    main_memory.load_job_into_partition(qued_jobs[2].clone());
    main_memory.load_job_into_partition(qued_jobs[3].clone());
    main_memory.clear_partition(2);
    main_memory.clear_partition(3);
    main_memory.load_job_into_partition(qued_jobs[4].clone());
    main_memory.load_job_into_partition(qued_jobs[5].clone());
    main_memory.get_partition_status();
}

