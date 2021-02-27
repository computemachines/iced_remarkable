use std::iter;

use iced_graphics::Primitive;

pub struct DepthFirstPrimitiveIterator<'a> {
    queue: Vec<(usize, &'a Primitive)>,
}

impl<'a> DepthFirstPrimitiveIterator<'a> {
    pub fn new(primitive_root: &'a Primitive) -> Self {
        Self {
            queue: vec![(0, primitive_root)],
        }
    }
}

pub fn primitive_children(primitive: &Primitive) -> Box<dyn Iterator<Item = &Primitive> + '_> {
    match primitive {
        Primitive::None => Box::new(iter::empty()),
        Primitive::Group { primitives } => Box::new(primitives.iter()),
        Primitive::Text {
            content,
            bounds,
            color,
            size,
            font,
            horizontal_alignment,
            vertical_alignment,
        } => Box::new(iter::empty()),
        Primitive::Quad {
            bounds,
            background,
            border_radius,
            border_width,
            border_color,
        } => Box::new(iter::empty()),
        Primitive::Image { handle, bounds } => Box::new(iter::empty()),
        Primitive::Svg { handle, bounds } => Box::new(iter::empty()),
        Primitive::Clip {
            bounds,
            offset,
            content,
        } => todo!(),
        Primitive::Translate {
            translation,
            content,
        } => todo!(),
        Primitive::Mesh2D { buffers, size } => Box::new(iter::empty()),
        Primitive::Cached { cache } => {
            unimplemented!("Have not implemented Cached Primitves yet!!!")
        }
    }
}

impl<'a> Iterator for DepthFirstPrimitiveIterator<'a> {
    type Item = &'a Primitive;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((depth, node)) = self.queue.pop() {
            let children = primitive_children(node);
            self.queue.extend(children.map(|child| (depth + 1, child)));
            Some(node)
        } else {
            None
        }
    }
}
