#![feature(linked_list_remove)]

mod io;
mod path;

fn main() {
  let (start, end) = io::read_tuple();

  let matrix = io::read_matrix();

  let mut vertices = std::collections::LinkedList::new();
  for i in 0..matrix.len() {
    if i != start {
      vertices.push_back(i);
    }
  }

  path::find_paths(&matrix, &mut vertices, start, end, &vec![start]);
}
