use::glam;
use glam::IVec2;
use itertools::Itertools;

#[derive(Debug, Clone, Default)]
pub struct Grid<T>(Vec<Vec<T>>)
where T : Clone;
impl<T: Clone> Grid<T> {
    pub fn cells_enumerate(&self) -> impl Iterator<Item = (IVec2, &T)>{
        self.0.iter().enumerate().flat_map(|(row, chars)| {
            chars.iter().enumerate().map(move |(col, c)|{
                (IVec2 {x: col as i32, y: row as i32}, c)
            })
        })
    }
    pub fn find_position_first<P: FnMut(&T) -> bool>(&self, mut predicate: P) -> Option<IVec2>{
        for y in 0..self.height() as i32 {
            for x in 0..self.width() as i32 {
                if predicate(self.get(IVec2 {x, y})?) {return Some(IVec2{x, y}) };
            }
        }
        None
    }
    pub fn coords(&self) -> impl Iterator<Item = IVec2>{
        (0..self.height()).cartesian_product(0..self.width()).map(|(y,x)|IVec2 {x: x as i32,y: y as i32})
    }
    pub fn default() -> Self{
        Grid(vec![vec![]])
    }
    pub fn new_from(vec: Vec<Vec<T>>) -> Grid<T> {
        Grid(vec)
    }
    pub fn clone_map<B: Clone, F: FnMut(T) -> B>(&self, func: F) -> Grid<B> {
        Grid::from_flat(self.clone().into_iter().map(func), self.width())
    }
    pub fn clone_map_enumerate<B: Clone, F: FnMut((IVec2, &T)) -> B>(&self, func: F) -> Grid<B> {
        Grid::from_flat(self.clone().cells_enumerate().map(func), self.width())
    }
    pub fn cells(&self) -> impl Iterator<Item = &T>{
        self.0.iter().flat_map(|row| row.iter())
    }
    pub fn into_cells(self) -> impl Iterator<Item = T>{
        self.0.into_iter().flat_map(|row| row.into_iter())
    }
    pub fn width(&self) -> usize {
        self.0[0].len()
    }
    pub fn height(&self) -> usize {
        self.0.len()
    }
    pub fn size(&self) -> IVec2{
        IVec2{
            x: self.width() as i32,
            y: self.height() as i32
        }
    }
    pub fn contains_coords(&self, pos: IVec2) -> bool{
        0 <= pos.x && pos.x < self.width() as i32 &&
        0 <= pos.y && pos.y < self.height() as i32
    }
    pub fn get(&self, pos: IVec2) -> Option<&T> {
        if !self.contains_coords(pos) {return None;}
        Some(&self.0[pos.y as usize][pos.x as usize])
    }
    pub fn get_unchecked(&self, pos: IVec2) -> T {
        self.0[pos.y as usize][pos.x as usize].clone()
    }
    pub fn get_unchecked_mut(&mut self, pos: IVec2) -> &mut T {
        if !self.contains_coords(pos) {panic!("get_unchecked_mut does not incluee {pos:?}");}
        &mut self.0[pos.y as usize][pos.x as usize]
    }
    pub fn adjacent_eight_cells(&self, pos: IVec2) -> impl Iterator <Item = IVec2>{
        [
            (-1, -1), ( 0, -1), ( 1, -1),
            (-1,  0),           ( 1,  0),
            (-1,  1), ( 0,  1), ( 1,  1),
        ].into_iter().map(move |(x, y)| pos + IVec2{x, y})
        // .filter(|&pos|self.contains_coords(pos))
    } 
    pub fn adjacent_cells(&self, pos: IVec2) -> impl Iterator <Item = IVec2>{
        [
                      ( 0, -1), 
            (-1,  0),           ( 1,  0),
                      ( 0,  1), 
        ].into_iter().map(move |(x, y)| pos + IVec2{x, y})
        // .filter(|&pos|self.contains_coords(pos))
    } 

    pub fn set_cell(&mut self, pos: IVec2, val: T){
        if !self.contains_coords(pos) {panic!("tried to set out of bounds :{}", pos);}
        self.0[pos.y as usize][pos.x as usize] = val;
    }
}
impl Grid<char> {
    pub fn to_string(self) -> String {
        self.0.join(&'\n').into_iter().collect::<String>()
    }
}
impl Grid<String> {
    pub fn to_string(self) -> String {
        self.0.join(&"\n".to_string()).into_iter().collect::<String>()
    }
}
impl Grid<bool> {
    pub fn to_string(self) -> String {
        self.clone_map(|x|{
            match x {
                true => {'#'}
                false => {'.'}
            }
        }).to_string()
    }
}
impl Grid<char> {
    pub fn from_str(string: &str) -> Self {
        Grid(string.lines().map(|l| l.chars().collect()).collect())
    }
}
impl<T: Clone> IntoIterator for Grid<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter().flatten().collect::<Vec<_>>().into_iter()
    }
}

impl<T: Clone> Grid<T> {
    pub fn from_flat<I: IntoIterator<Item = T>>(iter: I, width: usize) -> Self {
        let items: Vec<T> = iter.into_iter().collect();
        let grid = items
            .chunks(width)
            .map(|chunk| chunk.to_vec())
            .collect();
        Grid(grid)
    }
}