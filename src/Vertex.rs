use std::cmp::Ordering;
use std::rc::Rc;

pub struct Vertex {
    pub x: f64,
    pub y: f64,
    pub is_ghost: bool,
}

impl Vertex {
    pub fn new(x: f64, y: f64) -> Vertex {
        Vertex {
            x: x,
            y: y,
            is_ghost: false,
        }
    }

    pub fn new_ghost() -> Vertex {
        Vertex {
            x: 0.0,
            y: 0.0,
            is_ghost: true,
        }
    }

    pub fn from_coordinates(raw_array: Vec<f64>) -> Vec<Rc<Vertex>> {
        if raw_array.len() % 2 != 0 {
            panic!("Array must provide vertices by pair of x,y coordinates.");
        }

        let list_size = raw_array.len() / 2;

        let mut vertex_list: Vec<Rc<Vertex>> = Vec::with_capacity(list_size);

        for index in 0..list_size {
            let x = raw_array.get(index * 2).unwrap();
            let y = raw_array.get(index * 2 + 1).unwrap();

            let new_vertex = Vertex::new(*x, *y);
            vertex_list.push(Rc::new(new_vertex));
        }

        return vertex_list;
    }

    pub fn sort(vertex_list: &mut Vec<Rc<Vertex>>) {
        vertex_list.sort_by(|v1, v2| match v1.x.partial_cmp(&v2.x) {
            Some(Ordering::Equal) => v1.y.partial_cmp(&v2.y).unwrap(),
            _ => v1.x.partial_cmp(&v2.y).unwrap(),
        });
    }
}

#[cfg(test)]
mod ghost_vertex {
    use super::*;

    #[test]
    fn test_ghost_property_is_bool() {
        let v = Vertex::new_ghost();
        assert!(v.is_ghost);

        let v = Vertex::new(0.0, 0.0);
        assert!(!v.is_ghost);
    }
}

#[cfg(test)]
mod build_from_coordinates {
    use super::*;

    #[test]
    fn test_builds_all_vertices() {
        let mut raw_array = Vec::new();

        raw_array.push(0.0);
        raw_array.push(1.0);
        raw_array.push(4.0);
        raw_array.push(5.0);
        raw_array.push(2.0);
        raw_array.push(3.0);

        let mut vertex_list = Vertex::from_coordinates(raw_array);

        assert_eq!(vertex_list.len(), 3);

        assert_eq!(vertex_list.get(0).unwrap().x, 0.0);
        assert_eq!(vertex_list.get(0).unwrap().y, 1.0);

        assert_eq!(vertex_list.get(1).unwrap().x, 4.0);
        assert_eq!(vertex_list.get(1).unwrap().y, 5.0);

        assert_eq!(vertex_list.get(2).unwrap().x, 2.0);
        assert_eq!(vertex_list.get(2).unwrap().y, 3.0);

        Vertex::sort(&mut vertex_list);

        assert_eq!(vertex_list.get(1).unwrap().x, 2.0);
        assert_eq!(vertex_list.get(1).unwrap().y, 3.0);

        assert_eq!(vertex_list.get(2).unwrap().x, 4.0);
        assert_eq!(vertex_list.get(2).unwrap().y, 5.0);
    }

    #[test]
    #[should_panic]
    fn test_dont_accept_wrong_size_array() {
        let mut raw_array = Vec::new();

        raw_array.push(0.0);
        raw_array.push(0.0);
        raw_array.push(1.0);
        raw_array.push(0.0);
        raw_array.push(0.0);
        raw_array.push(1.0);
        raw_array.push(2.0);

        let vertex_list = Vertex::from_coordinates(raw_array);
    }
}