#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Diagonal,
    Horizontal,
}

#[derive(Copy, Clone, Debug)]
pub struct Margin {
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
}

impl Default for Margin {
    fn default() -> Self {
        Self {
            top: 0.0,
            bottom: 0.0,
            left: 0.0,
            right: 0.0,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Style {
    pub width: f32,
    pub height: f32,
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub margin: Margin,
    pub direction: Direction,
    pub row_gap: f32,
    pub column_gap: f32,

    pub columns: usize,
    pub rows: usize,
    pub row_start: usize,
    pub row_end: usize,
    pub column_start: usize,
    pub column_end: usize,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            width: 0.0,
            height: 0.0,
            x: 0.0,
            y: 0.0,
            radius: 0.0,
            margin: Margin::default(),
            direction: Direction::Horizontal,
            row_gap: 0.0,
            column_gap: 0.0,

            columns: 0,
            rows: 0,
            row_start: 0,
            row_end: 0,
            column_start: 0,
            column_end: 0,
        }
    }
}

impl Style {
    pub fn calculate_style(parent: Style, child: Style) -> Style {
        let single_column = parent.width as usize / parent.columns;
        let single_row = parent.height as usize / parent.rows;

        assert!(child.column_start <= child.column_end);
        assert!(child.row_start <= child.row_end);

        let (mut height_column, mut width_column) = {
            (
                child.row_end - child.row_start,
                child.column_end - child.column_start,
            )
        };

        height_column = if height_column > 0 { height_column } else { 1 };
        width_column = if width_column > 0 { width_column } else { 1 };

        let width = width_column * single_column + width_column;
        let height = height_column * single_row + height_column;

        Style {
            x: (single_column * child.column_start) as f32 + parent.x + child.margin.left,
            y: (single_row * child.row_start) as f32 + parent.y + child.margin.top,
            width: width as f32 + child.margin.right,
            height: height as f32 + child.margin.bottom,
            ..child
        }
    }
}
