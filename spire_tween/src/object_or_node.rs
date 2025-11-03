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
                match TM.node_get_status_fresh(*node) {
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

    pub fn to_object(&self) -> Gd<Object> {
        match self {
            ObjectOrNode::Object(obj) => obj.clone(),
            ObjectOrNode::Node(node) => node.upcast(),
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
