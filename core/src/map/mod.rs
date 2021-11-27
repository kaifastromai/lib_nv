pub trait Region {
    type RegionType;
    type CoordinateType;
    fn get_regions(&self) -> &[Self::RegionType];
    fn get_region_at(&self, coord: Self::CoordinateType) -> Option<&Self::RegionType>;
}
pub mod proc_2d {
    ///A procedural 2D map image
    /// Because of the nature of the map, most of its interactivity must come from a UI frontend.
    /// 
    /// The kernel only holds the serialzed data when dynamic editing of the map is finished by the end user
    struct MapProcedural2D {
        height: u32,
        width: u32,
    
    }
    struct Coordinate2D {
        x: u32,
        y: u32,
    }
    struct ProceduralRegion2D {
        name: &'static str,
        boundary: Path2D,
    }
    struct Path2D {
        path: Vec<Coordinate2D>,

    }
}

pub mod image_2d {
    struct MapImage2D {}
}

pub mod proc_3d_sphere {
    struct MapProcedural3DSphere {}
}
pub mod image_3d {
    struct MapImage3D {}
}
pub mod proc_3d_height {
    struct MapProcedural3DHeight {}
}
