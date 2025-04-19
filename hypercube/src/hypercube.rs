use bitmaps::Bitmap;

macro_rules! hypercube_by_dim {
    ($struct_name:ident, $dim:expr) => {
        #[derive(Debug, Clone)]
        pub struct $struct_name {
            pub labels: Vec<usize>,
            pub adjacency: Vec<Bitmap<{ 2_u32.pow($dim as u32) as usize }>>,
            pub edges: Vec<(usize, usize)>,
        }
        impl $struct_name {
            pub fn new() -> Self {
                let size = 2_u32.pow($dim as u32) as usize;
                let labels: Vec<usize> = (0..size).collect();
                let mut adjacency: Vec<Bitmap<{ 2_u32.pow($dim as u32) as usize }>> = Vec::new();
                let mut edges = Vec::new();

                for _ in 0..size {
                    adjacency.push(Bitmap::<{ 2_u32.pow($dim as u32) as usize }>::new())
                }

                for i in 0..size {
                    for j in i..size {
                        if (i as usize ^ j as usize).count_ones() == 1 {
                            adjacency[i].set(j, true);
                            adjacency[j].set(i, true);
                            edges.push((i, j));
                        }
                    }
                }
                Self {
                    labels,
                    adjacency,
                    edges,
                }
            }
        }
    };
}

hypercube_by_dim!(Hypercube0, 0);
hypercube_by_dim!(Hypercube1, 1);
hypercube_by_dim!(Hypercube2, 2);
hypercube_by_dim!(Hypercube3, 3);
hypercube_by_dim!(Hypercube4, 4);
hypercube_by_dim!(Hypercube5, 5);
hypercube_by_dim!(Hypercube6, 6);
hypercube_by_dim!(Hypercube7, 7);
hypercube_by_dim!(Hypercube8, 8);
hypercube_by_dim!(Hypercube9, 9);
hypercube_by_dim!(Hypercube10, 10);

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn hypercube0() {
        let hypercube = Hypercube0::new();
        assert_eq!(Vec::<(usize, usize)>::new(), hypercube.edges);
        assert_eq!(u32::pow(2, 0) as usize, hypercube.adjacency.len());
    }
    #[test]
    fn hypercube1() {
        let hypercube = Hypercube1::new();
        assert_eq!(vec![(0, 1)], hypercube.edges);
        assert_eq!(u32::pow(2, 1) as usize, hypercube.adjacency.len());
    }
    #[test]
    fn hypercube2() {
        let hypercube = Hypercube2::new();
        assert_eq!(vec![(0, 1), (0, 2), (1, 3), (2, 3)], hypercube.edges);
        assert_eq!(u32::pow(2, 2) as usize, hypercube.adjacency.len());
    }
    #[test]
    fn hypercube3() {
        let hypercube = Hypercube3::new();
        assert_eq!(
            vec![
                (0, 1),
                (0, 2),
                (0, 4),
                (1, 3),
                (1, 5),
                (2, 3),
                (2, 6),
                (3, 7),
                (4, 5),
                (4, 6),
                (5, 7),
                (6, 7)
            ],
            hypercube.edges
        );
        assert_eq!(u32::pow(2, 3) as usize, hypercube.adjacency.len());
    }
    #[test]
    fn hypercube4() {
        let hypercube = Hypercube4::new();
        assert_eq!(32, hypercube.edges.len());
        assert_eq!(u32::pow(2, 4) as usize, hypercube.adjacency.len());
    }
    #[test]
    fn hypercube5() {
        let hypercube = Hypercube5::new();
        assert_eq!(80, hypercube.edges.len());
        assert_eq!(u32::pow(2, 5) as usize, hypercube.adjacency.len());
    }
    #[test]
    fn hypercube6() {
        let hypercube = Hypercube6::new();
        assert_eq!(192, hypercube.edges.len());
        assert_eq!(u32::pow(2, 6) as usize, hypercube.adjacency.len());
    }
    #[test]
    fn hypercube7() {
        let hypercube = Hypercube7::new();
        assert_eq!(448, hypercube.edges.len());
        assert_eq!(u32::pow(2, 7) as usize, hypercube.adjacency.len());
    }
    #[test]
    fn hypercube8() {
        let hypercube = Hypercube8::new();
        assert_eq!(1024, hypercube.edges.len());
        assert_eq!(u32::pow(2, 8) as usize, hypercube.adjacency.len());
    }
    #[test]
    fn hypercube9() {
        let hypercube = Hypercube9::new();
        assert_eq!(2304, hypercube.edges.len());
        assert_eq!(u32::pow(2, 9) as usize, hypercube.adjacency.len());
    }
    #[test]
    fn hypercube10() {
        let hypercube = Hypercube10::new();
        assert_eq!(5120, hypercube.edges.len());
        assert_eq!(u32::pow(2, 10) as usize, hypercube.adjacency.len());
    }
}
