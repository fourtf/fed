use skia_safe::Rect;

pub trait Shrink1 {
    fn shrunken_by_1(&self) -> Rect;
}

impl Shrink1 for skia_safe::Rect {
    fn shrunken_by_1(&self) -> Rect {
        Self::from_xywh(self.x(), self.y(), self.width() - 1.0, self.height() - 1.0)
    }
}
