use Dimensions;
use Scalar;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct BoxConstraints {
    pub min_width: Scalar,
    pub max_width: Scalar,
    pub min_height: Scalar,
    pub max_height: Scalar,
}

impl BoxConstraints {

    pub fn min_width(mut self, min_width: Scalar) -> Self {
        self.min_width = min_width;
        self
    }

    pub fn max_width(mut self, max_width: Scalar) -> Self {
        self.max_width = max_width;
        self
    }

    pub fn min_height(mut self, min_height: Scalar) -> Self {
        self.min_height = min_height;
        self
    }

    pub fn max_height(mut self, max_height: Scalar) -> Self {
        self.max_height = max_height;
        self
    }

    pub fn fit_width(mut self, width: Scalar) -> Self {
        self.min_width(width).max_width(width)
    }

    pub fn fit_height(mut self, height: Scalar) -> Self {
        self.min_height(height).max_height(height)
    }

    pub fn fit(mut self, dimensions: Dimensions) -> Self {
        self.fit_width(dimensions[0]).fit_height(dimensions[1])
    }

    pub fn grow_to_max(mut self) -> Self {
        self.fit([self.max_width, self.max_height])
    }

    pub fn check_width(&self, width: Scalar) -> Scalar {
        if width < self.min_width {
            self.min_width
        } else if width < self.min_width {
            self.max_width
        } else {
            width
        }
    }

    pub fn check_height(&self, height: Scalar) -> Scalar {
        if height < self.min_height {
            self.min_height
        } else if height < self.min_height {
            self.max_height
        } else {
            height
        }
    }
}
