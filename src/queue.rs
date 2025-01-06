use std::sync::mpsc;
use std::thread;

pub trait Task {
    type Output: Send;
    fn run(&self) -> Option<Self::Output>;
}

pub struct WorkQueue<TaskType: 'static + Task + Send> {
    send_tasks: Option<spmc::Sender<TaskType>>,
    recv_tasks: spmc::Receiver<TaskType>,
    recv_output: mpsc::Receiver<TaskType::Output>,
    workers: Vec<thread::JoinHandle<()>>
}

impl<TaskType: 'static + Task + Send> WorkQueue<TaskType> {

    pub fn new(n_workers: usize) -> WorkQueue<TaskType> {
        // single-producer (main thread), multiple consumer (threads)
        let (main_task_sender, consumer_task_recver) = spmc::channel();

        // multiple-producer (threads with potential hash), multiple consumer (main thread)
        let (consumer_result_sender, main_result_recver) = mpsc::channel();

        let mut threads: Vec<thread::JoinHandle<()>> = Vec::with_capacity(n_workers);

        for _ in 0..n_workers {
            // give each thread a copy of the receiving and sending channels
            let recver_clone: spmc::Receiver<TaskType> = consumer_task_recver.clone();
            let sender_clone: mpsc::Sender<<TaskType as Task>::Output> = consumer_result_sender.clone();

            let thread: thread::JoinHandle<()> = thread::spawn(|| Self::run(recver_clone, sender_clone));
            threads.push(thread);
        }

        WorkQueue { 
            send_tasks: Some(main_task_sender), 
            recv_tasks: consumer_task_recver, 
            recv_output: main_result_recver, 
            workers: threads 
        }
    }

    fn run(recv_tasks: spmc::Receiver<TaskType>, send_output: mpsc::Sender<TaskType::Output>) {
        // main logic for a worker thread
        loop {
            let task_result: Result<TaskType, mpsc::RecvError> = recv_tasks.recv();
            
            match task_result {
                Ok(task) => {
                    let task_run_option: Option<<TaskType as Task>::Output> = task.run();
                    
                    // if valid result produced, send it to the ouput channel
                    if let Some(valid_result) = task_run_option {
                        match send_output.send(valid_result) {
                            Ok(()) => {}
                            Err(_) => return
                        }
                    } // else keep looping
                }
                Err(_) => return
            }
        }
    }

    pub fn enqueue(&mut self, t: TaskType) -> Result<(), mpsc::SendError<TaskType>> {
        if let Some(send_tasks) = &mut self.send_tasks {
            send_tasks.send(t)
        } else {
            panic!("Sender channel destoryed.")
        }
    }

    // Helper methods
    pub fn recv(&mut self) -> TaskType::Output {
        self.recv_output
            .recv()
            .expect("I have been shutdown incorrectly")
    }

    pub fn shutdown(&mut self) {
        self.send_tasks = None;
        
        // https://stackoverflow.com/questions/70404178/what-is-the-best-way-to-drain-a-mspc-channel-in-rust
        // 'keep receiving tasks as long as they have Ok result'
        while let Ok(_) = self.recv_tasks.recv() {}
    
        let drained_threads = self.workers.drain(..);

        for thread in drained_threads { 
            thread.join().expect("Thread didn't Join OK");
        }
    }
}

impl<TaskType: 'static + Task + Send> Drop for WorkQueue<TaskType> {
    fn drop(&mut self) {
        // "Finalisation in destructors" pattern: https://rust-unofficial.github.io/patterns/idioms/dtor-finally.html
        match self.send_tasks {
            None => {} // already shut down
            Some(_) => self.shutdown(),
        }
    }
}
