#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Diagonal,
    Horizontal,
}

#[derive(Copy, Clone, Debug)]
pub struct Style {
    pub width: f32,
    pub height: f32,
    pub x: f32,
    pub y: f32,
    pub radius: f32,

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
            width: 100.0,
            height: 100.0,
            x: 100.0,
            y: 100.0,
            radius: 100.0,

            direction: Direction::Horizontal,
            row_gap: 0.0,
            column_gap: 0.0,

            columns: 1,
            rows: 1,
            row_start: 1,
            row_end: 1,
            column_start: 1,
            column_end: 1,
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

        let width = width_column * single_column;
        let height = height_column * single_row + height_column / 2;

        Style {
            x: (single_column * child.column_start) as f32,
            y: (single_row * child.row_start) as f32,
            width: width as f32,
            height: height as f32,
            ..child
        }
    }
}
