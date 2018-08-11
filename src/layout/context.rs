use Ui;
use graph::{Graph, Node, Walker};
use widget::Id;
use position::{Point, Dimensions};
use layout::{Layout, LayoutItem, BoxConstraints};

pub struct LayoutContext<'a> {
    current_id: Option<Id>,
    ui: &'a mut Ui,
}

impl<'a> LayoutContext<'a> {
    pub fn new(ui: &'a mut Ui) -> Self {
        LayoutContext {
            current_id: None,
            ui,
        }
    }

    pub fn id(&self) -> Id {
        self.current_id.unwrap()
    }

    fn graph(&self) -> &Graph {
        self.ui.widget_graph()
    }

    fn graph_mut(&mut self) -> &mut Graph {
        self.ui.widget_graph_mut()
    }

    pub fn ui(&self) -> &Ui {
        &self.ui
    }

    pub fn layout_item(&self, id: Id) -> &LayoutItem {
        if let Node::Widget(ref node) = self.graph()[id] {
            &node.layout_item
        } else {
            panic!("")
        }
    }

    pub fn position(&mut self, id: Id, pos: Point) {
        if let Node::Widget(ref mut node) = self.graph_mut()[id] {
            node.position = pos;
        } else {
            panic!("")
        }
    }

    pub fn get_size(&self, id: Id) -> Option<Dimensions> {
        if let Node::Widget(ref node) = self.graph()[id] {
            Some([node.rect.w(), node.rect.h()])
        } else {
            None
        }
    }

    pub fn size(&mut self, id: Id, constraints: BoxConstraints) -> Dimensions {
        let mut children: Vec<Id> = self.graph()
                .children(id)
                .iter(self.graph())
                .map(|it| it.1)
                .collect();

        children.dedup();

        let mut layout = Layout::None;

        if let Node::Widget(ref mut node) = self.graph_mut()[id] {
            ::std::mem::swap(&mut layout, &mut node.layout);
        }

        let old_id = self.current_id;
        self.current_id = Some(id);

        let dim = layout.layout(constraints, children.as_ref(), self);

        self.current_id = old_id;

        if let Node::Widget(ref mut node) = self.graph_mut()[id] {
            node.size = dim;
        }

        dim
    }
}
