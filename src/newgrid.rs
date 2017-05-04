use image::{GenericImage, Pixel};

struct PixelEnergyPoint<P: Pixel> {
    pixel: P,
    energy: usize,
    path_energy: usize,
}

impl<P: Pixel> PixelEnergyPoint<P> {}

struct EnergyGrid<P: Pixel> {
    points: Vec<Vec<PixelEnergyPoint<P>>>,
}

impl<P: Pixel> EnergyGrid<P> {
    fn from_image<I>(image: &I) -> Self
        where I: GenericImage<Pixel = P>
    {
        let (width, height) = image.dimensions();
        let mut rows = vec![];
        for y in 0..height {
            let mut row = vec![];
            for x in 0..width {
                let pixel = image.get_pixel(x, y);
                let energy = 0;
                let path_energy = 0;
                let pep = PixelEnergyPoint {
                    pixel,
                    energy,
                    path_energy,
                };
                row.push(pep);
            }
            rows.push(row);
        }
        let mut grid = EnergyGrid { points: rows };
        grid.recalculate_all();
        grid
    }

    fn height(&self) -> usize {
        self.points.len()
    }

    fn width(&self) -> usize {
        self.points[0].len()
    }

    fn get(&self, x: isize, y: isize) -> &PixelEnergyPoint<P> {
        unimplemented!()
        // &self.points[y][x]
    }

    fn get_mut(&mut self, x: isize, y: isize) -> &mut PixelEnergyPoint<P> {
        unimplemented!()
        // &mut self.points[y][x]
    }

    fn recalculate_all(&mut self) {
        self.calculate_first_row_energy();
        for y in 1..self.height() {
            for x in 0..self.width() {
                self.calculate_both_energy(x, y);
            }
        }
    }

    fn calculate_first_row_energy(&mut self) {
        for x in 0..self.width() {
            self.calculate_energy(x, 0);
        }
    }

    fn calculate_both_energy(&mut self, x: usize, y: usize) {
        self.calculate_energy(x, y);
        self.calculate_path_energy(x, y);
    }

    fn calculate_energy(&mut self, x: usize, y: usize) {
        unimplemented!()
    }

    fn calculate_path_energy(&mut self, x: usize, y: usize) {
        unimplemented!()
    }
}
