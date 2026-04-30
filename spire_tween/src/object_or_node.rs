use super::*;

#[delegated_enum]
#[derive(Debug, Clone)]
pub enum ObjectOrNode {
    Object(Gd<Object>),
    Node(Gd<Node>),
}

#[delegate_impl]
impl ObjectOrNode {
    pub fn instance_id_unchecked(&self) -> InstanceId;
}

impl ObjectOrNode {
    pub fn is_instance_valid(&self) -> bool {
        match self {
            ObjectOrNode::Object(obj) => is_instance_id_valid(obj.instance_id_unchecked().to_i64()),
            ObjectOrNode::Node(node) => {
                // Validate ID before cloning — official gdext panics on clone of freed instances.
                if !is_instance_id_valid(node.instance_id_unchecked().to_i64()) {
                    return false;
                }
                match TM.node_get_status_fresh(node.clone()) {
                    NodeStatus::InsideTree | NodeStatus::OutsideTreeMaybeDead => true,
                    NodeStatus::Dead => false,
                }
            }
        }
    }
}

impl ObjectOrNode {
    pub fn from_unchecked_object(obj: Gd<Object>) -> Self {
        match obj.try_cast::<Node>() {
            Ok(node) => Self::Node(node),
            Err(obj) => Self::Object(obj.upcast()),
        }
    }

    /// Panics if the underlying instance has been freed.
    pub fn to_object(&self) -> Gd<Object> {
        match self {
            ObjectOrNode::Object(obj) => {
                debug_assert!(is_instance_id_valid(obj.instance_id_unchecked().to_i64()));
                obj.clone()
            }
            ObjectOrNode::Node(node) => {
                debug_assert!(is_instance_id_valid(node.instance_id_unchecked().to_i64()));
                node.clone().upcast()
            }
        }
    }

    pub fn into_object(self) -> Gd<Object> {
        match self {
            ObjectOrNode::Object(obj) => obj,
            ObjectOrNode::Node(node) => node.upcast(),
        }
    }
}

impl From<Gd<Node>> for ObjectOrNode {
    fn from(node: Gd<Node>) -> Self { Self::Node(node) }
}
