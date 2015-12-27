extern crate robots;

use std::any::Any;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use robots::actors::{Actor, ActorSystem, ActorCell, ActorContext, Arguments, Props};

#[derive(Copy, Clone)]
enum InternalStateMessage {
    Set(u32),
    Get,
    Panic,
}

/// Actor with an internal state that can me modified.
struct InternalState {
    counter: Mutex<u32>,
}

impl Actor for InternalState {
    fn receive<Args: Arguments>(&self,
                                message: Box<Any>,
                                _context: ActorCell<Args, InternalState>) {
        if let Ok(message) = Box::<Any>::downcast::<InternalStateMessage>(message) {
            match *message {
                InternalStateMessage::Get => {
                    println!("internal state: {}", *self.counter.lock().unwrap())
                }
                InternalStateMessage::Set(num) => *self.counter.lock().unwrap() = num,
                InternalStateMessage::Panic => panic!("actor panicked"),
            }
        }
    }
}

impl InternalState {
    fn new(count: u32) -> InternalState {
        InternalState { counter: Mutex::new(count) }
    }
}

fn main() {
    let actor_system = ActorSystem::new("test".to_owned());
    actor_system.spawn_threads(2);

    let restarted_props = Props::new(Arc::new(InternalState::new), 3);
    let restarted_actor_ref_1 = actor_system.actor_of(restarted_props.clone(), "sender".to_owned());
    let restarted_actor_ref_2 = actor_system.actor_of(restarted_props.clone(),
                                                      "receiver".to_owned());

    restarted_actor_ref_1.tell_to(restarted_actor_ref_2.clone(), InternalStateMessage::Get);
    restarted_actor_ref_1.tell_to(restarted_actor_ref_2.clone(), InternalStateMessage::Set(7));
    restarted_actor_ref_1.tell_to(restarted_actor_ref_2.clone(), InternalStateMessage::Get);
    restarted_actor_ref_1.tell_to(restarted_actor_ref_2.clone(), InternalStateMessage::Panic);
    restarted_actor_ref_1.tell_to(restarted_actor_ref_2.clone(), InternalStateMessage::Get);

    std::thread::sleep(Duration::from_millis(10));
    actor_system.shutdown();
}
