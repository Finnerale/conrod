use graph::{Graph, Node, Walker};
use widget::Id;
use position::{Point, Dimensions};
use layout::{Layout, LayoutItem, BoxConstraints};

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

    pub fn size(&mut self, id: Id, constraints: BoxConstraints) -> Dimensions {
        let mut children: Vec<Id> = self.graph
                .children(id)
                .iter(self.graph)
                .map(|it| it.1)
                .collect();

        children.dedup();

        let mut layout = Layout::None;

        if let Node::Widget(ref mut node) = self.graph[id] {
            ::std::mem::swap(&mut layout, &mut node.layout);
        }

        let dim = layout.layout(constraints, children.as_ref(), self);

        if let Node::Widget(ref mut node) = self.graph[id] {
            node.rect.x.end = dim[0];
            node.rect.y.end = dim[1];
        }

        dim
    }
}
