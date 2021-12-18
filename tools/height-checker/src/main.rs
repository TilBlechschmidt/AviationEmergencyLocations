use geo::{prelude::Centroid, Coordinate};
use proj::{Coord, Proj};
use serde::Deserialize;
use std::fs::File;

#[derive(Debug)]
struct PointOfInterest {
    lat: f64,
    lon: f64,
}

impl Coord<f64> for PointOfInterest {
    fn x(&self) -> f64 {
        self.lon
    }
    fn y(&self) -> f64 {
        self.lat
    }
    fn from_xy(x: f64, y: f64) -> Self {
        Self { lon: x, lat: y }
    }
}

#[derive(Deserialize, Debug)]
struct Coordinates {
    start: (f64, f64),
    end: (f64, f64),
}

#[derive(Deserialize, Debug)]
struct Location {
    coordinates: Coordinates,
    name: String,
}

impl Location {
    fn to_poi(&self) -> PointOfInterest {
        let centroid = geo::Line::new(
            Coordinate {
                x: self.coordinates.start.1,
                y: self.coordinates.start.0,
            },
            Coordinate {
                x: self.coordinates.end.1,
                y: self.coordinates.end.0,
            },
        )
        .centroid();

        let lat = centroid.lat();
        let lon = centroid.lng();

        PointOfInterest { lat, lon }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Download the necessary dataset from here and extract it to a folder called `data`:
    // https://geoportal-hamburg.de/geo-online/?Map/layerIds=12883,12884,16101,19969,19173&visibility=true,true,true,false,true&transparency=0,0,0,0,0&Map/center=%5B565744.3542366757,5934987.989125426%5D&Map/zoomLevel=5#LayerInfoDataDownload

    let path = "../../static/data/locations.yml";
    let f = File::open(path)?;
    let locations: Vec<Location> = serde_yaml::from_reader(f)?;

    for location in locations {
        let elevation = find_location_height(&location)?;
        match elevation {
            Some(elevation) => println!("{}\t{}", (elevation * 3.28084).round(), location.name),
            None => println!("NODATA\t{}", location.name),
        }
    }

    Ok(())
}

fn find_location_height(location: &Location) -> Result<Option<f64>, std::io::Error> {
    let donut_shop = location.to_poi();

    let from = "EPSG:4326";
    let to = "EPSG:25832";
    let proj = Proj::new_known_crs(&from, &to, None).unwrap();

    let result = proj.convert(donut_shop).unwrap();

    // Build the folder name
    let mut folder_coord = result.x().to_string();
    folder_coord.truncate(3);
    let folder = format!("s32_{}", folder_coord);

    // Build the file name
    let mut file_coord = result.y().to_string();
    file_coord.truncate(4);
    let file = format!("dgm1_32_{}_{}_1_hh.xyz", folder_coord, file_coord);

    // Read the data
    let path = format!("data/{}/{}", folder, file);
    let data = std::fs::read_to_string(&path)?;

    // Find the correct value
    let expected_y = result.y().round();
    let expected_x = result.x().round();

    for line in data.lines() {
        let mut components = line.split_whitespace();
        let x: f64 = components.next().unwrap().parse().unwrap();
        let y: f64 = components.next().unwrap().parse().unwrap();
        let z: f64 = components.next().unwrap().parse().unwrap();

        if expected_y == y && expected_x == x {
            return Ok(Some(z));
        }
    }

    Ok(None)
}
