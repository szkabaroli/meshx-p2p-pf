use crate::ports::IP2PSharingPort;
use std::cell::RefCell;
use std::rc::Rc;

/* Very basic hexagonal store */
pub struct TestDomain {
    sharing: Rc<RefCell<IP2PSharingPort>>,
}

impl IP2PSharingPort for TestDomain {
    fn inbound_object() {}

    fn inbound_invoke() {}
}

impl TestDomain {
    pub fn new(sharing: Rc<RefCell<IP2PSharingStore>>) -> Self {
        // TODO: logic here

    
        sharing.add_outbound_object(peer_id, obj);
        
        sharing.list_inbound_objects();
        sharing.get_inbound_objects();

        sharing.invoke(peer_id, flow_name, event_object);

        TestDomain { sharing }
    }
}
