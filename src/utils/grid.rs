fn convert_to_number(path: &str) -> Vec<usize> {
    path.split("/")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}

#[derive(Debug, Clone, Copy)]
pub struct GridResult {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Grid {
    width: f32,
    height: f32,
    columns: usize,
    rows: usize,
}

impl Grid {
    pub fn new(width: f32, height: f32, area: &str) -> Self {
        let [columns, rows] = {
            let numbers = convert_to_number(area);
            [numbers[0], numbers[1]]
        };

        Self {
            width,
            height,
            columns,
            rows,
        }
    }

    pub fn get_position(&self, area: &str) -> GridResult {
        let single_column = self.width as usize / self.columns;
        let single_row = self.height as usize / self.rows;

        let [col_start, col_end, row_start, row_end] = {
            let numbers = convert_to_number(area);
            [numbers[0], numbers[1], numbers[2], numbers[3]]
        };

        assert!(col_start <= col_end);

        let (mut height_column, mut width_column) = { (row_end - row_start, col_end - col_start) };

        height_column = if height_column > 0 { height_column } else { 1 };
        width_column = if width_column > 0 { width_column } else { 1 };

        let width = width_column * single_column;
        let height = height_column * single_row + height_column / 2;
        GridResult {
            width: width as f32,
            height: height as f32,
            x: (single_column * col_start) as f32,
            y: (single_row * row_start) as f32,
        }
    }
}
