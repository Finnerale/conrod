use graph::{Graph, Node};
use widget::Id;
use position::{Rect, Point, Dimensions};
use layout::enums::LayoutItem;

pub struct LayoutContext<'a> {
    graph: &'a mut Graph,
}

impl<'a> LayoutContext<'a> {
    pub fn new(graph: &'a mut Graph) -> Self {
        LayoutContext { graph }
    }

    pub fn layout_item(&self, id: Id) -> &LayoutItem {
        if let Node::Widget(ref node) = self.graph[id] {
            &node.layout_item
        } else {
            panic!("")
        }
    }

    pub fn position(&mut self, id: Id, pos: Point) {
        if let Node::Widget(ref mut node) = self.graph[id] {
            node.rect.x.start = pos[0];
            node.rect.y.start = pos[1];
        } else {
            panic!("")
        }
    }

    pub fn get_size(&self, id: Id) -> Option<Dimensions> {
        if let Node::Widget(ref node) = self.graph[id] {
            Some([node.rect.w(), node.rect.h()])
        } else {
            None
        }
    }
}
